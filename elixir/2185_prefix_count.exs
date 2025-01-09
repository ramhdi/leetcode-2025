# https://leetcode.com/problems/counting-words-with-a-given-prefix/
# 09/01/2025

defmodule Solution do
  @spec prefix_count(words :: [String.t()], pref :: String.t()) :: integer
  def prefix_count(words, pref) do
    words
    |> Enum.filter(&String.starts_with?(&1, pref))
    |> Enum.count()
  end
end

defmodule Main do
  def main do
    test_cases = [
      {
        ["pay", "attention", "practice", "attend"],
        "at",
        2
      },
      {
        ["leetcode", "win", "loops", "success"],
        "code",
        0
      }
    ]

    test_cases
    |> Enum.each(fn {words, pref, expected} ->
      result = Solution.prefix_count(words, pref)
      IO.puts("Input: #{inspect({words, pref})}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
