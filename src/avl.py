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
        self.leaf = True
        self._left = None
        self._right = None
        self.parent = None
        self.container = container
        self.left_height = self.right_height = 0

    @property
    def left(self):
        return self._left

    @left.setter
    def left(self, value):
        self._left = value
        self._left.parent = self
        self.left_height = self._left.max_height + 1 if self._left else 0

    @property
    def right(self):
        return self._right

    @right.setter
    def right(self, value):
        self._right = value
        self._right.parent = self
        self.right_height = self._right.max_height + 1 if self._right else 0

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

    def become_branch(self):
        self.left = self.empty_leaf()
        self.right = self.empty_leaf()
        self.leaf = False

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

    def balance(self):
        # if this assertion fails, the tree is more imbalanced than it ever should be.
        assert(self.balance_factor in (-2,-1,0,1,2))

        if self.balance_factor == 2:
            if (self.left.right and not self.left.left) or \
               (self.left.right and self.left.left and \
                # not sure why the following line matters, blindly ported from rust
                self.left.left_height - self.left.right_height < 0):
               self.left.rotate("left")
            self.rotate("right")
        if self.balance_factor == -2:
            if (self.right.left and not self.right.right) or \
               (self.right.left and self.right.right and \
                # not sure why the following line matters, blindly ported from rust
                self.right.right_height - self.right.left_height < 0):
               self.right.rotate("right")
            self.rotate("left")

        if not self.balance_factor in (-1,0,1):
            print(self)
            raise ValueError("the tree is too imbalanced after we attempted to balance it. "
                   "all hope is lost.")

    def rotate(self, direction):
        assert(direction in ("left", "right"))

        old_parent = self.parent

        if direction == "left":
            pivot = self.right
            self.right = pivot.left
            pivot.left = self
        elif direction == "right":
            pivot = self.left
            self.left = pivot.right
            pivot.right = self

        if old_parent is None:
            self.container.root = pivot

        # WHAT IS THIS VOODOO
        elif old_parent.right == self:
            old_parent.right = pivot
        elif old_parent.left == self:
            old_parent.left = pivot
        else:
            print(id(self), id(old_parent))
            raise

if __name__ == "__main__":
    tree = AVLTree(1)
    for i in range(10000):
        tree.insert(randint(0,10**12))
