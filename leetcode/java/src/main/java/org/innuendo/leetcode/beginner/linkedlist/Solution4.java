package org.innuendo.leetcode.beginner.linkedlist;

/**
 * 合并两个有序链表
 */
public class Solution4 {
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

    ListNode mergeTwoLists(ListNode list1, ListNode list2) {
        var fakeHead = new ListNode(0);
        var curr = fakeHead;
        while (list1 != null && list2 != null) {
            if (list1.val < list2.val) {
                curr.next = list1;
                list1 = list1.next;
            } else {
                curr.next = list2;
                list2 = list2.next;
            }
            curr = curr.next;
        }
        while (list1 != null) {
            curr.next = list1;
            list1 = list1.next;
            curr = curr.next;
        }
        while (list2 != null) {
            curr.next = list2;
            list2 = list2.next;
            curr = curr.next;
        }
        return fakeHead.next;
    }
}
