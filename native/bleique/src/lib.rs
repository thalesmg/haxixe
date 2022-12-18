use bytes::Buf;
use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;

use rustler::resource;
use rustler::resource::ResourceArc;
use rustler::Binary;
use rustler::Env;
use rustler::OwnedBinary;
use rustler::Term;

use std::io::Write;
use std::sync::Mutex;

const OUT: usize = 2048;

mod atomos {
    rustler::atoms! {
        ok
    }
}

struct HasherInt {
    hasher: blake3::Hasher,
    sum: Bytes,
}
struct HasherResource(Mutex<HasherInt>);

#[rustler::nif]
fn new() -> ResourceArc<HasherResource> {
    ResourceArc::new(HasherResource(Mutex::new(HasherInt {
        hasher: blake3::Hasher::new(),
        sum: Bytes::copy_from_slice(&[0u8; OUT]),
    })))
}

fn hash_int(hasher0: &mut blake3::Hasher, bin: Bytes) -> [u8; OUT] {
    hasher0.reset();
    hasher0.update(&bin);
    let hasher = hasher0.clone();
    let mut buf = [0u8; OUT];
    let mut out_reader = hasher.finalize_xof();
    out_reader.fill(&mut buf);
    buf
}

#[rustler::nif]
fn get<'a>(env: Env<'a>, resource: ResourceArc<HasherResource>) -> Term<'a> {
    let res = &mut *resource.0.try_lock().unwrap();
    let mut out = BytesMut::with_capacity(OUT);
    out.put(&res.sum[..]);
    let mut binary = OwnedBinary::new(out.len()).unwrap();
    binary.as_mut_slice().write_all(&out).unwrap();
    binary.release(env).to_term(env)
}

#[rustler::nif]
fn add<'a>(env: Env<'a>, resource: ResourceArc<HasherResource>, bin: Binary) -> Term<'a> {
    let res = &mut *resource.0.try_lock().unwrap();
    let binbuf = Bytes::copy_from_slice(&bin);
    let hash0 = hash_int(&mut res.hasher, binbuf);
    let hashed = Bytes::copy_from_slice(&hash0);
    res.sum = add16(hashed, &res.sum);
    atomos::ok().to_term(env)
}

#[rustler::nif]
fn sub<'a>(env: Env<'a>, resource: ResourceArc<HasherResource>, bin: Binary) -> Term<'a> {
    let res = &mut *resource.0.try_lock().unwrap();
    let binbuf = Bytes::copy_from_slice(&bin);
    let hash0 = hash_int(&mut res.hasher, binbuf);
    let hashed = Bytes::copy_from_slice(&hash0);
    res.sum = sub16(hashed, &res.sum);
    atomos::ok().to_term(env)
}

fn add16(h: Bytes, s: &Bytes) -> Bytes {
    let mut out = BytesMut::with_capacity(OUT);
    let xit = s.chunks(2);
    let yit = h.chunks(2);
    for (x, y) in xit.zip(yit) {
        let x = Bytes::copy_from_slice(x).get_u16_le();
        let y = Bytes::copy_from_slice(y).get_u16_le();
        let s = x.wrapping_add(y);
        out.put_u16_le(s);
    }
    let out = out.freeze();
    out
}

fn sub16(h: Bytes, s: &Bytes) -> Bytes {
    let mut out = BytesMut::with_capacity(OUT);
    let xit = s.chunks(2);
    let yit = h.chunks(2);
    for (x, y) in xit.zip(yit) {
        let x = Bytes::copy_from_slice(x).get_u16_le();
        let y = Bytes::copy_from_slice(y).get_u16_le();
        let s = x.wrapping_sub(y);
        out.put_u16_le(s);
    }
    let out = out.freeze();
    out
}

rustler::init!(
    "Elixir.Bleique",
    [new, add, sub, get],
    load = |env: Env, _: Term| {
        resource!(HasherResource, env);
        true
    }
);
