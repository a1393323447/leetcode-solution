#include <bits/stdc++.h>
using namespace std;

// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(nullptr), right(nullptr), next(nullptr) {}

    Node(int _val) : val(_val), left(nullptr), right(nullptr), next(nullptr) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {}
};

class Solution {
public:
    Node* connect(Node* root) {
        if (root == nullptr) {
            return nullptr;
        }

        queue<Node*> q{};
        q.push(root);

        while (!q.empty()) {
            int cur_size = q.size();
            Node* pre = nullptr;
            for (int i = 0; i < cur_size; i++) {
                auto cur = q.front();
                q.pop();
                
                if (pre != nullptr) pre->next = cur;
                if (cur->left != nullptr) q.push(cur->left);
                if (cur->right != nullptr) q.push(cur->right);

                pre = cur;
            }
        }

        return root;
    }
};