package org.innuendo.leetcode.medium.treegraph;

/**
 * 从前序与中序遍历序列构造二叉树
 */
public class Solution3 {
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

    public TreeNode buildTree(int[] preorder, int[] inorder) {
        record Builder(int[] preorder, int[] inorder) {
            TreeNode buildTree(int m, int n, int len) {
                if (len == 0) {
                    return null;
                }
                var i = 0;
                for (; i < len; i++) {
                    if (preorder[m] == inorder[n + i]) {
                        break;
                    }
                }
                var root = new TreeNode(preorder[m]);
                root.left = buildTree(m + 1, n, i);
                root.right = buildTree(m + 1 + i, n + i + 1, len - 1 - i);
                return root;
            }

        }
        var len = preorder.length;
        return new Builder(preorder, inorder).buildTree(0, 0, len);
    }
}
