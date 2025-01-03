# https://leetcode.com/problems/maximum-score-after-splitting-a-string/
# 01/01/2025

defmodule Solution do
  @spec max_score(s :: String.t()) :: integer
  def max_score(s) do
    total_ones = s |> String.graphemes() |> Enum.count(&(&1 == "1"))

    {max_score, _} =
      s
      |> String.graphemes()
      |> Enum.with_index(1)
      |> Enum.take(String.length(s) - 1)
      |> Enum.reduce({0, 0}, fn {char, idx}, {max, zeros} ->
        current_zeros = zeros + if(char == "0", do: 1, else: 0)
        left_ones = idx - current_zeros
        right_ones = total_ones - left_ones
        score = current_zeros + right_ones
        {max(max, score), current_zeros}
      end)

    max_score
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{
        input: "011101",
        expected: 5
      },
      %{
        input: "00111",
        expected: 5
      },
      %{
        input: "1111",
        expected: 3
      }
    ]

    Enum.each(test_cases, fn %{input: input, expected: expected} ->
      result = Solution.max_score(input)
      IO.puts("Input: #{inspect(input)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
