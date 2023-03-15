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
    void reorderList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) return;

        auto mid = splitInMid(head);
        auto mid_rev = reverseList(mid);
        auto cur = new ListNode();
        while (head != nullptr && mid_rev != nullptr) {
            cur->next = head;
            cur = cur->next;
            head = head->next;

            cur->next = mid_rev;
            cur = cur->next;
            mid_rev = mid_rev->next;
        }
    }

    ListNode* splitInMid(ListNode* head) {
        auto f = head;
        auto s = head;
        ListNode* pre = nullptr;
        while (f != nullptr && f->next != nullptr) {
            pre = s;
            s = s->next;
            f = f->next->next;
        }

        if (pre != nullptr) {
            pre->next = nullptr;
        }
        
        return s;
    }

    ListNode* reverseList(ListNode* head) {
        auto cur = head;
        ListNode* pre = nullptr;

        while (cur != nullptr) {
            auto next = cur->next;
            cur->next = pre;
            pre = cur;
            cur = next;
        }

        return pre;
    }
};