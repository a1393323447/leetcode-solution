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

// 暴力法
class Solution {
public:
    ListNode *detectCycle(ListNode *head) {
        if (head == nullptr || !hasCycle(head)) {
            return nullptr;
        }

        auto cur = head;
        while (cur != nullptr) {
            auto next = cur->next;
            cur->next = nullptr;
            if (!hasCycle(next)) {
                cur->next= next;
                return cur;
            };
            cur->next = next;
            cur = next;
        }

        return nullptr;
    }

    bool hasCycle(ListNode* head) {
        if (head == nullptr) return false;

        auto s = head;
        auto f = head->next;
        while (f != s) {
            if (f == nullptr || f->next == nullptr) {
                return false;
            }
            s = s->next;
            f = f->next->next;
        }

        return true;
    }
};

// 双指针 数学
// 解释: https://leetcode.cn/problems/linked-list-cycle-ii/solutions/12616/linked-list-cycle-ii-kuai-man-zhi-zhen-shuang-zhi-/
class Solution2 {
public:
    ListNode *detectCycle(ListNode* head) {
        if (head == nullptr) return nullptr;

        // 设链表总长为: n = a + b
        // 其中: b 为环长

        auto s = head;
        auto f = head;
        for (;;) {
            if (f == nullptr || f->next == nullptr) {
                return nullptr;
            }
            s = s->next;
            f = f->next->next;
            if (f == s) {
                break;
            }
        }

        // 此时 f == s
        // 有: step_f = 2 * step_s
        // step_f = step_s + nb
        // 即: step_s = nb step_f = 2 * nb
        // 再令 f = head, step_f = 0
        // 同时将 f, s 向前移
        // 当 step_f = a 时, step_s = a + nb
        // 因为 链表成环 所以此时必然有: f == b
        // 而 step_f = a , 即 f 前有 a 个结点
        // 也意味着 f 刚好走到环中的第一个节点

        f = head;
        while (f != s) {
            f = f->next;
            s = s->next;
        }

        return f;
    }
};