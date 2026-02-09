package org.innuendo.leetcode.array;

public class Solution106 {
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

    private int[] inorder;
    private int[] postorder;

    private TreeNode build(int iStart, int pStart, int length) {
        if (length == 0) return null;
        var pivot = postorder[pStart + length - 1];
        var root = new TreeNode(pivot);
        var iPivot = iStart;
        while (inorder[iPivot] != pivot) iPivot++;
        var leftLength = iPivot - iStart;
        var rightLength = length - leftLength - 1;
        root.left = build(iStart, pStart, leftLength);
        root.right = build(iPivot + 1, pStart + leftLength, rightLength);
        return root;
    }

    public TreeNode buildTree(int[] inorder, int[] postorder) {
        var length = inorder.length;
        if (length == 0) return null;
        this.inorder = inorder;
        this.postorder = postorder;
        return build(0, 0, length);
    }
}
