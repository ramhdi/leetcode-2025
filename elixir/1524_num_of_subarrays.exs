# https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/
# 25/02/2025

defmodule Solution do
  @mod 1_000_000_000 + 7

  @spec num_of_subarrays(arr :: [integer]) :: integer
  def num_of_subarrays(arr) do
    arr
    |> generate_prefix_sum([])
    |> count_odd_sums(0, 0, 0)
  end

  defp generate_prefix_sum([], acc), do: Enum.reverse(acc)
  defp generate_prefix_sum([num | rest], []), do: generate_prefix_sum(rest, [num])

  defp generate_prefix_sum([num | rest], acc),
    do: generate_prefix_sum(rest, [num + hd(acc) | acc])

  defp count_odd_sums([], result, _even, _odd), do: result

  defp count_odd_sums([curr_ps | rest], result, even, odd) do
    case(rem(curr_ps, 2)) do
      0 -> count_odd_sums(rest, rem(result + odd, @mod), even + 1, odd)
      1 -> count_odd_sums(rest, rem(result + even + 1, @mod), even, odd + 1)
    end
  end
end

# defmodule Solution do
#   @mod 1_000_000_000 + 7

#   @spec num_of_subarrays(arr :: [integer]) :: integer
#   def num_of_subarrays(arr) do
#     arr
#     |> Enum.scan(0, &(&1 + &2))
#     |> Enum.reduce({0, 0, 0}, fn curr_ps, {result, even, odd} ->
#       case(rem(curr_ps, 2)) do
#         0 -> {rem(result + odd, @mod), even + 1, odd}
#         1 -> {rem(result + even + 1, @mod), even, odd + 1}
#       end
#     end)
#     |> elem(0)
#   end
# end

defmodule Main do
  def main do
    test_cases = [
      {[1, 3, 5], 4},
      {[2, 4, 6], 0},
      {[1, 2, 3, 4, 5, 6, 7], 16}
    ]

    test_cases
    |> Enum.each(fn {nums, expected} ->
      result = Solution.num_of_subarrays(nums)
      IO.puts("Input: #{inspect(nums)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
