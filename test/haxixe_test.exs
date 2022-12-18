defmodule HaxixeTest do
  use ExUnit.Case
  use ExUnitProperties

  test "example1" do
    res0 = Haxixe.new()

    Haxixe.add(res0, {1, "apple"})
    Haxixe.add(res0, {2, "orange"})
    Haxixe.add(res0, {3, "banana"})

    hash0 = Haxixe.get(res0)

    Haxixe.add(res0, {2, "peach"})
    Haxixe.sub(res0, {2, "orange"})

    hash1 = Haxixe.get(res0)

    assert hash0 != hash1

    res1 = Haxixe.new()

    Haxixe.add(res1, {1, "apple"})
    Haxixe.add(res1, {2, "peach"})
    Haxixe.add(res1, {3, "banana"})

    hash2 = Haxixe.get(res1)

    assert hash1 == hash2
  end

  property "sub . add = id" do
    check all(
            x <- term(),
            y <- term(),
            max_runs: 300
          ) do
      res = Haxixe.new()
      Haxixe.add(res, y)
      hash0 = Haxixe.get(res)

      Haxixe.add(res, x)
      Haxixe.sub(res, x)

      assert hash0 == Haxixe.get(res)
    end
  end

  property "add . sub = id" do
    check all(
            x <- term(),
            y <- term(),
            max_runs: 300
          ) do
      res = Haxixe.new()
      Haxixe.add(res, y)
      hash0 = Haxixe.get(res)

      Haxixe.sub(res, x)
      Haxixe.add(res, x)

      assert hash0 == Haxixe.get(res)
    end
  end
end
