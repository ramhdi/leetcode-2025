# https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
# 12/02/2025

# Map wins!
defmodule Solution do
  @spec maximum_sum(nums :: [integer]) :: integer
  def maximum_sum(nums), do: maximum_sum(nums, %{}, -1)

  defp maximum_sum([], _sum_map, max_sum), do: max_sum

  defp maximum_sum([num | nums_rest], sum_map, max_sum) do
    sum_digit = sum_digits(num)
    n = Map.get(sum_map, sum_digit, 0)

    new_max_sum = if n > 0, do: max(max_sum, num + n), else: max_sum
    new_sum_map = Map.put(sum_map, sum_digit, max(n, num))

    maximum_sum(nums_rest, new_sum_map, new_max_sum)
  end

  defp sum_digits(num) do
    Integer.digits(num)
    |> Enum.sum()
  end
end

# Erlang array
# defmodule Solution do
#   @spec maximum_sum(nums :: [integer]) :: integer
#   def maximum_sum(nums), do: maximum_sum(nums, :array.new(82, default: 0), -1)

#   defp maximum_sum([], _sum_arr, max_sum), do: max_sum

#   defp maximum_sum([num | nums_rest], sum_arr, max_sum) do
#     sum_digit = sum_digits(num)
#     n = :array.get(sum_digit, sum_arr)

#     new_max_sum = if n > 0, do: max(max_sum, num + n), else: max_sum
#     new_sum_arr = :array.set(sum_digit, max(n, num), sum_arr)

#     maximum_sum(nums_rest, new_sum_arr, new_max_sum)
#   end

#   defp sum_digits(num) do
#     Integer.digits(num)
#     |> Enum.sum()
#   end
# end

# Tuple
# defmodule Solution do
#   @spec maximum_sum(nums :: [integer]) :: integer
#   def maximum_sum(nums), do: maximum_sum(nums, Tuple.duplicate(0, 81), -1)

#   defp maximum_sum([], _sum_tup, max_sum), do: max_sum

#   defp maximum_sum([num | nums_rest], sum_tup, max_sum) do
#     sum_digit = sum_digits(num)
#     n = elem(sum_tup, sum_digit - 1)

#     new_max_sum = if n > 0, do: max(max_sum, num + n), else: max_sum
#     new_sum_tup = put_elem(sum_tup, sum_digit - 1, max(n, num))

#     maximum_sum(nums_rest, new_sum_tup, new_max_sum)
#   end

#   defp sum_digits(num) do
#     num
#     |> Integer.digits()
#     |> Enum.sum()
#   end
# end

defmodule Main do
  def main do
    test_cases = [
      {
        [18, 43, 36, 13, 7],
        54
      },
      {
        [10, 12, 19, 14],
        -1
      }
    ]

    test_cases
    |> Enum.each(fn {nums, expected} ->
      result = Solution.maximum_sum(nums)
      IO.puts("Input: #{inspect(nums)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
