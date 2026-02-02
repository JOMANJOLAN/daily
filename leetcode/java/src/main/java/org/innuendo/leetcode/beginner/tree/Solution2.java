package org.innuendo.leetcode.beginner.tree;

/**
 * 验证二叉搜索树
 */
public class Solution2 {
    public class TreeNode {
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

    // Solution 1
    public boolean isValidBST(TreeNode root) {
        return check(root, Long.MIN_VALUE, Long.MAX_VALUE);
    }

    boolean check(TreeNode node, long low, long up) {
        if (node.val <= low || node.val >= up) return false;
        if (node.left == null && node.right == null) return true;
        boolean leftCheck = node.left == null || check(node.left, low, node.val);
        boolean rightCheck = node.right == null || check(node.right, node.val, up);
        return leftCheck && rightCheck;
    }
}
