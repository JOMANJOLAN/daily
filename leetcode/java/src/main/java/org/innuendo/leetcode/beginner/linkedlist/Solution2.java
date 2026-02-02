package org.innuendo.leetcode.beginner.linkedlist;

/**
 * 删除链表的倒数第N个节点
 */
public class Solution2 {
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

    ListNode removeNthFromEnd(ListNode head, int n) {
        head = new ListNode(0, head);
        var fast = head;
        var slow = head;

        for (int i = 0; i < n; i++) {
            fast = fast.next;
        }

        while (fast.next != null) {
            fast = fast.next;
            slow = slow.next;
        }

        slow.next = slow.next.next;
        return head.next;
    }
}
