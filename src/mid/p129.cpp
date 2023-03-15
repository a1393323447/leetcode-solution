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
private:
    int res = 0;
public:
    int sumNumbers(TreeNode* root) {
        if (root == nullptr) return 0;
        dfs(root, 0);
        return res;
    }
    void dfs(TreeNode* node, int acc) {
        acc = acc * 10 + node->val;
        if (node->left == nullptr && node->right == nullptr) {
            this->res += acc;
            return;
        }

        if (node->left != nullptr) dfs(node->left, acc);
        if (node->right != nullptr) dfs(node->right, acc);
    }
};