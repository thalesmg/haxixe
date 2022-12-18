defmodule Haxixe do
  def new(), do: Bleique.new()

  def add(res, x) do
    bin = :erlang.term_to_binary(x)
    Bleique.add(res, bin)
  end

  def sub(res, x) do
    bin = :erlang.term_to_binary(x)
    Bleique.sub(res, bin)
  end

  def get(res) do
    Bleique.get(res)
  end
end
