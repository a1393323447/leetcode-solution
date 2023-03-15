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
    ListNode* partition(ListNode* head, int x) {
    // 创建两个虚拟头节点，用来存储分割后的两个子链表
    auto *less_dummy = new ListNode(); // 存储小于x的节点
    auto *rest_dummy = new ListNode(); // 存储大于或等于x的节点

    // 创建两个指针，用来遍历和连接子链表中的节点
    auto *less_node = less_dummy; 
    auto *rest_node = rest_dummy;

    // 创建一个指针，用来遍历原始链表中的节点
    auto *cur = head;

    // 当原始链表没有遍历完时，循环执行以下操作：
    while (cur != nullptr) {
        // 如果当前节点的值小于x，则把它连接到小于x的子链表中，并更新指针位置
        if (cur->val < x) {
            less_node->next = cur; 
            less_node = less_node->next;
        }
        // 否则，把它连接到大于或等于x的子链表中，并更新指针位置
        else {
            rest_node->next = cur;
            rest_node = rest_node->next;
        }
        // 更新当前节点为下一个节点，继续遍历原始链表
        cur = cur->next;
    }

    // 把两个子链表最后一个节点后面都置为空，防止出现循环引用或内存泄漏
    less_node->next = nullptr;
    rest_node->next = nullptr;

    // 如果小于x的子链表为空，则直接返回大于或等于x的子链表头节点（跳过虚拟头节点）
    if (less_dummy->next == nullptr) {
        return rest_dummy->next;
    }
    else { 
        // 否则，把小于x和大于或等于x两个子链表连接起来，并返回小于x子链表头结点（跳过虚拟头结点）
        less_node->next = rest_dummy->next;
        return less_dummy->next;
    }
}
};