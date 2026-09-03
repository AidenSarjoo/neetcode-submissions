class Solution:
    def findKthNumber(self, n: int, k: int) -> int:
        
        def count_prefix(prefix):
            count = 0
            first = prefix
            last = prefix + 1

            while first <= n:
                count += min(n + 1, last) - first
                first *= 10
                last *= 10
            return count

        ptr = 1
        k -= 1

        while k:
            count = count_prefix(ptr)
            
            if k >= count:
                k -= count
                ptr += 1
            else:
                ptr *= 10 # value itself
                k -= 1
        return ptr 