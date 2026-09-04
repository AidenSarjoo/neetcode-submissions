from collections import deque

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isCompleteTree(self, root: Optional[TreeNode]) -> bool:
        lst = []
        q = deque()
        none_flag = False 

        q.append(root)

        while len(q) != 0:
            cur = q.popleft()
            if cur is None:
                none_flag = True
            else:
                if none_flag:
                    return False
                q.append(cur.left)
                q.append(cur.right)
        
        return True 