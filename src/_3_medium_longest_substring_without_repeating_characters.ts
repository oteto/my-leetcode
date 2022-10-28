import { assertEquals } from "test";
/*
 * @lc app=leetcode id=3 lang=typescript
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
function lengthOfLongestSubstring(s: string): number {
  const curr: string[] = [];
  let max = 0;

  for (const c of s) {
    const i = curr.indexOf(c);
    if (i !== -1) {
      for (let j = 0; j <= i; j++) {
        curr.shift();
      }
    }
    curr.push(c);
    max = Math.max(max, curr.length);
  }
  return max;
}
// @lc code=end

Deno.test("lengthOfLongestSubstring", (_) => {
  assertEquals(lengthOfLongestSubstring("pwwkew"), 3);
  assertEquals(lengthOfLongestSubstring("abcabcdbg"), 4);
  assertEquals(lengthOfLongestSubstring("aaaa"), 1);
});
