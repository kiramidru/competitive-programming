```
class Solution:
	def twoSum(self, nums: List[int], target: int) -> List[int]:
		value = dict()

		for i in range(0, len(nums)):
			if nums[i] in value:
				return [value[nums[i]], i]
			value[target - nums[i]] = i
```