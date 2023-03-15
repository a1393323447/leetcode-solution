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
    vector<vector<int>> levelOrderBottom(TreeNode* root) {
        deque<vector<int>> res{};
        if (root == nullptr) {
            return {};
        }

        queue<TreeNode*> q{};
        q.push(root);

        while (!q.empty()) {
            int cur_size = q.size();
            res.emplace_front(vector<int>{});
            for (int i = 0; i < cur_size; i++) {
                auto node = q.front();
                q.pop();
                res.front().push_back(node->val);
                if (node->left != nullptr) q.push(node->left);
                if (node->right != nullptr) q.push(node->right);
            }
        }

        return {res.begin(), res.end()};
    }
};