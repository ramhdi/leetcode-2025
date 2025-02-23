// https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/
// 21/02/2025

#include <unordered_set>

#include "TreeNode.hpp"

class FindElements {
    unordered_set<int> nums;

    void recoverTree(TreeNode* root, unordered_set<int>& nums) {
        if (root == nullptr) return;

        int x = root->val;
        nums.insert(x);

        if (root->left != nullptr) {
            root->left->val = 2 * x + 1;
        }

        if (root->right != nullptr) {
            root->right->val = 2 * x + 2;
        }

        recoverTree(root->left, nums);
        recoverTree(root->right, nums);
    }

   public:
    FindElements(TreeNode* root) {
        if (root != nullptr) {
            root->val = 0;
            recoverTree(root, nums);
        }
    }

    bool find(int target) { return nums.count(target) > 0; }
};
