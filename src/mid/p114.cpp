#include <bits/stdc++.h>
using namespace std;

/**
 * Definition for a binary tree node.
 */
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
public:
    void flatten(TreeNode* root) {
        auto* dummy = new TreeNode();
        trans(root, dummy);
    }

    TreeNode* trans(TreeNode* node, TreeNode* prev) {
        prev->left = nullptr;
        prev->right = node;
        
        if (node == nullptr) {
            return prev;
        }

        TreeNode* dummy = new TreeNode();

        auto left = node->left;
        auto right = node->right;
        auto left_last = trans(left, node);
        auto right_last = trans(right, dummy);

        left_last->right = dummy->right;

        return right_last;
    }
};