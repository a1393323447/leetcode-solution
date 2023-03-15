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
    int rob(TreeNode* root) {
        auto res = rob_impl(root);
        return std::max(res.first, res.second);
    }

    //    ns   s
    pair<int, int> rob_impl(TreeNode* root) {
        if (root == nullptr) return {0, 0};

        auto cur = root->val;
        auto left_res = rob_impl(root->left);
        auto right_res = rob_impl(root->right);

        // 不要 root
        auto res_1 = std::max(left_res.first, left_res.second) + 
                     std::max(right_res.first, right_res.second);
        // 要 root
        auto res_2 = cur + left_res.first + right_res.first;
        
        return { res_1, res_2 };
    }
};