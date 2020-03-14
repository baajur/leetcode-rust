class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def hasCycle(self, head: ListNode) -> bool:
        cur = head
        visited = set()
        while True:
            if cur is None:
                return False
            visited.add(cur)
            if cur.next in visited:
                return True
            cur = cur.next
