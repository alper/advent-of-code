from dataclasses import dataclass
from typing import List, Set

sample_input = """px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"""

real_input = sample_input
real_input = open('input.txt').read()

workflows = {}
parts = []

@dataclass
class Part:
    x: int
    m: int
    a: int
    s: int

    def rating(self) -> int:
        return self.x + self.m + self.a + self.s

@dataclass
class Criterium:
    cat: str
    comparator: str
    val: int
    dest: str

    def accept(self, part: Part) -> bool:
        part_value = getattr(part, self.cat)

        if self.comparator == '<':
            return part_value < self.val
        elif self.comparator == '>':
            return part_value > self.val

        return False

    def inclusive_set(self) -> Set[int]:
        if self.comparator == '<':
            return set(range(1, self.val))
        elif self.comparator == '>':
            return set(range(self.val + 1, 4001))

        return set()

    def exclusive_set(self) -> Set[int]:
        return FULL_RANGE - self.inclusive_set()


@dataclass
class Workflow:
    name: str
    crits: List[Criterium]
    default: str

    def dest(self, part: Part) -> str:
        for crit in self.crits:
            if crit.accept(part):
                return crit.dest

        return self.default

    def is_dest(self, dest: str) -> bool:
        if self.default == dest:
            return True

        for crit in self.crits:
            if crit.dest == dest:
                return True
        return False

FULL_RANGE = set(range(1, 4001))

workflows = {}

for line in real_input.splitlines():
    if line:
        if not line[0] == '{':
            name, rest = line.split('{')
            selectors = rest[:-1].split(',')

            crits = []
            term = None
            for s in selectors:
                if ':' in s:
                    c = Criterium(
                        s[0],
                        s[1],
                        int(s[2:].split(':')[0]),
                        s[2:].split(':')[1]
                    )
                    crits.append(c)
                else:
                    term = s

            # print(name, crits, term)
            workflows[name] = Workflow(name, crits, term)
        else:
            part = {}
            ratings = line[1:-1].split(',')

            p = Part(int(ratings[0][2:]), int(ratings[1][2:]), int(ratings[2][2:]), int(ratings[3][2:]))

            parts.append(p)

total = 0

for part in parts:
    workflow = workflows['in']

    while workflow:
        dest = workflow.dest(part)

        if dest == 'R':
            print(f"Part {part} rejected")
            break
        if dest == 'A':
            print(f"Part {part} accepted")
            total += part.rating()
            break

        workflow = workflows[dest]

print("To be shipped:", total, "parts")

# Part 2

for workflow in workflows.values():
    if workflow.default == 'A':
        pos = workflow.name

        print('A')

        while True:
            print("Pos", pos)
            for workflow in workflows.values():
                if workflow.is_dest(pos):
                    pos = workflow.name
                    break

            if pos == 'in':
                print('in')
                break
        print()
    elif workflow.is_dest('A'):
        print('A')
        pos = workflow.name

        while True:
            print("Pos", pos)
            for workflow in workflows.values():
                if workflow.is_dest(pos):
                    pos = workflow.name
                    break

            if pos == 'in':
                print('in')
                break
        print()