# https://leetcode.com/problems/shifting-letters-ii/
# 05/01/2025

defmodule Solution do
  @spec shifting_letters(s :: String.t(), shifts :: [[integer]]) :: String.t()
  def shifting_letters(s, shifts) do
    n = byte_size(s)

    diff = :array.new(n + 1, fixed: true, default: 0)

    diff =
      shifts
      |> Enum.reduce(diff, fn [start_pos, end_pos, dir], acc ->
        val = if dir == 1, do: 1, else: -1

        acc
        |> then(fn arr -> :array.set(start_pos, :array.get(start_pos, arr) + val, arr) end)
        |> then(fn arr -> :array.set(end_pos + 1, :array.get(end_pos + 1, arr) - val, arr) end)
      end)

    {shifts, _} =
      0..(n - 1)
      |> Enum.map_reduce(0, fn i, acc ->
        curr = acc + :array.get(i, diff)
        {curr, curr}
      end)

    s
    |> :binary.bin_to_list()
    |> Enum.zip(shifts)
    |> Enum.map(fn {c, shift} ->
      rem(rem(c - ?a + shift, 26) + 26, 26) + ?a
    end)
    |> :binary.list_to_bin()
  end
end

defmodule Main do
  def main do
    test_cases = [
      {
        "abc",
        [[0, 1, 0], [1, 2, 1], [0, 2, 1]],
        "ace"
      },
      {
        "dztz",
        [[0, 0, 0], [1, 1, 1]],
        "catz"
      }
    ]

    Enum.each(test_cases, fn {s, shifts, expected} ->
      result = Solution.shifting_letters(s, shifts)
      IO.puts("Input: s=#{inspect(s)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
