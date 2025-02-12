# https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
# 11/02/2025

defmodule Solution do
  @spec remove_occurrences(s :: String.t(), part :: String.t()) :: String.t()
  def remove_occurrences(s, part) do
    case String.contains?(s, part) do
      true ->
        s
        |> String.replace(part, "", global: false)
        |> remove_occurrences(part)

      false ->
        s
    end
  end
end

defmodule Main do
  def main do
    test_cases = [
      {
        "daabcbaabcbc",
        "abc",
        "dab"
      },
      {
        "axxxxyyyyb",
        "xy",
        "ab"
      }
    ]

    test_cases
    |> Enum.each(fn {s, part, expected} ->
      result = Solution.remove_occurrences(s, part)
      IO.puts("Input: #{inspect({s, part})}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
