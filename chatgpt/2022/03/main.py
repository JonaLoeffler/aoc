f = open("input.txt", "r")
lines = f.readlines()
elf_calories = {}
for line in lines:
    items = line.split("\n")
    for item in items:
        if item == "":
            continue
        if item[0] not in elf_calories:
            elf_calories[item[0]] = 0
        elf_calories[item[0]] += int(item[1:])
max_calories = 0
max_elf = ""
for elf in elf_calories:
    if elf_calories[elf] > max_calories:
        max_calories = elf_calories[elf]
        max_elf = elf
print(f"Elf {max_elf} has the maximum calories: {max_calories}")
