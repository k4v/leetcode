climbing_ways = {0: 0, 1: 1, 2: 2}

def climbStairs(self, n: int) -> int:
    if n in climbing_ways:
        return climbing_ways[n]
    
    ways = climbStairs(n - 1) + climbStairs(n - 2)
    climbing_ways[n] = ways
    return ways
