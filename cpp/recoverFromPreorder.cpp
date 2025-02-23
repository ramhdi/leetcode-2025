// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/
// 22/02/2025

#include <stack>

#include "TreeNode.hpp"

class Solution {
   public:
    TreeNode* recoverFromPreorder(string traversal) {
        stack<TreeNode*> stack;
        int index = 0;

        while (index < traversal.size()) {
            int depth = 0;
            while (index < traversal.size() && traversal[index] == '-') {
                depth++;
                index++;
            }

            while (stack.size() > depth) {
                stack.pop();
            }

            int value = 0;
            while (index < traversal.size() && isdigit(traversal[index])) {
                value = value * 10 + (traversal[index] - '0');
                index++;
            }

            TreeNode* node = new TreeNode(value);
            if (!stack.empty()) {
                if (stack.top()->left == nullptr) {
                    stack.top()->left = node;
                } else {
                    stack.top()->right = node;
                }
            }

            stack.push(node);
        }

        while (stack.size() > 1) {
            stack.pop();
        }

        return stack.top();
    }
};