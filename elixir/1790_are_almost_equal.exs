# https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
# 05/02/2025

# defmodule Solution do
#   @spec are_almost_equal(s1 :: String.t(), s2 :: String.t()) :: boolean
#   def are_almost_equal(s1, s2) do
#     diff =
#       String.to_charlist(s1)
#       |> Enum.zip(String.to_charlist(s2))
#       |> Enum.filter(fn {c1, c2} -> c1 != c2 end)
#       |> Enum.count()

#     {count1, count2} = {count_chars(s1), count_chars(s2)}

#     (diff == 0 || diff == 2) && count1 == count2
#   end

#   defp count_chars(s) do
#     s
#     |> String.to_charlist()
#     |> Enum.reduce(List.to_tuple(List.duplicate(0, 26)), fn ch, acc ->
#       idx = ch - ?a
#       put_elem(acc, idx, elem(acc, idx) + 1)
#     end)
#   end
# end

# defmodule Solution do
#   @spec are_almost_equal(s1 :: String.t(), s2 :: String.t()) :: boolean
#   def are_almost_equal(s1, s2) when s1 == s2, do: true

#   def are_almost_equal(s1, s2) do
#     case mismatched_pairs(s1, s2) do
#       [{c1, c2}, {c3, c4}] -> c1 == c4 and c2 == c3
#       [] -> true
#       _ -> false
#     end
#   end

#   defp mismatched_pairs(s1, s2) do
#     String.to_charlist(s1)
#     |> Enum.zip(String.to_charlist(s2))
#     |> Enum.filter(fn {c1, c2} -> c1 != c2 end)
#   end
# end

defmodule Solution do
  @spec are_almost_equal(s1 :: String.t(), s2 :: String.t()) :: boolean
  def are_almost_equal(s1, s2) when s1 == s2, do: true

  def are_almost_equal(s1, s2) do
    case mismatched_pairs(s1, s2, []) do
      [{c1, c2}, {c3, c4}] -> c1 == c4 and c2 == c3
      [] -> true
      _ -> false
    end
  end

  defp mismatched_pairs("", "", acc), do: acc

  defp mismatched_pairs(<<c1::utf8, s1::binary>>, <<c2::utf8, s2::binary>>, acc) when c1 != c2,
    do: mismatched_pairs(s1, s2, [{c1, c2} | acc])

  defp mismatched_pairs(<<_c1::utf8, s1::binary>>, <<_c2::utf8, s2::binary>>, acc),
    do: mismatched_pairs(s1, s2, acc)
end

defmodule Main do
  def main do
    test_cases = [
      {
        "bank",
        "kanb",
        true
      },
      {
        "attack",
        "defend",
        false
      },
      {
        "kelb",
        "kelb",
        true
      }
    ]

    test_cases
    |> Enum.each(fn {s1, s2, expected} ->
      result = Solution.are_almost_equal(s1, s2)
      IO.puts("Input: #{inspect({s1, s2})}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
