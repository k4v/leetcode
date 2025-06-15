def firstBadVersion(self, n: int) -> int:
    l = 0
    r = n - 1

    while l <= r:
        mid = (l + r) // 2
        if isBadVersion(mid + 1):
            if l == mid:
                return mid + 1
            else:
                r = mid
        else:
            l = mid + 1

    return 1