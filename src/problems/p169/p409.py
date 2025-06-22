def majorityElement(self, nums: List[int]) -> int:
    """
    Starting with the first number as a candidate majority element, keep eliminating the
    candidate majority element as a pair with the current element, if they are different.
    If the candidate count reaches 0, the next number is chosen as candidate. Eventually,
    the candidate that survives pair-wise cancellation is the majority element.
    """
    major = 0
    count = 0

    for num in nums:
        if not count:
            major = num
            count = 1
        elif num == major:
            count += 1
        else:
            count -= 1

    return major
