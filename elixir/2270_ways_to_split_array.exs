# https://leetcode.com/problems/number-of-ways-to-split-array/
# 03/01/2025

defmodule Solution do
  @spec ways_to_split_array(nums :: [integer]) :: integer
  def ways_to_split_array(nums) do
    sum = nums |> Enum.sum()

    nums
    |> Enum.take(length(nums) - 1)
    |> Enum.reduce({0, 0}, fn num, {count, left_sum} ->
      new_left_sum = left_sum + num
      new_count = if new_left_sum >= sum - new_left_sum, do: count + 1, else: count
      {new_count, new_left_sum}
    end)
    |> Kernel.elem(0)
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{
        nums: [10, 4, -8, 7],
        expected: 2
      },
      %{
        nums: [2, 3, 1, 0],
        expected: 2
      }
    ]

    Enum.each(test_cases, fn %{nums: nums, expected: expected} ->
      result = Solution.ways_to_split_array(nums)
      IO.puts("Input: nums=#{inspect(nums)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
