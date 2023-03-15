#include <bits/stdc++.h>
using namespace std;

class Node {
public:
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = nullptr;
        random = nullptr;
    }
};

class Solution {
public:
    Node* copyRandomList(Node* head) {
        if (head == nullptr) return nullptr;

        vector<Node*> nodes;
        unordered_map<Node*, int> map{};
        auto cur = head;
        auto idx = 0;
        while (cur != nullptr) {
            auto node = new Node(cur->val);
            nodes.push_back(node);
            if (idx > 0) {
                nodes[idx - 1]->next = node;
            }

            map.insert({cur, idx});
            
            cur = cur->next;
            idx++;
        }

        idx = 0;
        cur = head;
        while (cur != nullptr) {
            if (cur->random == nullptr) {
                nodes[idx]->random = nullptr;
            } else {
                auto entry = map.find(cur->random);
                auto rand_idx = entry->second;
                nodes[idx]->random = nodes[rand_idx];
            }
            idx++;
            cur = cur->next;
        }

        return nodes[0];
    }
};