# https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/
# 07/02/2025

defmodule Solution do
  @spec query_results(limit :: integer, queries :: [[integer]]) :: [integer]
  def query_results(_limit, queries), do: helper(queries, %{}, %{}, [])

  defp helper([], _color_map, _ball_map, acc), do: Enum.reverse(acc)

  defp helper([[ball, color] | queries_rest], color_map, ball_map, acc) do
    {color_map, ball_map} =
      if Map.has_key?(ball_map, ball) do
        prev_color = Map.get(ball_map, ball)

        color_map =
          if Map.get(color_map, prev_color) == 1 do
            Map.delete(color_map, prev_color)
          else
            Map.update(color_map, prev_color, 0, fn val -> val - 1 end)
          end

        {color_map, ball_map}
      else
        {color_map, ball_map}
      end

    ball_map = Map.put(ball_map, ball, color)
    color_map = Map.update(color_map, color, 1, fn val -> val + 1 end)

    helper(queries_rest, color_map, ball_map, [map_size(color_map) | acc])
  end
end

defmodule Main do
  def main do
    test_cases = [
      {
        4,
        [[1, 4], [2, 5], [1, 3], [3, 4]],
        [1, 2, 2, 3]
      },
      {
        4,
        [[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]],
        [1, 2, 2, 3, 4]
      }
    ]

    test_cases
    |> Enum.each(fn {limit, queries, expected} ->
      result = Solution.query_results(limit, queries)
      IO.puts("Input: #{inspect({limit, queries})}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Result: #{if result == expected, do: "PASSED", else: "FAILED"}")
      IO.puts("------")
    end)
  end
end

Main.main()
