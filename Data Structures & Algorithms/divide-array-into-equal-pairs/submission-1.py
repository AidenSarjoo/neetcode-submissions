from collections import Counter

class Solution:
    def divideArray(self, nums: List[int]) -> bool:
        c = Counter(nums)
        for n in c:
            if c[n] % 2 != 0:
                return False
        return True 