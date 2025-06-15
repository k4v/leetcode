def longestPalindrome(self, s: str) -> int:
    counts = defaultdict(int)
    for c in s:
        counts[c] += 1
    
    palindrome_length = 0
    extra_middle = 0

    for count in counts.values():
        palindrome_length += count if (count % 2 == 0) else count - 1
    
    if palindrome_length < len(s):
        palindrome_length += 1
    
    return palindrome_length
