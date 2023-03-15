struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    // 和循环移动数组的思路一致
    ListNode* rotateRight(ListNode* head, int k) {
        if (head == nullptr || k == 0) return head;

        auto len = listLen(head);
        k = (len - 1) - k % len;
        // find kth node
        auto node_k = head;
        while (k > 0) {
            node_k = node_k->next;
            k--;
        }

        auto node_k_next = node_k->next;
        node_k->next = nullptr;

        auto new_head = reverseList(head);
        auto next_head = reverseList(node_k_next);
        head->next = next_head;

        return reverseList(new_head);
    }

    int listLen(ListNode* head) {
        int i = 0;
        while (head != nullptr) {
            head = head->next;
            i++;
        }

        return i;
    }

    ListNode* reverseList(ListNode* head) {
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