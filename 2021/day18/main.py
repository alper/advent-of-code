i = "[[[[[9,8],1],2],3],4]"

b = eval(i)

print(b)


class TreeBranch:
    def __init__(self, left, right):
        self.left = left
        self.right = right
        self.parent = None

    def __unicode__(self):
        return f"""-- [{isinstance(self.left, TreeLeaf) and self.left.value or "/"}] --[{isinstance(self.right, TreeLeaf) and self.right.value or "|"}]"""

    def __eq__(self, o: object) -> bool:
        return self.left == o.left and self.right == o.right


class TreeLeaf:
    def __init__(self, value):
        self.value = value
        self.parent = None

    def __unicode__(self):
        return f"--- [{self.value}]"

    def __eq__(self, o: object) -> bool:
        isinstance(o, TreeLeaf) and self.value == o.value

    def __repr__(self):
        return f"TreeLeaf[{self.value}]"


def tree_from_list(l):
    if isinstance(l[0], list):
        left = tree_from_list(l[0])
    else:
        left = TreeLeaf(l[0])

    if isinstance(l[1], list):
        right = tree_from_list(l[1])
    else:
        right = TreeLeaf(l[1])

    t = TreeBranch(left, right)
    left.parent = t
    right.parent = t

    return t


t = tree_from_list(b)

print(t.left.left.left.left.left)


def explode(t):
    q = [t]

    for i in range(5):
        new_q = []

        for el in q:
            if isinstance(el, TreeBranch):
                new_q.append(el.left)
                new_q.append(el.right)

        print("New q", new_q)
        q = new_q

    # Do something with q[0].parent

    return TreeLeaf(1)


def test_explode():
    assert explode(
        tree_from_list(eval("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"))
    ) == tree_from_list(eval("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))

    assert explode(tree_from_list(eval("[[[[[9,8],1],2],3],4]"))) == tree_from_list(
        eval("[[[[0,9],2],3],4]")
    )
