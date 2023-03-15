#include <bits/stdc++.h>
using namespace std;

/**
 * Definition for a binary tree node.
 */
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

// 空间复杂度 O(H): H 为树高
class Solution {
public:
    TreeNode* prev = nullptr;
    TreeNode* t1 = nullptr;
    TreeNode* t2 = nullptr;

    void recoverTree(TreeNode* root) {
        if (root == nullptr) return ;

        trans(root);

        std::swap(t1->val, t2->val);
    }

    void trans(TreeNode* root) {
        if (root == nullptr) {
            return;
        }

        trans(root->left);
        if (prev != nullptr && prev->val > root->val) {
            if (t1 == nullptr) {
                t1 = prev;
            }
            t2 = root;
        }
        prev = root;
        trans(root->right);
    }
};

// 还有一个空间复杂度为 O(1) 的 Morris 中序遍历
class Solution2 {
public:
    void recoverTree(TreeNode* root) {
        TreeNode *x = nullptr, *y = nullptr, *pred = nullptr, *predecessor = nullptr;

        while (root != nullptr) {
            if (root->left != nullptr) {
                // predecessor 节点就是当前 root 节点向左走一步，然后一直向右走至无法走为止
                predecessor = root->left;
                while (predecessor->right != nullptr && predecessor->right != root) {
                    predecessor = predecessor->right;
                }
                
                // 让 predecessor 的右指针指向 root，继续遍历左子树
                if (predecessor->right == nullptr) {
                    predecessor->right = root;
                    root = root->left;
                }
                // 说明左子树已经访问完了，我们需要断开链接
                else {
                    if (pred != nullptr && root->val < pred->val) {
                        y = root;
                        if (x == nullptr) {
                            x = pred;
                        }
                    }
                    pred = root;

                    predecessor->right = nullptr;
                    root = root->right;
                }
            }
            // 如果没有左孩子，则直接访问右孩子
            else {
                if (pred != nullptr && root->val < pred->val) {
                    y = root;
                    if (x == nullptr) {
                        x = pred;
                    }
                }
                pred = root;
                root = root->right;
            }
        }
        swap(x->val, y->val);
    }
};