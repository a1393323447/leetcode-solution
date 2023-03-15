#include <bits/stdc++.h>
using namespace std;

class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};

class Solution {
public:
    vector<Node*> nodes;
    bool* is_vis = nullptr;

    Solution() {
        for (int i = 0; i <= 100; i++) {
            this->nodes.push_back(new Node(i));
        }
        this->is_vis = new bool[101];
    }

    Node* cloneGraph(Node* node) {
        queue<Node*> q{};
        q.push(node);

        while (!q.empty()) {
            auto cur_node = q.front();
            q.pop();

            auto val = cur_node->val;
            if (is_vis[val]) {
                continue;
            }

            is_vis[val] = true;
            for (auto nb: cur_node->neighbors) {
                nodes[val]->neighbors.push_back(nodes[nb->val]);
                q.push(nb);
            }
        }

        return nodes[node->val];
    }
};