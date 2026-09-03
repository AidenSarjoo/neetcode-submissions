class Solution:
    def isPalindrome(self, s: str) -> bool:
        lp = 0
        rp = len(s)-1

        while rp > lp:
            print(s[lp], s[rp])
            if s[lp].isalnum() is False:
                lp += 1
            elif s[rp].isalnum() is False:
                rp -= 1
            else:

                if s[lp].lower() != s[rp].lower():
                    return False
                
                lp += 1
                rp -= 1
                
        return True 