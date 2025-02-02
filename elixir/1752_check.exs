# https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/
# 02/02/2025

defmodule Solution do
  @spec check(nums :: [integer]) :: boolean
  def check(nums) when length(nums) <= 1, do: true

  def check(nums) do
    n = length(nums)
    array = :array.from_list(nums)

    0..(n - 1)
    |> Enum.filter(fn i ->
      :array.get(i, array) > :array.get(rem(i + 1, n), array)
    end)
    |> Enum.count() <= 1
  end
end

defmodule Main do
  def main do
    test_cases = [
      {[3, 4, 5, 1, 2], true},
      {[2, 1, 3, 4], false},
      {[1, 2, 3], true}
    ]

    Enum.each(test_cases, fn {nums, expected} ->
      result = Solution.check(nums)
      IO.puts("Input: #{inspect(nums)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
