# https://leetcode.com/problems/find-unique-binary-string/
# 19/02/2025

defmodule Solution do
  @spec find_different_binary_string(nums :: [String.t()]) :: String.t()
  def find_different_binary_string(nums) do
    nums
    |> Enum.with_index()
    |> find_different_binary_string([])
    |> Enum.reverse()
    |> Enum.join()
  end

  defp find_different_binary_string([], result), do: result

  defp find_different_binary_string([{s, i} | rest], result) do
    case binary_part(s, i, 1) do
      "0" -> find_different_binary_string(rest, ["1" | result])
      "1" -> find_different_binary_string(rest, ["0" | result])
    end
  end
end

# defmodule Solution do
#   @spec find_different_binary_string(nums :: [String.t()]) :: String.t()
#   def find_different_binary_string(nums) do
#     nums
#     |> Enum.with_index()
#     |> Enum.map(fn {s, i} ->
#       case binary_part(s, i, 1) do
#         "0" -> "1"
#         "1" -> "0"
#       end
#     end)
#     |> Enum.join()
#   end
# end

defmodule Main do
  def main do
    test_cases = [
      {["01", "10"], "11"},
      {["00", "01"], "11"},
      {["111", "011", "001"], "101"}
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
