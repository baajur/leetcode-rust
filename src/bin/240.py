class Solution:
    def searchMatrix(self, matrix, target):
        if len(matrix) == 0:
            return False
        if len(matrix[0]) == 0:
            return False

        row_upper = row_search(matrix, target)
        col_upper = col_search(matrix, target)

        for i in range(row_upper + 1):
            if found(matrix[i], target, col_upper + 1):
                return True
        
        return False


def row_search(matrix, target):
    ok = -1
    ng = len(matrix)
    while ng - ok > 1:
        mid = (ok + ng) // 2
        if matrix[mid][0] > target:
            ng = mid
        else:
            ok = mid
    return ok

def col_search(matrix, target):
    ok = -1
    ng = len(matrix[0])
    while ng - ok > 1:
        mid = (ok + ng) // 2
        if matrix[0][mid] > target:
            ng = mid
        else:
            ok = mid
    return ok

def found(row, target, col_upper):
    ok = -1
    ng = col_upper
    while ng - ok > 1:
        mid = (ok + ng) // 2
        if row[mid] <= target:
            ok = mid
        else:
            ng = mid
    
    if ok != -1 and row[ok] == target:
        return True
    else:
        return False
