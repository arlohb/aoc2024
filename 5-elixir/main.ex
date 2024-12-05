
defmodule Rule do
  defstruct left: 0, right: 0

  def parse(lines) do
    lines
      |> Enum.map(fn line ->
        [ left , right | _ ] = line
          |> String.split("|")
          |> Enum.map(fn s ->
            { i, _ } = Integer.parse(s)
            i
          end)
        %Rule{ left: left, right: right }
      end)
  end

  def is_valid(%{ left: left, right: right }, { a, b }) do
    cond do
      left == a && right == b -> true
      left == b && right == a -> false
      true -> true
    end
  end
end

defmodule Pairs do
  def pairs_first([ x | xs ]) do
    xs |> Enum.map(fn y -> { x, y } end)
  end

  def pairs([]) do
    []
  end

  def pairs([ x | xs ]) do
    pairs_first([x | xs]) ++ pairs(xs)
  end
end

defmodule Update do
  def parse(lines) do
    lines
      |> Enum.filter(fn l -> l != "" end)
      |> Enum.map(&String.split(&1, ","))
      |> Enum.map(fn update ->
        Enum.map(update, fn s ->
          { i, _ } = Integer.parse(s)
          i
        end)
      end)
  end

  def is_update_pair_correct(rules, update_pair) do
    rules |> Enum.all?(&Rule.is_valid(&1, update_pair))
  end

  def is_update_correct(rules, update) do
    update |> Pairs.pairs |> Enum.all?(&is_update_pair_correct(rules, &1))
  end

  def middle_factor(rules, update) do
    if is_update_correct(rules, update) do
      Enum.at(update, update |> length() |> div(2))
    else
      0
    end
  end
end

defmodule Main do
  def main do
    {:ok, input} = File.read("input.txt")
    lines = String.split(input, "\n")

    { rules, updates } = Enum.split_while(lines, &(&1 != ""))
    updates = updates |> Update.parse()
    rules = rules |> Rule.parse()

    sum = updates
      |> Enum.map(&Update.middle_factor(rules, &1))
      |> Enum.sum()

    IO.puts(sum)
  end
end

Main.main

