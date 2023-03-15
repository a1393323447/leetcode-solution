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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        auto* const res = l1; // 结果存在 l1 中
        auto* cur = res;
        ListNode* pre = nullptr;
        int carry = 0;
        while (l1 != nullptr && l2 != nullptr) {
            cur->val = l1->val + l2->val + carry;
            // 设置进位
            carry = cur->val / 10;
            cur->val %= 10;

            l1 = l1->next;
            l2 = l2->next;
            pre = cur;
            cur = cur->next;
        }

        // 检查 l1 和 l2 是否遍历完毕
        if (l1 != nullptr) pre->next = l1; // 连接链表
        else if (l2 != nullptr) pre->next = l2; // 连接链表
        // 继续处理下一个
        cur = pre->next;
        // 如果没遍历完, 且有进位, 就继续加进位
        while (cur != nullptr && carry != 0) {
            cur->val += carry;
            // 设置进位
            carry = cur->val / 10;
            cur->val %= 10;

            pre = cur;
            cur = cur->next;
        }

        // 检查是否还有进位剩余, 有就创建新节点
        if (carry != 0) {
            pre->next = new ListNode(carry);
        }

        return res;
    }

    // 优雅的写法, 就是分支太多了性能不好
    ListNode* addTwoNumbers2(ListNode* l1, ListNode* l2) {
        /* 哨兵节点：新建一个 dummy 结点，用于指向返回结果 */
        ListNode *dummy = new ListNode(0);
        ListNode *tmp = dummy;
        /* 存放当前位计算的结果 */
        int t = 0;
        /* l1 或者 l2 有结点就进入循环 */
        while (l1 || l2 || t != 0) {
            int a = (l1 != nullptr) ? l1->val : 0;
            int b = (l2 != nullptr) ? l2->val : 0;
            t = a + b + t;
            tmp->next = new ListNode(t % 10);
            t /= 10;
            if(l1 != nullptr) l1 = l1->next;
            if(l2 != nullptr) l2 = l2->next;
            tmp = tmp->next;
        }
        return dummy->next;
    }
};