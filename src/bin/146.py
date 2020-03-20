from collections import OrderedDict

class LRUCache:
    def __init__(self, capacity: int):
        self.dict = OrderedDict()
        self.capacity = capacity
    
    def get(self, key: int) -> int:
        if key in self.dict:
            self.dict.move_to_end(key)
            return self.dict[key]
        else:
            return -1
    
    def put(self, key: int, value: int) -> None:
        if key in self.dict:
            self.dict[key] = value
            self.dict.move_to_end(key)
        else:
            if len(self.dict) == self.capacity:
                self.dict.popitem(last=False)
            self.dict[key] = value

