# https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/
# 13/02/2025

defmodule MinHeap do
  @moduledoc """
  A simple MinHeap implementation using Erlang's :gb_trees.
  """

  @type t :: :gb_trees.tree(any, non_neg_integer())

  @spec new() :: t
  def new(), do: :gb_trees.empty()

  @spec push(t, any) :: t
  def push(pq, value) do
    updated_pq =
      case :gb_trees.lookup(value, pq) do
        :none -> :gb_trees.enter(value, 1, pq)
        {:value, count} -> :gb_trees.enter(value, count + 1, pq)
      end

    updated_pq
  end

  @spec pop(t) :: {any, t} | :empty
  def pop(pq) do
    case :gb_trees.smallest(pq) do
      {min, 1} -> {min, :gb_trees.delete(min, pq)}
      {min, count} -> {min, :gb_trees.enter(min, count - 1, pq)}
    end
  rescue
    _ -> :empty
  end

  @spec from_list([any]) :: t
  def from_list(list) do
    Enum.reduce(list, new(), &push(&2, &1))
  end
end

defmodule Solution do
  @spec min_operations([integer], integer) :: integer
  def min_operations(nums, k) do
    nums
    |> MinHeap.from_list()
    |> min_operations(k, 0)
  end

  defp min_operations(min_heap, k, count) do
    case MinHeap.pop(min_heap) do
      :empty ->
        count

      {x, _min_heap} when x >= k ->
        count

      {x, min_heap} ->
        case MinHeap.pop(min_heap) do
          :empty -> count
          {y, min_heap} -> min_operations(MinHeap.push(min_heap, x * 2 + y), k, count + 1)
        end
    end
  end
end

defmodule Main do
  def main do
    test_cases = [
      {
        [2, 11, 10, 1, 3],
        10,
        2
      },
      {
        [1, 1, 2, 4, 9],
        20,
        4
      }
    ]

    test_cases
    |> Enum.each(fn {nums, k, expected} ->
      result = Solution.min_operations(nums, k)
      IO.puts("Input: #{inspect({nums, k})}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
