import math
import collections

sample_input = """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"""

real_input = sample_input
real_input = open('input.txt').read()

cards = real_input.split('\n')

total_score = 0

extra_cards = {}

card_number = 1

number_of_cards_with_duplicates = 0

for card in cards:
    print("Card", card)
    colon = card.find(':')

    winners = [int(n.strip()) for n in card[colon+1:].split('|')[0].split()]
    numbers = [int(n.strip()) for n in card[colon+1:].split('|')[1].split()]

    hit_count = 0
    for n in numbers:
        if n in winners:
            hit_count += 1

    print("Hit count: ", hit_count)

    def score(c):
        if c == 0:
            return 0
        elif c == 1:
            return 1
        elif c > 1:
            return math.pow(2, c-1)

    total_score += score(hit_count)

    # Part 2 bookkeeping
    extra_copies = extra_cards.get(card_number, 0)
    number_of_current_card = extra_copies + 1

    number_of_cards_with_duplicates += number_of_current_card
    print("Instances of current card: ", number_of_current_card)

    # Process the number of current card times current hitcount for the following cards
    for j in range(1, hit_count+1):
        if card_number+j in extra_cards:
            extra_cards[card_number+j] += number_of_current_card
        else:
            extra_cards[card_number+j] = number_of_current_card

    print("Add ahead: ", extra_cards)
    print()

    card_number += 1

print("Total score: ", int(total_score))

print("Total number of cards: ", number_of_cards_with_duplicates)