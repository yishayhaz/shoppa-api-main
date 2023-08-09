import string, secrets, random


def generate_string(length):
    alphabet = string.ascii_letters + string.digits

    punctuation_amount = length // 4

    valid_punctuation = "#$%&*+,-./:;<=>?@[]^_{|}~"

    length -= punctuation_amount

    string_ = [secrets.choice(alphabet) for _ in range(length)] + [
        secrets.choice(valid_punctuation) for _ in range(punctuation_amount)
    ]

    random.shuffle(string_)
    
    return "".join(string_)


print(generate_string(32))

