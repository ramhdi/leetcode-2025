# https://leetcode.com/problems/maximum-ascending-subarray-sum/
# 04/02/2025

# defmodule Solution do
#   @spec max_ascending_sum(nums :: [integer]) :: integer
#   def max_ascending_sum(nums) do
#     first_element = hd(nums)

#     nums
#     |> Enum.chunk_every(2, 1, :discard)
#     |> Enum.map(&List.to_tuple/1)
#     |> Enum.reduce({first_element, first_element}, fn {w0, w1}, {max_sum, curr_sum} ->
#       if w0 < w1 do
#         {max(max_sum, curr_sum + w1), curr_sum + w1}
#       else
#         {max_sum, w1}
#       end
#     end)
#     |> elem(0)
#   end
# end

defmodule Solution do
  @spec max_ascending_sum(nums :: [integer]) :: integer
  def max_ascending_sum([first | rest]) do
    do_max_ascending_sum(rest, first, first, first)
  end

  defp do_max_ascending_sum([], max_sum, _curr_sum, _prev), do: max_sum

  defp do_max_ascending_sum([num | rest], max_sum, curr_sum, prev) when num > prev do
    do_max_ascending_sum(rest, max(max_sum, curr_sum + num), curr_sum + num, num)
  end

  defp do_max_ascending_sum([num | rest], max_sum, _curr_sum, _prev) do
    do_max_ascending_sum(rest, max_sum, num, num)
  end
end

defmodule Main do
  def main do
    test_cases = [
      {
        [10, 20, 30, 5, 10, 50],
        65
      },
      {
        [10, 20, 30, 40, 50],
        150
      },
      {
        [12, 17, 15, 13, 10, 11, 12],
        33
      }
    ]

    test_cases
    |> Enum.each(fn {nums, expected} ->
      result = Solution.max_ascending_sum(nums)
      IO.puts("Input: #{inspect(nums)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
