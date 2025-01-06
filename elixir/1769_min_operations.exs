# https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/
# 06/01/2025

# defmodule Solution do
#   @spec min_operations(boxes :: String.t()) :: [integer]
#   def min_operations(boxes) do
#     chars = String.to_charlist(boxes)
#     n = length(chars)
#     indexed = Enum.with_index(chars)

#     Enum.map(0..(n - 1), fn pos ->
#       indexed
#       |> Enum.reduce(0, fn {char, i}, acc ->
#         case char do
#           ?1 -> acc + abs(i - pos)
#           _ -> acc
#         end
#       end)
#     end)
#   end
# end

defmodule Solution do
  @spec min_operations(boxes :: String.t()) :: [integer]
  def min_operations(boxes) do
    chars = String.to_charlist(boxes)
    n = length(chars)
    initial_map = Map.new(0..(n - 1), &{&1, 0})

    {left_result, _} =
      chars
      |> left_to_right(initial_map, 0, 0, 0)

    {right_result, _} =
      chars
      |> Enum.reverse()
      |> right_to_left(left_result, 0, 0, n - 1)

    0..(n - 1)
    |> Enum.map(&Map.get(right_result, &1))
  end

  defp left_to_right([], result, _index, _balls, _moves), do: {result, 0}

  defp left_to_right([char | rest], result, index, balls, moves) do
    result = Map.update!(result, index, &(&1 + moves))
    new_balls = if char == ?1, do: balls + 1, else: balls
    new_moves = moves + new_balls

    left_to_right(rest, result, index + 1, new_balls, new_moves)
  end

  defp right_to_left([], result, _balls, _moves, _index), do: {result, 0}

  defp right_to_left([char | rest], result, balls, moves, index) do
    result = Map.update!(result, index, &(&1 + moves))
    new_balls = if char == ?1, do: balls + 1, else: balls
    new_moves = moves + new_balls

    right_to_left(rest, result, new_balls, new_moves, index - 1)
  end
end

defmodule Main do
  def main do
    test_cases = [
      {
        "110",
        [1, 1, 3]
      },
      {
        "001011",
        [11, 8, 5, 4, 3, 4]
      }
    ]

    Enum.each(test_cases, fn {boxes, expected} ->
      result = Solution.min_operations(boxes)
      IO.puts("Input: boxes=#{inspect(boxes)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
