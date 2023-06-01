/*
 * @lc app=leetcode id=2 lang=csharp
 *
 * [2] Add Two Numbers
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     public int val;
 *     public ListNode next;
 *     public ListNode(int val=0, ListNode next=null) {
 *         this.val = val;
 *         this.next = next;
 *     }
 * }
 */

public class Solution
{
    public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
    {
        var sum = (l1?.val ?? 0) + (l2?.val ?? 0);
        var listNode = new ListNode(sum % 10);
        var first = listNode;
        int cr = sum / 10;
        l1 = l1?.next;
        l2 = l2?.next;
        while ((l1 != null) || (l2 != null) || (cr != 0))
        {
            sum = (l1?.val ?? 0) + (l2?.val ?? 0) + cr;
            var tempListNode = new ListNode(sum % 10);
            listNode.next = tempListNode;
            listNode = tempListNode;
            cr = sum / 10;
            l1 = l1?.next;
            l2 = l2?.next;
        }
        return first;
    }
}
// @lc code=end

