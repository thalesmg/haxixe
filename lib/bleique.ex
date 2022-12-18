defmodule Bleique do
  use Rustler, otp_app: :haxixe, crate: "bleique"

  def new(), do: :erlang.nif_error(:nif_not_loaded)
  def add(_res, _bin), do: :erlang.nif_error(:nif_not_loaded)
  def sub(_res, _bin), do: :erlang.nif_error(:nif_not_loaded)
  def get(_res), do: :erlang.nif_error(:nif_not_loaded)
end
