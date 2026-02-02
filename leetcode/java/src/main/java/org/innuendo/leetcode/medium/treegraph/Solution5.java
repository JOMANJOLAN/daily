package org.innuendo.leetcode.medium.treegraph;

/**
 * 二叉搜索树中第K小的元素
 */
public class Solution5 {
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    public int kthSmallest(TreeNode root, int k) {
        class KthSmallest {
            int curr = 0;
            int kth = 0;

            boolean inorderFind(TreeNode root, int k) {
                if (root == null) return false;
                if (inorderFind(root.left, k)) {
                    return true;
                }
                curr++;
                if (curr == k) {
                    kth = root.val;
                    return true;
                }
                return inorderFind(root.right, k);
            }

            int find(TreeNode root, int k) {
                inorderFind(root, k);
                return kth;
            }
        }

        return new KthSmallest().find(root, k);
    }
}
