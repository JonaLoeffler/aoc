with open('input.txt') as f:
  lines = f.readlines()

max_calories = 0
current_calories = 0
for line in lines:
  line = line.strip()
  if line == '':
    max_calories = max(max_calories, current_calories)
    current_calories = 0
  else:
    current_calories += int(line)

max_calories = max(max_calories, current_calories)

print(max_calories)

calorie_counts = []
current_calories = 0
for line in lines:
  line = line.strip()
  if line == '':
    calorie_counts.append(current_calories)
    current_calories = 0
  else:
    current_calories += int(line)

calorie_counts.append(current_calories)

calorie_counts.sort(reverse=True)

print(sum(calorie_counts[:3]))
