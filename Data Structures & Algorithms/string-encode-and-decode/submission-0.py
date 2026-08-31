class Solution:

    def encode(self, strs: List[str]) -> str:
        code = ""
        for word in strs:
            code += chr(len(word)) + word
        return code

    def decode(self, s: str) -> List[str]:
        decoded = []
        if len(s) == 0:
            return decoded
        
        cur = 0
        while cur < len(s):
            word = ""
            wl = ord(s[cur])
            word += s[cur+1 : cur+wl+1]
            decoded.append(word)
            cur += wl + 1
        
        return decoded