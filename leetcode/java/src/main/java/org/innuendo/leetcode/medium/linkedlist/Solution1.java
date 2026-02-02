package org.innuendo.leetcode.medium.linkedlist;

/**
 * 两数相加
 */
public class Solution1 {
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

    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        var cnt = 0;
        var fakeHead = new ListNode();
        var curr = fakeHead;
        while (l1 != null || l2 != null || cnt != 0) {
            if (l1 != null) {
                cnt += l1.val;
                l1 = l1.next;
            }
            if (l2 != null) {
                cnt += l2.val;
                l2 = l2.next;
            }
            var tmp = new ListNode(cnt % 10);
            cnt /= 10;
            curr.next = tmp;
            curr = tmp;
        }
        return fakeHead.next;
    }
}
