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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        auto* cur = head;
        // 统计链表长度
        int len = 0;
        while (cur != nullptr) {
            len++;
            cur = cur->next;
        }
        // 增加一个头节点用来处理一些 eage cases
        auto* dummy = new ListNode(0);
        dummy->next = head;
        // 计算倒数第 n 个是顺数第几个
        int k = len - n + 1;
        k--; // 找到前一个
        cur = dummy;
        while (k > 0) {
            cur = cur->next;
            k--;
        }
        // 删除一个 (这里内存泄漏了, 但是不重要)
        cur->next = cur->next->next;

        return head;
    }
    // 还有两种解法: 递归和快慢指针有待补充
};
