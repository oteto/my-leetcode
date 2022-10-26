/*
 * @lc app=leetcode id=1 lang=typescript
 *
 * [1] Two Sum
 */
import { assertEquals } from "test";

// @lc code=start
function twoSum(nums: number[], target: number): number[] {
  const map = new Map<number, number>();
  for (let i = 0; i < nums.length; i++) {
    if (map.has(target - nums[i])) {
      return [map.get(target - nums[i])!, i];
    }
    map.set(nums[i], i);
  }

  return [];
}
// @lc code=end

Deno.test("twoSum", (_) => {
  assertEquals(twoSum([2, 11, 4, 7], 9), [0, 3]);
  assertEquals(twoSum([3, 4, 3, 7, 10], 6), [0, 2]);
});
