# Haxixe

Implementation of
LtHash<sup>[[1]](https://cseweb.ucsd.edu/~daniele/papers/IncHash.pdf)
[[2]](https://eprint.iacr.org/2019/227.pdf)</sup> in Elixir + Rust,
based on [this Go project](https://github.com/lukechampine/lthash).

```elixir
res = Haxixe.new()

Haxixe.add(res, {1, "apple"})
Haxixe.add(res, {2, "orange"})
Haxixe.add(res, {3, "banana"})

hash0 = Haxixe.get(res)

Haxixe.add(res, {2, "peach"})
Haxixe.sub(res, {2, "orange"})

hash1 = Haxixe.get(res)

true = hash0 != hash1

new_res = Haxixe.new()

Haxixe.add(new_res, {1, "apple"})
Haxixe.add(new_res, {2, "peach"})
Haxixe.add(new_res, {3, "banana"})

hash2 = Haxixe.get(new_res)

true = hash1 == hash2
```
