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
    bool isBalanced(TreeNode* root) {
        if(root == nullptr) return true;
        return check(root) != -1;
    }
    int check(TreeNode* node) {
        if(node == nullptr) return 0;
        int left = check(node->left);
        int right = check(node->right);
        if(left == -1 || right == -1) return -1;
        return abs(left - right) > 1 ? -1 : max(left, right) + 1;
    }
};