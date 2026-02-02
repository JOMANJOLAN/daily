package org.innuendo.leetcode.beginner.linkedlist;

/**
 * 回文链表
 */
public class Solution5 {
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

//    // Solution 1
//    ListNode frontPointer;
//
//    boolean recursivelyCheck(ListNode currentNode) {
//        if (currentNode != null) {
//            if (!recursivelyCheck(currentNode.next)) {
//                return false;
//            }
//            if (currentNode.val != frontPointer.val) {
//                return false;
//            }
//            frontPointer = frontPointer.next;
//        }
//        return true;
//    }
//
//    boolean isPalindrome(ListNode head) {
//        frontPointer = head;
//        return recursivelyCheck(head);
//    }

//    // Solution 2
//    boolean isPalindrome(ListNode head) {
//        if (head == null) {
//            return true;
//        }
//
//        // 找到前半部分链表的尾节点并反转后半部分链表
//        ListNode firstHalfEnd = endOfFirstHalf(head);
//        ListNode secondHalfStart = reverseList(firstHalfEnd.next);
//
//        // 判断是否回文
//        ListNode p1 = head;
//        ListNode p2 = secondHalfStart;
//        boolean result = true;
//        while (result && p2 != null) {
//            if (p1.val != p2.val) {
//                result = false;
//            }
//            p1 = p1.next;
//            p2 = p2.next;
//        }
//
//        // 还原链表并返回结果
//        firstHalfEnd.next = reverseList(secondHalfStart);
//        return result;
//    }
//
//    ListNode reverseList(ListNode head) {
//        ListNode prev = null;
//        ListNode curr = head;
//        while (curr != null) {
//            ListNode nextTemp = curr.next;
//            curr.next = prev;
//            prev = curr;
//            curr = nextTemp;
//        }
//        return prev;
//    }
//
//    ListNode endOfFirstHalf(ListNode head) {
//        ListNode fast = head;
//        ListNode slow = head;
//        while (fast.next != null && fast.next.next != null) {
//            fast = fast.next.next;
//            slow = slow.next;
//        }
//        return slow;
//    }

    // Solution 3
    boolean isPalindrome(ListNode head) {
        ListNode slow = head;
        ListNode fast = head;
        ListNode curr = null;
        while (fast != null && fast.next != null) {
            fast = fast.next.next;
            ListNode tmp = slow.next;
            slow.next = curr;
            curr = slow;
            slow = tmp;
        }
        if (fast == null) {
            while (slow != null) {
                assert curr != null;
                if (slow.val != curr.val) return false;
                slow = slow.next;
                curr = curr.next;
            }
        }
        if (fast != null) {
            slow = slow.next;
            while (slow != null) {
                assert curr != null;
                if (slow.val != curr.val) return false;
                slow = slow.next;
                curr = curr.next;
            }
        }
        return true;
    }
}
