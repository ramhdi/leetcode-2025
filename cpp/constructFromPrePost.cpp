// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
// 23/02/2025

#include <unordered_map>

#include "TreeNode.hpp"

class Solution {
   private:
    TreeNode* buildTree(vector<int>& preorder, int preorderStart,
                        int preorderEnd, vector<int>& postorder,
                        int postorderStart, int postorderEnd,
                        unordered_map<int, int>& postorderMap) {
        if (preorderStart > preorderEnd) return nullptr;

        TreeNode* root = new TreeNode(preorder[preorderStart]);
        if (preorderStart == preorderEnd) return root;

        int leftRootVal = preorder[preorderStart + 1];
        int index = postorderMap[leftRootVal];
        int leftSize = index - postorderStart + 1;

        root->left =
            buildTree(preorder, preorderStart + 1, preorderStart + leftSize,
                      postorder, postorderStart, index, postorderMap);

        root->right =
            buildTree(preorder, preorderStart + leftSize + 1, preorderEnd,
                      postorder, index + 1, postorderEnd - 1, postorderMap);

        return root;
    }

   public:
    TreeNode* constructFromPrePost(vector<int>& preorder,
                                   vector<int>& postorder) {
        unordered_map<int, int> postorderMap;
        for (int i = 0; i < postorder.size(); i++) {
            postorderMap[postorder[i]] = i;
        }

        return buildTree(preorder, 0, preorder.size() - 1, postorder, 0,
                         postorder.size() - 1, postorderMap);
    }
};