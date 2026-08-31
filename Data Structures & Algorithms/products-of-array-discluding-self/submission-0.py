class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        prefix = [1 for num in nums]
        suffix = [1 for num in nums]

        for i in range(1, len(nums)):
            prefix[i] *= prefix[i-1] * nums[i-1]
        
        for i in range(len(nums)-1, 0, -1):
            suffix[i-1] *= suffix[i] * nums[i]
        
        return [suffix[i] * prefix[i] for i in range(len(nums))]
        