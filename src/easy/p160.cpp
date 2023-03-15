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

// 三种解法
// 1. Hash
// 2. 先求出两个链表的长度, 然后长的先走几步直到和短的一样, 再一起走
// 3. 神奇的双指针: 
// 证明 https://leetcode.cn/problems/intersection-of-two-linked-lists/solutions/811625/xiang-jiao-lian-biao-by-leetcode-solutio-a8jn/

// 解法三
class Solution {
public:
    ListNode* getIntersectionNode(ListNode *headA, ListNode *headB) {
        if (headA == nullptr || headB == nullptr) return nullptr;
        auto* pa = headA;
        auto* pb = headB;
        while (pa != pb) {
            pa = pa->next;
            pb = pb->next;
            if (pa == nullptr && pb == nullptr) return nullptr;
            if (pa == nullptr) pa = headB;
            if (pb == nullptr) pb = headA; 
        }
        return pa;
    }
};