from random import randint

class AVLTree():
    def __init__(self, value=None):
        self._root = AVLTreeNode(self)
        if value is not None:
            self._root.insert(value)

    def insert(self, value):
        self.root.insert(value)

    def __str__(self):
        return str(self.root)

    @property
    def root(self):
        return self._root

    @root.setter
    def root(self, value):
        self._root = value
        self._root.parent = None


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
        return max(self.left_height, self.right_height)

    @property
    def leaf(self):
        return self.left is None and self.right is None

    def become_branch(self):
        self._left = self.empty_leaf()
        self._right = self.empty_leaf()

    @property
    def balance_factor(self) -> int:
        return self.left_height - self.right_height

    def insert(self, value):
        if self.leaf:
            self.value = value
            self.become_branch()
        elif value > self.value:
            self.left.insert(value)
        elif value < self.value:
            self.right.insert(value)
        else:
            return # ignore duplicates

        self.balance()

    # this could be __getitem__ (same for set_child and __setitem__)
    def child(self, side):
        assert side in ("left", "right")
        return getattr(self, side)

    def set_child(self, side, value):
        assert side in ("left", "right")
        setattr(self, "_%s" % side, value)
        child = self.child(side)
        child.parent = self
        setattr(self, "%s_height" % side, child.max_height + 1 if child else 0)

    def height(self, side):
        assert side in ("left", "right")
        return getattr(self, "%s_height" % side)

    def balance(self):
        # if this assertion fails, the tree is more imbalanced than it ever should be.
        assert self.balance_factor in (-2, -1, 0, 1, 2)

        side = None
        if self.balance_factor == 2:
            side = "left"
            other_side = "right"
        if self.balance_factor == -2:
            side = "right"
            other_side = "left"

        if side:
            if (self.child(side).child(other_side) and not self.child(side).child(side)) or \
               (self.child(side).child(other_side) and self.child(side).child(side) and \
                # not sure why the following line matters, blindly ported from rust
                self.child(side).height(side) - self.child(side).height(other_side) < 0):
               self.child(side).rotate(side)
            self.rotate(other_side)

        if not self.balance_factor in (-1,0,1):
            print(self)
            raise ValueError("the tree is too imbalanced after we attempted to balance it. "
                   "all hope is lost.")

    def rotate(self, rotating_side):
        assert(rotating_side in ("left", "right"))

        other_side = "left" if rotating_side == "right" else "right"

        old_parent = self.parent

        pivot = self.child(rotating_side)
        self.set_child(other_side, pivot.child(rotating_side))
        pivot.set_child(rotating_side, self)

        if old_parent is None:
            self.container.root = pivot
        elif old_parent.right == self:
            old_parent.right = pivot
        elif old_parent.left == self:
            old_parent.left = pivot
        else:
            raise ValueError("we are not a child of our own used-to-be parent. wat.")

if __name__ == "__main__":
    tree = AVLTree(1)
    for i in range(10000):
        tree.insert(randint(0,10**12))
