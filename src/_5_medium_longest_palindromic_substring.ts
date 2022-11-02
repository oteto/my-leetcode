import { assertEquals } from "test";
/*
 * @lc app=leetcode id=5 lang=typescript
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
function longestPalindrome(s: string): string {
  if (s.length < 2) {
    return s;
  }

  let start = 0;
  let maxLength = 1;
  let i = 0;

  while (i < s.length) {
    let left = i;
    let right = i;

    while (right < s.length - 1 && s.at(right) === s.at(right + 1)) {
      right++;
    }
    i = right + 1;
    while (
      right < s.length - 1 && left > 0 && s.at(right + 1) === s.at(left - 1)
    ) {
      left--;
      right++;
    }
    if (maxLength < right - left + 1) {
      start = left;
      maxLength = right - left + 1;
    }
  }
  return s.slice(start, start + maxLength);
}
// @lc code=end

Deno.test("longestPalindrome", (_) => {
  assertEquals(longestPalindrome("abacd"), "aba");
});
