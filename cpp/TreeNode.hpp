#pragma once
#include <iostream>
#include <queue>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr){};
    TreeNode(int x) : val(x), left(nullptr), right(nullptr){};
    TreeNode(int x, TreeNode *left, TreeNode *right)
        : val(x), left(left), right(right){};
};

TreeNode *createTree(const std::vector<int> &nodes) {
    if (nodes.empty()) return nullptr;

    int nullValue = -999999;  // Define magic number for null nodes
    TreeNode *root =
        new TreeNode(nodes[0]);  // The root of the tree is the first element
    queue<TreeNode *> queue;     // Use a queue to manage tree node insertions
    queue.push(root);

    int i = 1;  // Start from the second element in the vector
    while (!queue.empty() && i < nodes.size()) {
        TreeNode *current = queue.front();
        queue.pop();

        // Process the left child
        if (i < nodes.size() && nodes[i] != nullValue) {
            current->left = new TreeNode(nodes[i]);
            queue.push(current->left);
        }
        i++;

        // Process the right child
        if (i < nodes.size() && nodes[i] != nullValue) {
            current->right = new TreeNode(nodes[i]);
            queue.push(current->right);
        }
        i++;
    }

    return root;
}

void printTree(TreeNode *root) {
    queue<TreeNode *> q;
    q.push(root);

    while (!q.empty()) {
        TreeNode *current = q.front();
        q.pop();

        if (current == nullptr) {
            cout << "null, ";
        } else {
            cout << current->val << ", ";
        }

        if (current->left) {
            q.push(current->left);
        }

        if (current->right) {
            q.push(current->right);
        }
    }

    cout << endl;
}