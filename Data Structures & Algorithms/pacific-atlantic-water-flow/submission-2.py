class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        cb = len(heights[0]) - 1
        rb = len(heights) - 1
        pacific = set()
        atlantic = set()

        pacs = [(0,y) for y in range(len(heights[0]))] + [(x,0) for x in range(len(heights))]
        ats = [(rb, y) for y in range(len(heights[0]))] + [(x, cb) for x in range(len(heights))]

        def get_borders(r,c):
            border_cells = set()
            if r != 0: 
                border_cells.add((r-1, c))
            if c != 0:
                border_cells.add((r, c-1))
            if r != rb:
                border_cells.add((r+1, c))
            if c != cb:
                border_cells.add((r,c+1))
            
            return border_cells

        while len(pacs) > 0:
            cur = pacs.pop()
            if cur in pacific:
                pass
            else:
                pacific.add(cur)
                bc = get_borders(*cur)

                for r,c in bc:
                    if heights[r][c] >= heights[cur[0]][cur[1]]:
                        pacs.append((r,c))

        while len(ats) > 0:
            cur = ats.pop()
            if cur in atlantic:
                pass
            else:
                atlantic.add(cur)
                bc = get_borders(*cur)

                for r,c in bc:
                    if heights[r][c] >= heights[cur[0]][cur[1]]:
                        ats.append((r,c))
        
        return [[r,c] for r,c in pacific & atlantic]
