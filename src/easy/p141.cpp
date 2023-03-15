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
    bool hasCycle(ListNode *head) {
        if (head == nullptr) {
            return false;
        }

        auto* slow = head;
        auto* fast = head->next;
        while (fast != nullptr) {
            fast = fast->next;
            slow = slow->next;
            if (fast == nullptr) {
                break;
            }
            fast = fast->next;
            if (fast == slow) {
                return true;
            }
        }

        return false;
    }
};