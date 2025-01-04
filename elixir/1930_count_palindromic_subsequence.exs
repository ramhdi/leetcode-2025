# https://leetcode.com/problems/unique-length-3-palindromic-subsequences/
# 04/01/2025

defmodule Solution do
  @spec count_palindromic_subsequence(s :: String.t()) :: integer
  def count_palindromic_subsequence(s) do
    chars = String.to_charlist(s)

    char_positions =
      Enum.with_index(chars)
      |> Enum.reduce(%{}, fn {char, idx}, acc ->
        Map.update(acc, char, {idx, idx}, fn {first, _last} -> {first, idx} end)
      end)

    char_positions
    |> Enum.filter(fn {_char, {first, last}} -> first < last end)
    |> Enum.map(fn {_char, {first, last}} ->
      chars
      |> Enum.slice((first + 1)..(last - 1))
      |> MapSet.new()
      |> MapSet.size()
    end)
    |> Enum.sum()
  end
end

defmodule Main do
  def main do
    test_cases = [
      {
        "aabca",
        3
      },
      {
        "adc",
        0
      },
      {
        "bbcbaba",
        4
      }
    ]

    Enum.each(test_cases, fn {s, expected} ->
      result = Solution.count_palindromic_subsequence(s)
      IO.puts("Input: s=#{inspect(s)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
