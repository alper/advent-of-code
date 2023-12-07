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

cards_ordered = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"]
types_ordered = ["FIVE_OF_A_KIND", "FOUR_OF_A_KIND", "FULL_HOUSE", "THREE_OF_A_KIND", "TWO_PAIR", "ONE_PAIR", "HIGH_CARD"]

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
            return "FIVE_OF_A_KIND"
        elif 4 in frequencies:
            return "FOUR_OF_A_KIND"
        elif 3 in frequencies and 2 in frequencies:
            return "FULL_HOUSE"
        elif 3 in frequencies and not 2 in frequencies:
            return "THREE_OF_A_KIND"
        elif frequencies.count(2) == 2:
            return "TWO_PAIR"
        elif 2 in frequencies and frequencies.count(1) == 3:
            return "ONE_PAIR"
        elif frequencies.count(1) == 5:
            return "HIGH_CARD"

    @staticmethod
    def get_typ_with_joker(i):
        return "HIGH_CARD"

    def __lt__(self, other):
        if self.typ == other.typ:
            for i in range(len(self.raw)):
                if self.raw[i] == other.raw[i]:
                    continue
                else:
                    # Compare the cards
                    return cards_ordered.index(self.raw[i]) < cards_ordered.index(other.raw[i])
        else:
            # Compare the types
            return types_ordered.index(self.typ) < types_ordered.index(other.typ)

        return self.typ < other.typ

    def __eq__(self, other):
        return self.raw == other.raw


hands = []

for line in real_input.split("\n"):
    raw, bid = line.split()

    hand = Hand(raw, int(bid))

    hands.append(hand)

hands.sort()

print(hands)

total = 0
for i, h in enumerate(hands):
    print("Rank:", len(hands)-i)
    print("Bid:", h.bid, h.raw)

    total += h.bid * (len(hands)-i)

print("Solution:", total)
