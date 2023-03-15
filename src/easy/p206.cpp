#include <bits/stdc++.h>
using namespace std;

/**
 * Definition for singly-linked list.
 */
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    // 使用栈
    ListNode* reverseList(ListNode* head) {
        if (head == nullptr) return nullptr;

        stack<ListNode*> s{};
        
        auto* node = head;
        while (node != nullptr) {
            s.push(node);
            node = node->next;
        }

        node = s.top();
        s.pop();
        auto* res = node;
        while (!s.empty()) {
            node->next = s.top();
            s.pop();
            node = node->next;
        }

        node->next = nullptr;
        return res;
    }
    // 双指针
    ListNode* reverseList2(ListNode* head) {
        ListNode *pre = nullptr;
        ListNode *cur = head;
        while(cur != nullptr) {
            ListNode *next = cur->next;
            cur->next = pre;
            pre = cur;
            cur = next;
        }
        return pre;
    }
};