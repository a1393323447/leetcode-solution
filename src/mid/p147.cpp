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
    ListNode* insertionSortList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) return head;

        auto dummy = new ListNode();
        auto ordered = dummy;
        auto rest = head;

        while (rest != nullptr) {
            auto node = rest;
            auto next = rest->next;
            insertToList(ordered, node);
            rest = next;
        }

        return ordered->next;
    }

    // head 是已排序的链表头 node 是要插入的链表节点
    void insertToList(ListNode* head, ListNode* node) {
        while (head != nullptr && head->next != nullptr) {
            if (head->next->val >= node->val) {
                break;
            }
            head = head->next;
        }
        auto next = head->next;
        head->next = node;
        node->next = next;
    }
};