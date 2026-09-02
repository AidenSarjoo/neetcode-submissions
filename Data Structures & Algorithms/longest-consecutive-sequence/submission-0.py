class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        numss = set(nums)
        sol = 0
        for num in nums:
            if (num - 1) in numss:
                continue
            tmp = num
            sl = 0
            while tmp in numss:
                tmp += 1
                sl += 1
                if sl > sol:
                    sol = sl
            
        return sol