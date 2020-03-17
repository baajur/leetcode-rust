class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def detectCycle(self, head: ListNode) -> ListNode:
        visited = set()
        cur = head
        while cur is not None:
            if cur in visited:
                return cur
            
            visited.add(cur)
            cur = cur.next
        return None