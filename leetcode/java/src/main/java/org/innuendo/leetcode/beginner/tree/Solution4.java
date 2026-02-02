package org.innuendo.leetcode.beginner.tree;

import java.util.ArrayList;
import java.util.List;

/**
 * 二叉树的层次遍历
 */
public class Solution4 {
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

    //    // Solution 1
//    public List<List<Integer>> levelOrder(TreeNode root) {
//        if (root == null) return new ArrayList<>();
//        var result = new ArrayList<List<Integer>>();
//        var queue = new ArrayDeque<TreeNode>();
//        queue.add(root);
//        while (!queue.isEmpty()) {
//            var list = new ArrayList<Integer>();
//            for (var node : queue) {
//                list.add(node.val);
//            }
//            result.add(list);
//            var count = queue.size();
//            for (int i = 0; i < count; i++) {
//                var node = queue.poll();
//                assert node != null;
//                var left = node.left;
//                var right = node.right;
//                if (left != null) queue.add(left);
//                if (right != null) queue.add(right);
//            }
//        }
//        return result;
//    }

    // Solution 2
    List<List<Integer>> res = new ArrayList<List<Integer>>();

    public List<List<Integer>> levelOrder(TreeNode root) {
        checkFunc(root, 0);
        return res;
    }

    void checkFunc(TreeNode node, int deep) {
        if (node == null) {
            return;
        }
        deep++;
        if (res.size() < deep) {
            List<Integer> item = new ArrayList<Integer>();
            res.add(item);
        }
        res.get(deep - 1).add(node.val);
        checkFunc(node.left, deep);
        checkFunc(node.right, deep);
    }
}
