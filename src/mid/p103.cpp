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
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
        deque<int> cache{};
        vector<vector<int>> res{};

        if (root == nullptr) {
            return res;
        }

        queue<TreeNode*> q{};
        q.push(root);

        bool ltr = true;
        while (!q.empty()) {
            int size = q.size();
            for (int i = 0; i < size; i++) {
                auto* node = q.front();
                q.pop();
                if (ltr) {
                    cache.push_back(node->val);
                } else {
                    cache.push_front(node->val);
                }
                if (node->left != nullptr) q.push(node->left);
                if (node->right != nullptr) q.push(node->right);
            }
            res.push_back(vector<int>{cache.begin(), cache.end()});
            cache.clear();
            ltr = !ltr;
        }

        return res;
    }
};