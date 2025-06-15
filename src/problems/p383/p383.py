from collections import defaultdict

def canConstruct(self, ransomNote: str, magazine: str) -> bool:
    if len(ransomNote) > len(magazine):
        return False

    availables = [0] * 26
    for c in magazine:
        availables[ord(c) - ord('a')] += 1
    
    for c in ransomNote:
        index = ord(c) - ord('a')
        if not availables[index]:
            return False
        else:
            availables[index] -= 1
    return True
