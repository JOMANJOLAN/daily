package org.innuendo.leetcode.beginner.linkedlist;

/**
 * 删除链表中的节点
 */
public class Solution1 {
    static class ListNode {
        int val;
        ListNode next;

        ListNode(int x) {
            val = x;
        }
    }

    void deleteNode(ListNode node) {
        var next = node.next;
        node.val = next.val;
        node.next = next.next;
    }
}
