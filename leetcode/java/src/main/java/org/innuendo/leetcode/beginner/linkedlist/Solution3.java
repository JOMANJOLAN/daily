package org.innuendo.leetcode.beginner.linkedlist;

/**
 * 反转链表
 */
public class Solution3 {
    static class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    ListNode reverseList(ListNode head) {
        if (head == null || head.next == null) return head;
        var rear = head;
        var count = 0;
        while (rear.next != null) {
            rear = rear.next;
            count++;
        }
        for (int i = 0; i < count; i++) {
            var buf = head;
            head = head.next;
            buf.next = rear.next;
            rear.next = buf;
        }
        return head;
    }
}
