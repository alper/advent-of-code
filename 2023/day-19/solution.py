from dataclasses import dataclass
from typing import List, Set
from copy import deepcopy

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

    def __str__(self):
        return f"{self.cat}{self.comparator}{self.val}:{self.dest}"

@dataclass
class Workflow:
    name: str
    crits: List[Criterium]
    default: str

    def dest(self, part: Part) -> str:
        """The destination in this workflow where a part goes."""
        for crit in self.crits:
            if crit.accept(part):
                return crit.dest

        return self.default

    def has_dest(self, dest: str) -> List[int]:
        """List the indexes of the criteria that have a destination of this name."""
        out = []

        for i, crit in enumerate(self.crits):
            if crit.dest == dest:
                out.append(i)
        
        if self.default == dest:
            out.append(-1)
        
        return out
    
    def __str__(self):
        cr = ','.join([str(c) for c in self.crits])
        return f"Workflow {self.name}: {cr} ({self.default})"

FULL_RANGE = set(range(1, 4001))

workflows = {}

def get_path_to(name: str, workflows) -> List[str]:
    out = [name]

    while True:
        for workflow in workflows.values():
            if workflow.has_dest(name):
                name = workflow.name
                out.append(name)
                
                break

        if name == 'in':
            break
    return out

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
print()

# Part 2

possibilities = 0

for workflow in workflows.values():
    print(workflow)
    a_dest = workflow.has_dest('A')

    for ad in a_dest:
        accepted_ranges = {a: deepcopy(FULL_RANGE) for a in ['x', 'm', 'a', 's']}
        if ad == -1:
            # print("Tracing terminal", ad)
            positive = []
            negative = workflow.crits
        else:
            # print("Tracing criteria", ad)
            positive = [workflow.crits[ad]]
            negative = workflow.crits[:ad]

        for crit in positive:
            accepted_ranges[crit.cat] &= crit.inclusive_set()
        for crit in negative:
            accepted_ranges[crit.cat] -= crit.inclusive_set()

        pos = workflow.name

        while True:
            for workflow_search in workflows.values():
                w_dest = workflow_search.has_dest(pos)
                if w_dest:
                    pos = workflow_search.name

                    if w_dest[0] == -1:
                        positive = []
                        negative = workflow_search.crits
                    else:
                        positive = [workflow_search.crits[w_dest[0]]]
                        negative = workflow_search.crits[:w_dest[0]]
                    
                    for crit in positive:
                        accepted_ranges[crit.cat] &= crit.inclusive_set()
                    for crit in negative:
                        accepted_ranges[crit.cat] -= crit.inclusive_set()
                    
                    break

            if pos == 'in':
                break
        print(f"FINAL for {workflow.name}", [(a, min(accepted_ranges[a]), max(accepted_ranges[a])) for a in ['x', 'm', 'a', 's']])

        possibilities += len(accepted_ranges['x']) * len(accepted_ranges['m']) * len(accepted_ranges['a']) * len(accepted_ranges['s'])

print("Possible parts:", possibilities)
