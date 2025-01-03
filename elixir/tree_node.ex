defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule TreeNodeHelpers do
  @spec from_list(list :: [integer() | nil]) :: TreeNode.t() | nil
  def from_list([]), do: nil

  def from_list(list) do
    build_tree(list, 0)
  end

  defp build_tree(list, i) when i < length(list) do
    case Enum.at(list, i) do
      nil ->
        nil

      val ->
        %TreeNode{
          val: val,
          left: build_tree(list, 2 * i + 1),
          right: build_tree(list, 2 * i + 2)
        }
    end
  end

  defp build_tree(_, _), do: nil

  @spec to_list(root :: TreeNode.t() | nil) :: [integer() | nil]
  def to_list(nil), do: []

  def to_list(root) do
    queue = [root]

    do_to_list(queue, [])
    |> trim_trailing_nils()
  end

  defp do_to_list([], acc), do: acc

  defp do_to_list(queue, acc) do
    {vals, next_queue} =
      Enum.reduce(queue, {[], []}, fn node, {vals_acc, q_acc} ->
        if node == nil do
          {vals_acc ++ [nil], q_acc}
        else
          {vals_acc ++ [node.val], q_acc ++ [node.left, node.right]}
        end
      end)

    if Enum.all?(next_queue, &is_nil/1) do
      acc ++ vals
    else
      do_to_list(next_queue, acc ++ vals)
    end
  end

  defp trim_trailing_nils(list) do
    Enum.reverse(Enum.drop_while(Enum.reverse(list), &is_nil/1))
  end
end
