import { assertEquals } from "test";
/*
 * @lc app=leetcode id=6 lang=typescript
 *
 * [6] ZigZag Conversion
 */

// @lc code=start
function convert(s: string, numRows: number): string {
  const rows: [string, number][] = [];
  let f = true;
  for (let i = 0, j = 0; i < s.length; i++) {
    rows.push([s[i], j]);
    if (f) {
      j++;
      if (j == numRows - 1) {
        f = false;
      }
    } else {
      j--;
      if (j == 0) {
        f = true;
      }
    }
  }

  rows.sort(([, a], [, b]) => a - b);

  return rows.map(([c]) => c).join("");
}
// @lc code=end

Deno.test("convert", (_) => {
  assertEquals(convert("PAYPALISHIRING", 4), "PINALSIGYAHRPI");
  assertEquals(convert("A", 1), "A");
});
