class Solution:
    def isStrobogrammatic(self, num: str) -> bool:
        mp = {"8" : "8", "6" : "9", "9" : "6", "1" : "1", "0" : "0"}
        new = ""
        for c in num:
            if c in mp:
                new += mp[c]
            else:
                return False
        
        return new[::-1] == num