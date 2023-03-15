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
    vector<TreeNode*> res{};
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        vector<TreeNode*> cache{};
        searchPath(root, p, cache);
        auto path_to_p = res;

        searchPath(root, q, cache);
        auto path_to_q = res;
        
        int size = min(path_to_p.size(), path_to_q.size());
        for (int i = 0; i < size; i++) {
            if (path_to_p[i] != path_to_q[i]) {
                return path_to_p[i - 1];
            }
        }

        return path_to_p[size - 1];
    }

    void searchPath(TreeNode* root, TreeNode* target, vector<TreeNode*>& cache) {
        if (root == target) {
            cache.push_back(root);
            res = cache;
            cache.pop_back();
            return;
        }
        
        cache.push_back(root);
        if (root->left != nullptr) searchPath(root->left, target, cache);
        if (root->right != nullptr) searchPath(root->right, target, cache);
        cache.pop_back();
    }
};