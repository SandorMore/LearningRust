list: list = [1, 2, 3, 4, 6, 7]
target: int = 10

left, right = 0, len(list) - 1
found = False

while left <= right:
    mid = (left + right) // 2
    if list[mid] == target:
        found = True
        break
    elif list[mid] < target:
        left = mid + 1
    else:
        right = mid - 1

print("Found!" if found else "Not found.")
