# 1. Two Sum
`Easy`

Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

You may assume that each input would have **exactly one solution**, and you may not use the same element twice.

You can return the answer in any order.

### Examples

#### Example 1
- **Input:** `nums = [2,7,11,15], target = 9`
- **Output:** `[0,1]`
- **Explanation:** `Because nums[0] + nums[1] == 9, we return [0, 1].`

#### Example 2
- **Input:** `nums = [3,2,4], target = 6`
- **Output:** `[1,2]`

### Constraints:
- 2 <= `nums.length` <= 10<sup>4</sup>
- -10<sup>9</sup> <= `nums[i]` <= 10<sup>9</sup>
- -10<sup>9</sup> <= `target` <= 10<sup>9</sup>
- Only 1 valid answer exists

### Topics
- `Array`
- `Hash Table`