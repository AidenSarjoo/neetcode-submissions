class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        sol = 0
        mins = prices[0]

        for price in prices:
            sol = max(sol, price - mins)
            mins = min(mins, price)
        
        return sol 
                