class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        ops = set(["*", "+", "-", "/"])

        for c in tokens:
            if c in ops:
                val2 = stack.pop()
                val1 = stack.pop()
                if c == "*":
                    stack.append(val1 * val2)
                if c == "+":
                    stack.append(val1 + val2)
                if c == "-":
                    stack.append(val1 - val2)
                if c == "/":
                    stack.append(int(val1 / val2))
            else:
                stack.append(int(c))
        
        return stack.pop()
