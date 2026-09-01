class Solution:
    def isValid(self, s: str) -> bool:
        stack = []

        mp = {"(" : ")", "{" : "}", "[" : "]"}

        for c in s:
            if c in mp.keys():
                stack.append(c)
            else:
                if len(stack) <= 0: 
                    return False 
                if mp[stack.pop()] != c:
                    return False

        return len(stack) == 0