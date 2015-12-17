import json
from random import randint
import functools

from hypothesis import given
import hypothesis.strategies as st

class AVLTree():
    def __init__(self, value=None):
        self._root = AVLTreeNode(self)
        if value is not None:
            self._root.insert(value)

        self.log = []

    def insert(self, value):
        try:
            self.root.insert(value)
        except:
            self.write_log()
            raise

    def write_log(self, file_name="tree_log.json"):
        with open("tree_log.json", "w") as f:
            f.write(json.dumps(self.log))

    def __str__(self):
        return str(self.root)

    @property
    def root(self):
        return self._root

    @root.setter
    def root(self, value):
        self._root = value
        self._root.parent = None

    def to_dict(self):
        return self._root.to_dict()

    def __iter__(self):
        return iter(self._root)


class AVLTreeNode():
    def empty_leaf(self):
        return AVLTreeNode(self.container)

    def __init__(self, container):
        self.value = None
        self._left = None
        self._right = None
        self.parent = None
        self.container = container
        self.left_height = self.right_height = 0

    @property
    def left(self):
        return self._left

    @property
    def right(self):
        return self._right

    def __str__(self, depth=0):
        def helper(child):
            return "\n" + ( child.__str__(depth + 1) if child is not None else "  " * (depth + 1) + "nothing to see here" )

        return ( "  " * depth + (
            "AVLTreeNode("
            "id = %s, "
            "value = %s, "
            "balance_factor = %s, "
            "left_height = %s, "
            "right_height = %s)"
        ) % (id(self),
             self.value,
             self.balance_factor,
             self.left_height,
             self.right_height
        ) ) + helper(self.left) + helper(self.right)


    @property
    def max_height(self) -> int:
        return max(self.left_height, self.right_height)

    @property
    def leaf(self):
        return self.left is None and self.right is None

    def become_branch(self):
        self._left = self.empty_leaf()
        self._right = self.empty_leaf()
        self.fix_height_metadata()

    @property
    def balance_factor(self) -> int:
        return self.left_height - self.right_height

    def update_log(self, operation, force=False):
        # if we aren't the root, don't log
        if not force and (self.container.root != self):
            return

        self.container.log.append([operation, self.to_dict()])

    def insert(self, value):
        force = self.container._root == self

        def post_insert():
            self.update_log("after inserting %i" % value, force=force)

        self.update_log("before inserting %i" % value)

        if self.leaf:
            self.value = value
            self.become_branch()
        elif value < self.value:
            self.left.insert(value)
        elif value > self.value:
            self.right.insert(value)
        else:
            # need to use a decorator for logging to avoid problems because of early return
            post_insert()
            return # ignore duplicates

        self.fix_height_metadata()
        self.balance()

        post_insert()

    # this could be __getitem__ (same for set_child and __setitem__)
    def child(self, side):
        assert side in ("left", "right")
        return getattr(self, "_" + side)

    def set_child(self, side, value):
        assert side in ("left", "right")
        setattr(self, "_%s" % side, value)
        child = self.child(side)
        child.parent = self
        setattr(self, "%s_height" % side, child.max_height + 1 if child else 0)
        self.fix_height_metadata()
        if self.parent:
            self.parent.fix_height_metadata()

    def height(self, side):
        assert side in ("left", "right")
        return getattr(self, "%s_height" % side)

    def balance(self):
        self.update_log("before balancing")

        # if this assertion fails, the tree is more imbalanced than it ever should be.
        assert(self.balance_factor in (-2,-1,0,1,2))

        side = None
        if self.balance_factor == 2:
            side = "left"
        if self.balance_factor == -2:
            side = "right"

        other_side = "left" if side == "right" else "right"

        if side:
            if (self.child(side).child(other_side) and not self.child(side).child(side)):
               self.child(side).rotate(side)

            elif (self.child(side).child(other_side) and self.child(side).child(side) and \
                # not sure why the following line matters, blindly ported from rust
                self.child(side).height(side) - self.child(side).height(other_side) < 0):

               self.child(side).rotate(side)

            self.rotate(other_side)

        self.fix_height_metadata()

        self.update_log("after balancing")

        if not self.balance_factor in (-1,0,1):
            print(self)
            raise ValueError("the tree is too imbalanced after we attempted to balance it. "
                   "all hope is lost.")

    def fix_height_metadata(self):
        self.left_height = self.left.max_height + 1
        self.right_height = self.right.max_height + 1

    def rotate(self, rotating_side):
        force = self.container._root == self
        assert(rotating_side in ("left", "right"))

        self.update_log("before rotate %s" % rotating_side, force=force)

        other_side = "left" if rotating_side == "right" else "right"

        old_parent = self.parent

        pivot = self.child(other_side)
        self.set_child(other_side, pivot.child(rotating_side))
        pivot.set_child(rotating_side, self)

        if old_parent is None:
            self.container.root = pivot
        elif old_parent.right == self:
            old_parent.set_child("right", pivot)
        elif old_parent.left == self:
            old_parent.set_child("left", pivot)
        else:
            raise ValueError("we are not a child of our own used-to-be parent. wat.")

        if self.parent is not None:
            self.parent.fix_height_metadata()

        self.update_log("after rotate %s" % rotating_side, force=force)

    def to_dict(self):
        def or_dict(node):
            return node.to_dict() if node and node.value is not None else {}
        return {
            "children": [or_dict(self.left), or_dict(self.right)],
            "left_height": self.left_height,
            "right_height": self.right_height,
            "value": self.value
        }

    def __iter__(self):
        return AVLTreeIterator(self)

class AVLTreeIterator():
    def __init__(self, root):
        self.to_visit = []
        self.current = root

    @property
    def current(self):
        return self._current

    @current.setter
    def current(self, node):
        if node:
            for side in ("left", "right"):
                child = node.child(side)
                if child and child.value:
                    self.to_visit.append(child)

        self._current = node if node and node.value else None

    def __next__(self):
        ret = self.current

        if self.to_visit:
            self.current = self.to_visit.pop()
        else:
            self.current = None

        if ret:
            return ret
        else:
            raise StopIteration()


def tree_from_values(values):
    tree = AVLTree()
    for value in values:
        tree.insert(value)
    return tree

@given(st.lists(st.integers(), max_size=100))
def test_height_is_maintained(values):
    tree = tree_from_values(values)

    def height_checker(tree):
        #@functools.lru_cache()
        def height_for_side(side):
           return height_checker(tree.child(side)) + 1 

        if tree.leaf:
            assert tree.left_height == tree.right_height == 0
            return 0
        else:
            assert tree.left_height == height_for_side("left")
            assert tree.right_height == height_for_side("right")

            return max(height_for_side(side) for side in ("left", "right"))

    height_checker(tree._root)

@given(st.lists(st.integers(), max_size=10))
def test_ordering_property_is_maintained(values):
    tree = tree_from_values(values)._root

    left = tree.child("left")
    right = tree.child("right")

    if left:
        all(sub_tree.value < tree.value for sub_tree in left)

    if right:
        all(sub_tree.value > tree.value for sub_tree in right)

@given(st.lists(st.integers(), max_size=10), st.integers())
def test_inserting_never_shrinks_the_tree(values, value):
    import copy

    def tree_size(tree):
        return sum(1 for _ in tree)

    tree = tree_from_values(values)._root

    if value in values:
        tree_size(tree) == tree_size(copy.deepcopy(tree).insert(value))
    else:
        tree_size(tree) + 1 == tree_size(copy.deepcopy(tree).insert(value))

if __name__ == "__main__":
    test_height_is_maintained()
    test_ordering_property_is_maintained()

#    import copy
#    try:
#        tree = AVLTree(1)
#        i = 0
#        while True:
#            #last_tree = copy.deepcopy(tree)
#            tree.insert(randint(0,10**4))
#            print(tree)
#            i += 1
#    except:
#        #print(tree)
#        import json
#        for i in (0,1):
#            with open("./avl%i.json" % i, 'w') as f:
#                f.write(json.dumps(( tree if i == 1 else last_tree ).to_dict()))
#        raise
