package org.innuendo.leetcode.medium.treegraph;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;

/**
 * 二叉树的锯齿状层次遍历
 */
public class Solution2 {
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

    public List<List<Integer>> zigzagLevelOrder(TreeNode root) {
        if (root == null) return new ArrayList<>();
        var lists = new ArrayList<List<Integer>>();
        var queue = new ArrayDeque<TreeNode>();
        var flag = false;
        queue.add(root);
        while (!queue.isEmpty()) {
            var size = queue.size();
            var list = new ArrayList<Integer>();
            for (int i = 0; i < size; i++) {
                if (flag) {
                    var node = queue.pollLast();
                    assert node != null;
                    list.add(node.val);
                    if (node.right != null) queue.addFirst(node.right);
                    if (node.left != null) queue.addFirst(node.left);
                } else {
                    var node = queue.pollFirst();
                    assert node != null;
                    list.add(node.val);
                    if (node.left != null) queue.addLast(node.left);
                    if (node.right != null) queue.addLast(node.right);
                }
            }
            lists.add(list);
            flag = !flag;
        }
        return lists;
    }
}
