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
    // 定义一个反转链表的函数，输入是链表头节点和要反转的区间[left, right]，输出是反转后的链表头节点
    ListNode *reverseBetween(ListNode *head, int left, int right) {
        // 创建一个虚拟节点，指向原链表头节点，方便处理边界情况
        auto *dummy = new ListNode();
        dummy->next = head;
        // 创建一个指针p，用来遍历链表，初始指向虚拟节点
        auto *p = dummy;
        // 计算需要移动的步数，即left-1
        auto step = left - 1;
        // 移动p到要反转区间的前一个节点
        while (step > 0) {
            p = p->next;
            step -= 1;
        }
        // 创建三个指针pre, cur, next，用来实现局部反转
        ListNode *pre = nullptr; // 指向当前节点的前一个节点
        ListNode *cur = p->next; // 指向当前要反转的节点
        step = right - left + 1; // 计算需要反转的次数，即right-left+1
        while (step > 0){               // 反复执行以下操作直到完成局部反转
            ListNode *next = cur->next; // 指向当前节点的后一个节点
            cur->next = pre;            // 将当前节点指向前一个节点，实现反转
            pre = cur;                  // 更新pre为当前节点
            cur = next;                 // 更新cur为下一个要反转的节点
            step--;
        }
        // 将原来区间内第一个和最后一个元素与剩余部分连接起来
        if (p->next != nullptr) {
            p->next->next = cur;
        }
        p->next = pre;
        return dummy->next; // 返回新链表头结点（虚拟结点后面那个）
    }
};