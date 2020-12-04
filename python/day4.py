import re

with open("inputs/4.txt") as file:
    data = file.read().strip()

passports = [passport.replace("\n", " ") for passport in data.split("\n\n")]
required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]  # cid


def valid_1(passport: str) -> bool:
    return all(field in passport for field in required)


def valid_2(passport: str) -> bool:
    if not valid_1(passport):
        return False

    p = {field.split(":")[0]: field.split(":")[1] for field in passport.split(" ")}

    if not 1920 <= int(p["byr"]) <= 2002:
        return False

    if not 2010 <= int(p["iyr"]) <= 2020:
        return False

    if not 2020 <= int(p["eyr"]) <= 2030:
        return False

    height = int(p["hgt"][:-2])
    height_units = p["hgt"][-2:]
    if height_units == "cm":
        if not 150 <= height <= 193:
            return False
    elif height_units == "in":
        if not 59 <= height <= 76:
            return False
    else:
        return False

    if re.match(r"#[0-9a-f]{6}", p["hcl"]) is None:
        return False

    if p["ecl"] not in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
        return False

    if len(p["pid"]) != 9 or not p["pid"].isnumeric():
        return False

    return True


print(sum(valid_1(passport) for passport in passports))
print(sum(valid_2(passport) for passport in passports))
