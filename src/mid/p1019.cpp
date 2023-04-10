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
    vector<int> nextLargerNodes(ListNode* head) {
        vector<int> values {};
        
        auto* cur = head;
        while (cur != nullptr) {
            values.push_back(cur->val);
            cur = cur->next;
        }
        
        auto size = values.size();
        auto* dp = new int[size]{0};
        dp[size - 1] = 0;
        if (size < 2) {
            return values;
        }

        for (int i = size - 2; i >= 0; i--) {
            if (values[i] < values[i + 1]) {
                dp[i] = values[i + 1];
            } else {
                dp[i] = 0;
                int j = i + 1;
                while (j < size) {
                    if (values[i] < dp[j] || dp[j] == 0) {
                        dp[i] = dp[j];
                        break;
                    }
                    j++;
                }
            }
        }

        return vector<int>(&dp[0], &dp[size]);
    }
};

// 单调栈 + 反转链表
class Solution2 {
public:
    vector<int> nextLargerNodes(ListNode* head) {
        auto info = reverseList(head);
        auto new_head = info.head;
        auto len = info.len;

        auto* answer = new int[len]{0};
        stack<int> s{};

        int i = len - 1;
        auto cur = new_head;
        while (cur != nullptr) {
            auto cur_value = cur->val;
            while (!s.empty() && cur_value >= s.top()) {
                s.pop();
            }
            answer[i] = s.empty() ? 0 : s.top();
            s.push(cur_value);
            cur = cur->next;
            i--;
        }

        return vector<int>(&answer[0], &answer[len]);
    }

    struct ReverseInfo {
        int len;
        ListNode* head;
    };
    // 双指针
    ReverseInfo reverseList(ListNode* head) {
        ListNode *pre = nullptr;
        ListNode *cur = head;
        int len = 0;
        while(cur != nullptr) {
            ListNode *next = cur->next;
            cur->next = pre;
            pre = cur;
            cur = next;
            len++;
        }
        return ReverseInfo {
            .len = len, 
            .head = pre
        };
    }
};