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

        auto* cur = root;
        while (cur != nullptr) {
            auto* dummy = new Node();
            auto* pre = dummy;
            while (cur != nullptr) {
                if (cur->left != nullptr) {
                    pre->next = cur->left;
                    pre = pre->next;
                }
                if (cur->right != nullptr) {
                    pre->next = cur->right;
                    pre = pre->next;
                }
                cur = cur->next;
            }
            cur = dummy->next;
        }

        return root;
    }
};