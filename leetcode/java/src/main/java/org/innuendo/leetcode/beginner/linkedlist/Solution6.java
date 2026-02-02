package org.innuendo.leetcode.beginner.linkedlist;

/**
 * 环形链表
 */
public class Solution6 {
    static class ListNode {
        int val;
        ListNode next;

        ListNode(int x) {
            val = x;
            next = null;
        }
    }

    boolean hasCycle(ListNode head) {
        var fast = head;
        var slow = head;
        while (fast != null && fast.next != null) {
            fast = fast.next.next;
            slow = slow.next;
            if (fast == slow) {
                return true;
            }
        }
        return false;
    }
}
