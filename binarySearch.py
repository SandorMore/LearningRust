nums = [1,2,3,4,5,6]
nums.append(7)

L,R = 0, len(nums) - 1

while L < R:
    print(f"Nums L = {nums[L]}, nums R = {nums[R]}")
    L += 1
    R -= 1
if L == R:
    print(f"Middle element: {nums[L]}")