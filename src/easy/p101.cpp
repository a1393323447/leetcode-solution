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
    bool isSymmetric(TreeNode* root) {
        if (root == nullptr) return true;
        return check(root->left, root->right);
    }

    bool check(TreeNode* left, TreeNode* right) {
        if (left != nullptr && right != nullptr) {
            return left->val == right->val &&
                check(left->left, right->right) &&
                check(left->right, right->left);
        } else {
            return left == nullptr && right == nullptr;
        }
    }
};