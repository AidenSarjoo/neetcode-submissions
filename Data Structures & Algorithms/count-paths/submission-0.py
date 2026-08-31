class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        
        dp = {(0,0) : 1}
        def helper(x, y):
            if x < 0:
                return 0
            if y < 0:
                return 0
            if (x,y) in dp.keys():
                return dp[(x,y)]
            
            dp[(x,y)] = helper(x-1, y) + helper(x, y-1)
            return dp[(x,y)]
        return helper(m-1, n-1)