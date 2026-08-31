class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        sm = {}
        tm = {}

        for c in s:
            if c not in sm.keys():
                sm[c] = 1
            else:
                sm[c] += 1
        
        for c in t:
            if c not in tm.keys():
                tm[c] = 1
            else:
                tm[c] += 1
        
        return sm == tm