from typing import Set, Dict, List

with open("../inputs/21.txt") as file:
    data = file.read().strip()

# data = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)"

allergy_causes: Dict[str, Set[str]] = {}
all_ingredients: List[str] = []

for food in data.splitlines():
    ingredients, allergens = food.split("(contains ")
    ingredients = ingredients.split()
    all_ingredients.extend(ingredients)
    for allergen in allergens[:-1].split(", "):
        if allergen in allergy_causes:
            allergy_causes[allergen].intersection_update(ingredients)
        else:
            allergy_causes[allergen] = set(ingredients)

allergic_foods = set(ingredient for food in allergy_causes.values() for ingredient in food)

part_1 = sum(1 for ingredient in all_ingredients if ingredient not in allergic_foods)
print(part_1)

while any(len(x) > 1 for x in allergy_causes.values()):
    for allergen, possible_causes in allergy_causes.items():
        if len(possible_causes) == 1:
            [other_causes.difference_update(possible_causes) for other_allergen, other_causes in allergy_causes.items()
             if other_allergen != allergen]

print(','.join(food.pop() for (_, food) in sorted(allergy_causes.items(), key=lambda x: x[0])))
