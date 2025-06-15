# 125. Valid Palindrome
`Easy`

A phrase is a **palindrome** if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string `s`, *return `true` if it is a **palindrome**, or `false` otherwise.*

### Examples

#### Example 1
- **Input:** `s = "A man, a plan, a canal: Panama"`
- **Output:** `true`
- **Explanation:** `"amanaplanacanalpanama" is a palindrome.`

#### Example 2
- **Input:** `s = "race a car"`
- **Output:** `false`
- **Explanation:** `"raceacar" is not a palindrome.`

### Constraints:
- 1 <= `s.length` <= `2 * 10`<sup>`5`</sup>
- `s` consists only of printable ASCII characters.

### Topics
- `Two Pointers`
- `String`