from dataclasses import dataclass, field
import itertools
from functools import total_ordering
from typing import List

sample_input = """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"""

real_input = sample_input
real_input = open("input.txt").read()

cards_ordered = ["J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A"]
types_ordered = ["1HIGH_CARD", "2ONE_PAIR", "3TWO_PAIR", "4THREE_OF_A_KIND", "5FULL_HOUSE", "6FOUR_OF_A_KIND", "7FIVE_OF_A_KIND"]

@dataclass
@total_ordering
class Hand:
    raw: str
    bid: int
    typ: str = field(init=False)
    typ_with_joker: str = field(init=False)

    def __init__(self, raw, bid):
        self.raw = raw
        self.bid = bid

        self.typ = Hand.get_typ(self.raw)
        self.typ_with_joker = self.get_typ_with_joker(self.raw)

    @staticmethod
    def get_typ(i):
        counts = {els: len(list(grouper)) for els, grouper in itertools.groupby(sorted(list(i)))}

        frequencies = list(counts.values())

        if 5 in frequencies:
            return "7FIVE_OF_A_KIND"
        elif 4 in frequencies:
            return "6FOUR_OF_A_KIND"
        elif 3 in frequencies and 2 in frequencies:
            return "5FULL_HOUSE"
        elif 3 in frequencies and not 2 in frequencies:
            return "4THREE_OF_A_KIND"
        elif frequencies.count(2) == 2:
            return "3TWO_PAIR"
        elif 2 in frequencies and frequencies.count(1) == 3:
            return "2ONE_PAIR"
        elif frequencies.count(1) == 5:
            return "1HIGH_CARD"

    @staticmethod
    def get_typ_with_joker(i):
        print("With joker", i)

        joker_count = i.count('J')

        if joker_count == 0:
            return Hand.get_typ(i)
        else:
            non_jokers = [c for c in i if c != 'J']
            frequencies = [len(list(grouper)) for _, grouper in itertools.groupby(sorted(list(non_jokers)))]
            frequencies.sort()

            # 945 KJJ63 478 2ONE_PAIR 6FOUR_OF_A_KIND
            if joker_count == 1:
                if 4 in frequencies:
                    return "7FIVE_OF_A_KIND"
                elif 3 in frequencies:
                    return "6FOUR_OF_A_KIND"
                elif 2 in frequencies and 1 in frequencies:
                    return "4THREE_OF_A_KIND"
                elif frequencies.count(2) == 2:
                    return "5FULL_HOUSE"
                else:
                    return "2ONE_PAIR"
            elif joker_count == 2:
                if 3 in frequencies:
                    return "7FIVE_OF_A_KIND"
                elif 2 in frequencies:
                    return "6FOUR_OF_A_KIND"
                else:
                    return "4THREE_OF_A_KIND"
            elif joker_count == 3:
                if 2 in frequencies:
                    return "7FIVE_OF_A_KIND"
                else:
                    return "6FOUR_OF_A_KIND"
            elif joker_count == 4:
                return "7FIVE_OF_A_KIND"
            elif joker_count == 5:
                return "7FIVE_OF_A_KIND"


    def __lt__(self, other):
        if self.typ_with_joker == other.typ_with_joker:
            for i in range(len(self.raw)):
                if self.raw[i] == other.raw[i]:
                    continue
                else:
                    # Compare the cards
                    return cards_ordered.index(self.raw[i]) < cards_ordered.index(other.raw[i])
        else:
            # Compare the types
            return types_ordered.index(self.typ_with_joker) < types_ordered.index(other.typ_with_joker)

    def __eq__(self, other):
        return self.raw == other.raw


hands = []

for line in real_input.split("\n"):
    raw, bid = line.split()

    hand = Hand(raw, int(bid))

    hands.append(hand)

hands.sort()


total = 0
for i, h in enumerate(hands):
    print(f"{i+1} {h.raw} {h.bid} {h.typ} {h.typ_with_joker}")

    total += h.bid * (i+1)

print("Solution :", total)

