# https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
# 19/02/2025

# 193 ms, 72.8 MB
# defmodule Solution do
#   @spec get_happy_string(n :: integer, k :: integer) :: String.t()
#   def get_happy_string(n, k) do
#     generate_happy_strings(n)
#     |> Enum.at(k - 1, "")
#   end

#   defp generate_happy_strings(n) do
#     generate_happy_strings(["a", "b", "c"], n)
#   end

#   defp generate_happy_strings(strings, n) do
#     if strings |> hd() |> String.length() == n do
#       strings
#     else
#       strings
#       |> generate_new_strings([])
#       |> generate_happy_strings(n)
#     end
#   end

#   defp generate_new_strings([], new_strings), do: Enum.reverse(new_strings)

#   defp generate_new_strings([curr_string | rest_old_strings], new_strings) do
#     case String.last(curr_string) do
#       "a" ->
#         s1 = curr_string <> "b"
#         s2 = curr_string <> "c"
#         generate_new_strings(rest_old_strings, [s2, s1 | new_strings])

#       "b" ->
#         s1 = curr_string <> "a"
#         s2 = curr_string <> "c"
#         generate_new_strings(rest_old_strings, [s2, s1 | new_strings])

#       "c" ->
#         s1 = curr_string <> "a"
#         s2 = curr_string <> "b"
#         generate_new_strings(rest_old_strings, [s2, s1 | new_strings])
#     end
#   end
# end

# 1 ms, 72.4 MB
import Bitwise

defmodule Solution do
  @spec get_happy_string(n :: integer, k :: integer) :: String.t()
  def get_happy_string(n, k) do
    total = 3 * (1 <<< (n - 1))

    if k > total or k <= 0 do
      ""
    else
      build_happy_string(n, k - 1, [], nil)
    end
  end

  defp build_happy_string(n, k, chars, prev_char) do
    current_length = length(chars)

    if current_length == n do
      chars
      |> Enum.reverse()
      |> List.to_string()
    else
      remaining = n - current_length - 1
      possible_chars = possible_chars(prev_char)
      {char, new_k} = find_char(possible_chars, remaining, k)

      if char == nil do
        ""
      else
        build_happy_string(n, new_k, [char | chars], char)
      end
    end
  end

  defp possible_chars(nil), do: [?a, ?b, ?c]

  defp possible_chars(prev_char) do
    case prev_char do
      ?a -> [?b, ?c]
      ?b -> [?a, ?c]
      ?c -> [?a, ?b]
    end
  end

  defp find_char([], _remaining, _k), do: {nil, 0}

  defp find_char([char | rest], remaining, k) do
    count = 1 <<< remaining

    if k < count do
      {char, k}
    else
      find_char(rest, remaining, k - count)
    end
  end
end

defmodule Main do
  def main do
    test_cases = [
      {1, 3, "c"},
      {1, 4, ""},
      {3, 9, "cab"}
    ]

    test_cases
    |> Enum.each(fn {n, k, expected} ->
      result = Solution.get_happy_string(n, k)
      IO.puts("Input: #{inspect({n, k})}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
