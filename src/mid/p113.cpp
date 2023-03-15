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
    vector<int> cache{};
    vector<vector<int>> res{};
public:
    vector<vector<int>> pathSum(TreeNode* root, int targetSum) {
        if (root == nullptr) {
            return std::move(res);
        }
        dfs(root, targetSum);
        return std::move(res);
    }

    void dfs(TreeNode* node, int target) {
        if (node->left == nullptr && 
            node->right == nullptr && 
            target == node->val) {
            cache.push_back(node->val);
            res.push_back(cache);
            cache.pop_back();
            return;
        }
        
        auto val = node->val;
        cache.push_back(val);
        
        if (node->left != nullptr) dfs(node->left, target - val);
        if (node->right != nullptr) dfs(node->right, target - val);

        cache.pop_back();
    }
};