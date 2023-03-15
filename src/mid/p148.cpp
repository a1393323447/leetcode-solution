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
    using NodePtr = ListNode*;
    ListNode* sortList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }
        
        auto first_part = head;
        auto second_part = partaiton(head);
        
        auto first_sorted = sortList(first_part);
        auto second_sorted = sortList(second_part);

        return merge(first_sorted, second_sorted);
    }

    NodePtr partaiton(NodePtr head) {
        auto f = head;
        auto s = head;
        NodePtr pre = nullptr;

        while (f != nullptr && f->next != nullptr) {
            pre = s;
            s = s->next;
            f = f->next->next;
        }

        pre->next = nullptr;

        return s;
    }

    NodePtr merge(NodePtr list1, NodePtr list2) {
        NodePtr dummy = new ListNode();
        auto cur = dummy;
        for (;;) {
            __builtin_prefetch(list1);
            __builtin_prefetch(list2);
            if (list1 != nullptr && list2 != nullptr) {
                if (list1->val < list2->val) {
                    cur->next = list1;
                    list1 = list1->next;
                } else {
                    cur->next = list2;
                    list2 = list2->next;
                }
            } else if (list1 == nullptr) [[unlikely]] {
                cur->next = list2;
                break;
            } else if (list2 == nullptr) [[unlikely]] {
                cur->next = list1;
                break;
            } else [[unlikely]] {
                break;
            }
            cur = cur->next;
        }
        return dummy->next;
    }
};