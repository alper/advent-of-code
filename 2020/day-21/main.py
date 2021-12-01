t = open("input.txt").readlines()
# t = open("test_input.txt").readlines()

allergens_to_ingredients = {}

all_ingredients = []

for f in t:
    ingredients = f.split("(")[0].strip().split(" ")
    all_ingredients.append(ingredients)
    allergens = f.split("(contains")[1].strip()[:-1].split(", ")

    print(ingredients)
    print(allergens)

    for allergen in allergens:
        if allergen not in allergens_to_ingredients:
            allergens_to_ingredients[allergen] = set(ingredients)
        else:
            allergens_to_ingredients[allergen] &= set(ingredients)

        print(allergens_to_ingredients)
        print("_____")

print("DONE")
print(allergens_to_ingredients)

allergenic_ingredients = set()
for allergenics in allergens_to_ingredients.values():
    allergenic_ingredients |= allergenics

print(allergenic_ingredients)

count = 0
for item in all_ingredients:
    for ingredient in item:
        if ingredient not in allergenic_ingredients:
            count += 1

print(count)

print("Part 2")
# Part 2
final_mapping = {}

while True:
    for (allergens, ingredients) in allergens_to_ingredients.items():
        if len(ingredients) == 1:
            ingredient = ingredients.pop()

            print(f"Found {allergens} is {ingredient}")
            final_mapping[allergens] = ingredient

            for other_ingredients in allergens_to_ingredients.values():
                if ingredient in other_ingredients:
                    other_ingredients.remove(ingredient)

            break

            # del allergens_to_ingredients[a]
    print(allergens_to_ingredients)
    if not any(allergens_to_ingredients.values()):
        break

print("Final mapping:", final_mapping)

from operator import itemgetter

print(
    "Answerp part 2",
    ",".join(
        [
            pair[1]
            for pair in sorted(list(final_mapping.items()), key=itemgetter(0))
        ]
    ),
)
