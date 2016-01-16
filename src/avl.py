import json

from hypothesis import given
import hypothesis.strategies as st



class AVLTree():
    """The interface to an AVL tree.

    The tree itself is actually represented by AVLTreeNode instances, but because of
    needing to be able to change which node is the root, we need a container class.

    Args:
      value: an initial value to insert into the tree.
    """

    def __init__(self, value=None):
        self._root = AVLTreeNode(self)
        if value is not None:
            self._root.insert(value)

        self.log = []

    def insert(self, value):
        """Inserts the vallue into the tree.

        Args:
          value: the value to insert into the tree
        """

        try:
            self.root.insert(value)
            self.write_log()
        except:
            self.write_log()
            raise

    def __del__(self):
        self.write_log()

    def write_log(self, file_name="tree_log.json"):
        """Writes a log of operation description and tree pairs in JSON for visualization."""

        with open(file_name, "w") as f:
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
        """Returns a dictionary representation of this tree."""

        return self._root.to_dict()

    def __iter__(self):
        return iter(self._root)


class AVLTreeNode():
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
            """Simplifies printing children."""

            return "\n" + (child.__str__(depth + 1)
                           if child is not None
                           else "  " * (depth + 1) + "None")

        return ("  " * depth + (
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
            )) + helper(self.left) + helper(self.right)

    @property
    def max_height(self) -> int:
        """Returns the height of the maximal depth subtree."""

        return max(self.left_height, self.right_height)

    @property
    def leaf(self) -> bool:
        """Returns whether this node is a leaf or not."""

        return self.left is None and self.right is None

    def become_branch(self):
        """Converts this node from a leaf to a branch."""

        self._left = AVLTreeNode(self.container)
        self._right = AVLTreeNode(self.container)
        self._left.parent = self._right.parent = self
        self.fix_height_metadata()

    @property
    def balance_factor(self) -> int:
        """This represents how, and in which direction, this node is imbalanced."""

        return self.left_height - self.right_height

    def update_log(self, operation, force=False):
        """Updates the log of operation and tree pairs for later visualization.

        The log is only ever updated at the root, to avoid visualizing multiple layers
        in one stream, which would be very confusing.

        Args:
          operation: A textual description of the operation taking place, intended
            for human consumption.
          force: Defaults to false. If this is true, we will append to the log regardless
            of whether or not we are the root. This is used during operations where the
            root has just changed, but the whole tree should still be visualized.
        """

        # if we aren't the root, don't log
        if not force and (self.container.root != self):
            return

        self.container.log.append([operation, self.container.root.to_dict()])

    def insert(self, value):
        """Inserts the given value into the appropriate spot in the tree.

        Args:
          value: the value to insert.
        """

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
        """Returns the child on side (where side is "right" or "left").

        Args:
          side: One of "right" or "left". The side of the child we want to fetch."""

        assert side in ("left", "right")
        return getattr(self, "_" + side)

    def set_child(self, side, new_node):
        """Sets the child on side (where side is "right" or "left").

        Args:
          side: One of "right" or "left". The side of the child we want to set.
          new_node: The node to set the child on side to.
        """

        assert side in ("left", "right")
        setattr(self, "_%s" % side, new_node)
        child = self.child(side)
        child.parent = self
        setattr(self, "%s_height" % side, child.max_height + 1 if child else 0)
        self.fix_height_metadata()
        if self.parent:
            self.parent.fix_height_metadata()

    def height(self, side):
        """Returns the height of the child on side.

        Args:
          side: Which side the child we want the height of.
        """

        assert side in ("left", "right")
        return getattr(self, "%s_height" % side)

    def balance(self):
        """Balances the tree if it is imbalanced."""

        def values():
            return set(node.value for node in self.container)

        starting_values = values()

        self.update_log("before balancing")

        # if this assertion fails, the tree is more imbalanced than it ever should be.
        assert self.balance_factor in (-2, -1, 0, 1, 2)

        side = None
        if self.balance_factor == 2:
            side = "left"
        if self.balance_factor == -2:
            side = "right"

        other_side = "left" if side == "right" else "right"

        # this if is suspicious...
        if side:
            if self.child(side).child(other_side).value is not None \
               and self.child(side).child(side).value is None:
                assert starting_values == values()
                self.child(side).rotate(side)
                assert starting_values == values()

            elif (self.child(side).child(other_side).value is not None
                  and self.child(side).child(side).value is not None and
                  # not sure why the following line matters, blindly portedofrom rust
                  self.child(side).height(side) - self.child(side).height(other_side) < 0):

                assert starting_values == values()
                self.update_log("before rotate outside")
                self.child(side).rotate(side)
                self.update_log("after rotate outside", force=True)
                assert starting_values == values()

            #assert starting_values == values()
            self.rotate(other_side)
            #assert starting_values == values()

        self.fix_height_metadata()
        #assert starting_values == values()
        self.update_log("after balancing")

        assert self.balance_factor in (-1, 0, 1)

    def fix_height_metadata(self):
        """Locally adjust the height metadata based on the height metadata of our children.

        It is crucial that this code only be called when the children have sane height
        metadata. If not, the whole tree will almost certainly break.
        """

        self.left_height = self.left.max_height + 1
        self.right_height = self.right.max_height + 1

    def rotate(self, rotating_side):
        """Rotate the tree around this node, to the direction specified by rotating_side.

        Args:
          rotating_side: which direction to rotate the tree.
        """

        def values():
            return set(node.value for node in self.container)

        starting_values = values()

        force = True
        assert rotating_side in ("left", "right")

        self.update_log("before rotate %s" % rotating_side, force=force)

        other_side = "left" if rotating_side == "right" else "right"

        old_parent = self.parent
        import inspect

        def lineno():
            """Returns the current line number in our program."""
            return inspect.currentframe().f_back.f_lineno

        self.update_log("at line %s" % lineno(), force=force)
        pivot = self.child(other_side)
        print(pivot)
        assert pivot.value is not None
        self.update_log("at line %s" % lineno(), force=force)
        print(*(id(thing) for thing in (self.child(other_side), pivot, pivot.child(rotating_side))))
        self.set_child(other_side, pivot.child(rotating_side))
        print(*(id(thing) for thing in (self.child(other_side), pivot, pivot.child(rotating_side))))
        self.update_log("at line %s" % lineno(), force=force)
        pivot.set_child(rotating_side, self)
        self.update_log("at line %s" % lineno(), force=force)

        if old_parent is None:
            print("old_parent is None")
            self.container.root = pivot
        elif old_parent.right == self:
            print("we're the right child")
            self.update_log("at line %s" % lineno(), force=force)
            old_parent.set_child("right", pivot)
        elif old_parent.left == self:
            print("we're the left child")
            self.update_log("at line %s" % lineno(), force=force)
            old_parent.set_child("left", pivot)
        else:
            raise ValueError("we are not a child of our own used-to-be parent. wat.")

        if self.parent is not None:
            self.parent.fix_height_metadata()

        self.update_log("after rotate %s" % rotating_side, force=force)

    def to_dict(self):
        """Returns a representation of this node as a dict."""

        def or_dict(node):
            """Helper function that returns the dict-ified child, or an empty dict."""

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
    """Simple depth first iterator for AVLTree."""

    def __init__(self, root):
        self.to_visit = []
        self.current = root

    @property
    def current(self):
        return self._current

    def __iter__(self):
        return self

    @current.setter
    def current(self, node):
        if node:
            for side in ("left", "right"):
                child = node.child(side)
                if child and child.value is not None:
                    self.to_visit.append(child)

        self._current = node if node and node.value is not None else None

    def __next__(self):
        ret = self.current

        if self.to_visit:
            self.current = self.to_visit.pop()
        else:
            self.current = None

        if ret is not None:
            return ret
        else:
            raise StopIteration()


def tree_from_values(values):
    tree = AVLTree()
    for value in values:
        tree.insert(value)
    return tree

def after_each_insert(values):
    tree = AVLTree()
    for value in values:
        tree.insert(value)
        yield tree

@given(st.lists(st.integers(), max_size=10))
def test_height_is_maintained(values):
    tree = tree_from_values(values)

    def height_checker(tree):
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

    tree = tree_from_values(values)

    new_tree = copy.deepcopy(tree)
    new_tree.insert(value)

    if value in values:
        assert tree_size(tree) == tree_size(new_tree)
    else:
        print(tree_size(tree), tree_size(new_tree))
        assert tree_size(tree) + 1 == tree_size(new_tree)

@given(st.lists(st.integers(), max_size=10))
def test_all_nodes_are_either_children_or_roots(values):
    for tree in after_each_insert(values):
        for node in iter(tree):
            assert (node == tree._root) or (node.parent)



if __name__ == "__main__":
    test_inserting_never_shrinks_the_tree()
    test_height_is_maintained()
    test_ordering_property_is_maintained()
    test_all_nodes_are_either_children_or_roots()
