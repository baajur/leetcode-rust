class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
    
class Solution:
    def __init__(self):
        self.visited = {}

    def copyRandomList(self, head: 'Node') -> 'Node':
        if head in self.visited:
            return self.visited[head]
        else:
            cloned = Node(head.val)
            cloned.next = self.copyRandomList(head.next)
            cloned.random = self.copyRandomList(head.random)
            self.visited[head] = cloned
            return cloned
