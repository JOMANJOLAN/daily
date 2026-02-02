package org.innuendo.leetcode.medium.linkedlist;

/**
 * 奇偶链表
 */
public class Solution2 {
    public static class ListNode {
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

    public ListNode oddEvenList(ListNode head) {
        var odd = new ListNode();
        var even = new ListNode();
        var oddCurr = odd;
        var evenCurr = even;
        var flag = true;
        while (head != null) {
            if (flag) {
                oddCurr.next = head;
                oddCurr = oddCurr.next;
            } else {
                evenCurr.next = head;
                evenCurr = evenCurr.next;
            }
            head = head.next;
            flag = !flag;
        }
        oddCurr.next = even.next;
        evenCurr.next = null;
        return odd.next;
    }
}
