#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

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
    TreeNode* sortedListToBST(ListNode* head) {
        if (head == nullptr) return nullptr;
        
        vector<int> vals{};
        auto* cur = head;
        while (cur != nullptr) {
            vals.push_back(cur->val);
            cur = cur->next;
        }

        return sortedListToBST(vals, 0, vals.size() - 1);
    }

    // [start, end]
    TreeNode* sortedListToBST(const vector<int>& vals, int start, int end) {
        if (start > end) return nullptr;
        auto mid = (start + end) / 2;
        auto* root = new TreeNode(vals[mid]);
        root->left = sortedListToBST(vals, start, mid - 1);
        root->right = sortedListToBST(vals, mid + 1, end);
        return root;
    }
};