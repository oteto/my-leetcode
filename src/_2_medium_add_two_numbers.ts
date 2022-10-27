import { assertEquals } from "test";
/*
 * @lc app=leetcode id=2 lang=typescript
 *
 * [2] Add Two Numbers
 */

class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

// @lc code=start
function addTwoNumbers(
  l1: ListNode | null,
  l2: ListNode | null,
): ListNode | null {
  let ret: ListNode | null = null;
  let tmp: ListNode | null = null;
  let up = 0;

  while (l1 || l2 || up != 0) {
    const v1 = l1?.val ?? 0;
    const v2 = l2?.val ?? 0;
    l1 &&= l1?.next;
    l2 &&= l2?.next;
    const sum = v1 + v2 + up;
    if (tmp !== null) {
      tmp.next = new ListNode(sum % 10);
      tmp = tmp.next;
    } else {
      ret = new ListNode(sum % 10);
      tmp = ret;
    }
    up = Math.floor(sum / 10);
  }

  return ret;
}
// @lc code=end

Deno.test("addTwoNumbers", (_) => {
  assertEquals(
    addTwoNumbers(
      new ListNode(2, new ListNode(4, new ListNode(3, new ListNode(9)))),
      new ListNode(5, new ListNode(6, new ListNode(4, new ListNode(9)))),
    ),
    new ListNode(
      7,
      new ListNode(0, new ListNode(8, new ListNode(8, new ListNode(1)))),
    ),
  );
});
