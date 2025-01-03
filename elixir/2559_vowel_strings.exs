# https://leetcode.com/problems/count-vowel-strings-in-ranges/
# 02/01/2025

defmodule Solution do
  @vowels MapSet.new(["a", "e", "i", "o", "u"])

  @spec vowel_strings(words :: [String.t()], queries :: [[integer]]) :: [integer]
  def vowel_strings(words, queries) do
    prefix_sum =
      words
      |> Enum.reduce({[0], 0}, fn word, {acc, count} ->
        new_count = count + if is_vowel_string?(word), do: 1, else: 0
        {[new_count | acc], new_count}
      end)
      |> Kernel.elem(0)
      |> Enum.reverse()
      |> List.to_tuple()

    queries
    |> Enum.map(fn [start, end_idx] ->
      Kernel.elem(prefix_sum, end_idx + 1) - Kernel.elem(prefix_sum, start)
    end)
  end

  defp is_vowel_string?(word) do
    [first, last] = [String.first(word), String.last(word)]
    MapSet.member?(@vowels, first) and MapSet.member?(@vowels, last)
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{
        words: ["aba", "bcb", "ece", "aa", "e"],
        queries: [[0, 2], [1, 4], [1, 1]],
        expected: [2, 3, 0]
      },
      %{
        words: ["a", "e", "i"],
        queries: [[0, 2], [0, 1], [2, 2]],
        expected: [3, 2, 1]
      }
    ]

    Enum.each(test_cases, fn %{words: words, queries: queries, expected: expected} ->
      result = Solution.vowel_strings(words, queries)
      IO.puts("Input: words = #{inspect(words)}, queries = #{inspect(queries)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
