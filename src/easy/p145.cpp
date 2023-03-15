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
    vector<int> postorderTraversal(TreeNode* root) {
        vector<int> res{};
        postorderTraversal(root, res);

        return res;
    }

    void postorderTraversal(TreeNode* root, vector<int>& res) {
        if (root == nullptr) {
            return;
        }

        postorderTraversal(root->left, res);
        postorderTraversal(root->right, res);
        res.push_back(root->val);

        return;
    }
};