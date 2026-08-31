class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        seen = defaultdict(int)
        buckets = [set() for _ in range(len(nums) + 1)]
        buckets[0] = set(nums)

        for n in nums:
            seen[n] += 1
            buckets[seen[n]].add(n)
            buckets[seen[n]-1].remove(n)
            

        results = []
        while len(results) < k:
            results += [x for x in buckets.pop()]

        return results