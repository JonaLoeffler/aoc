# Read the input string
with open('input.txt') as f:
    s = f.read().strip()

# Keep track of the last four characters
last_four = []

# Iterate over the characters in the string
for i, c in enumerate(s):
    # Append the current character to the last_four list
    last_four.append(c)

    # If the length of the set of the last four characters is four,
    # then we have found the first start-of-packet marker
    if len(set(last_four)) == 4:
        # Print the result
        print(i + 1)
        break

    # If the length of the last_four list is more than four, remove
    # the first element from the list
    if len(last_four) >= 4:
        last_four.pop(0)

# Keep track of the last 14 characters
last_fourteen = []

# Iterate over the characters in the string
for i, c in enumerate(s):
    # Append the current character to the last_fourteen list
    last_fourteen.append(c)

    # If the length of the set of the last 14 characters is 14,
    # then we have found the first start-of-message marker
    if len(set(last_fourteen)) == 14:
        # Print the result
        print(i + 1)
        break

    # If the length of the last_fourteen list is more than 14, remove
    # the first element from the list
    if len(last_fourteen) >= 14:
        last_fourteen.pop(0)
