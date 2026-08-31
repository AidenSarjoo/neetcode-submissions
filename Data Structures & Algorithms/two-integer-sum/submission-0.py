class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        seen = {}
        for i in range(len(nums)):
            tmp = target - nums[i]
            if tmp in seen:
                return [seen[tmp], i]
            
            seen[nums[i]] = i