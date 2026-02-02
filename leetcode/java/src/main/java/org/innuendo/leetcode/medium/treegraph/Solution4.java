package org.innuendo.leetcode.medium.treegraph;

import java.util.ArrayDeque;

/**
 * 填充每个节点的下一个右侧节点指针
 */
public class Solution4 {
    public static class Node {
        public int val;
        public Node left;
        public Node right;
        public Node next;

        public Node() {
        }

        public Node(int _val) {
            val = _val;
        }

        public Node(int _val, Node _left, Node _right, Node _next) {
            val = _val;
            left = _left;
            right = _right;
            next = _next;
        }
    }

    public Node connect(Node root) {
        if (root == null) return null;
        var queue = new ArrayDeque<Node>();
        queue.addFirst(root);
        while (!queue.isEmpty()) {
            var node = queue.pollFirst();
            if (node.left != null) {
                queue.addLast(node.left);
            }
            if (node.right != null) {
                queue.addLast(node.right);
            }
            node.next = queue.peekFirst();
        }
        var last = root;
        while (last != null) {
            last.next = null;
            last = last.right;
        }
        return root;
    }
}
