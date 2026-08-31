from collections import defaultdict

class Solution:
    def longestPalindrome(self, s: str) -> int:
        letters = defaultdict(int)
        for c in s:
            letters[c] += 1
        
        sum = 0
        odd = False
        for c in letters:
            if letters[c] % 2 == 1:
                odd = True
            sum += (letters[c] // 2) * 2
        
        return sum + (1 if odd else 0)