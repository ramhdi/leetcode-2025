# https://leetcode.com/problems/clear-digits/
# 10/02/2025

defmodule Solution do
  @spec clear_digits(String.t()) :: String.t()
  def clear_digits(s), do: clear_digits(s, [])

  defp clear_digits(<<>>, stack), do: stack |> Enum.reverse() |> List.to_string()

  defp clear_digits(<<c::utf8, s_tail::binary>>, [_ | stack_tail]) when c in ?0..?9,
    do: clear_digits(s_tail, stack_tail)

  defp clear_digits(<<c::utf8, s_tail::binary>>, stack), do: clear_digits(s_tail, [c | stack])
end

defmodule Main do
  def main do
    test_cases = [
      {
        "abc",
        "abc"
      },
      {
        "cb34",
        ""
      }
    ]

    test_cases
    |> Enum.each(fn {s, expected} ->
      result = Solution.clear_digits(s)
      IO.puts("Input: #{inspect(s)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
