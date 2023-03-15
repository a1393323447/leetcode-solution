#include <bits/stdc++.h>
using namespace std;

/**
 * Definition for singly-linked list.
 */
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* deleteDuplicates(ListNode* head) {
        auto *dummy = new ListNode();
        dummy->next = head;

        auto *cur = dummy;
        while (
            cur->next != nullptr && 
            cur->next->next != nullptr) {
            if (cur->next->val == cur->next->next->val) {
                auto val = cur->next->val;
                auto *p = cur->next->next->next;
                while (p != nullptr && p->val == val) {
                    p = p->next;
                }
                cur->next = p;
            } else {
                // 注意这个细节, 只有当接下来的两个节点的值不同时, 才往前走
                cur = cur->next;
            }
        }

        return dummy->next;
    }
};