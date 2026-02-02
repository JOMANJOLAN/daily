package org.innuendo.leetcode.medium.linkedlist;

/**
 * 相交链表
 */
public class Solution3 {
    public static class ListNode {
        int val;
        ListNode next;

        ListNode(int x) {
            val = x;
            next = null;
        }
    }

    public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        if (headA == headB) return headB;
        var rearB = headB;
        while (rearB.next != null) {
            rearB = rearB.next;
        }
        rearB.next = headB;
        var fast = headA;
        var slow = headA;
        while (fast.next != null && fast.next.next != null) {
            fast = fast.next.next;
            slow = slow.next;
            if (fast == slow) {
                var third = headA;
                while (third != slow) {
                    third = third.next;
                    slow = slow.next;
                }
                rearB.next = null;
                return third;
            }
        }
        rearB.next = null;
        return null;
    }
}
