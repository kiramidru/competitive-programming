```
class Solution:
	def threeSum(self, nums: List[int]) -> List[List[int]]:
		nums.sort()
		result = []

		for i in range(len(nums)):
			if i > 0 and nums[i] == nums[i-1]:
				continue
			start = i + 1
			end = len(nums) - 1

			while start < end:
				value = nums[i] + nums[start] + nums[end]
				if value > 0:
					end -= 1
				elif value < 0:
					start += 1
				else:
					result.append([nums[i], nums[start], nums[end]])
					start += 1
					end -=1

					while nums[start] == nums[start-1] and start < end:
						start += 1

	return result
```
