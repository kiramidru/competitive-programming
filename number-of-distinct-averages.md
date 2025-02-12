```
class Solution:
	def distinctAverages(self, nums: List[int]) -> int:
		nums.sort()
		unique = set()
		start = 0
		end = len(nums) - 1

		while start < end:
			unique.add((nums[start] + nums[end]) / 2)
			start += 1
			end -= 1
		return len(unique)
```