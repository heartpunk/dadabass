#[derive(Debug)]
enum BinaryTree<V: Ord+Copy, M> {
    Branch {
        metadata: M,
        value: V,
        left: Option<Box<BinaryTree<V, M>>>,
        right: Option<Box<BinaryTree<V, M>>>
    },
    Leaf {
        value: V,
        metadata: M
    }
}

type AvlTree<'a, V: 'a> = BinaryTree<V, i32>;

impl <'a, V: Ord+Copy> AvlTree<'a, V> {
    fn insert(&mut self, new_value: V) {
        match *self {
            BinaryTree::Leaf {value, metadata: _} => {
                if new_value > value {
                    *self = BinaryTree::Branch {
                        metadata: 1,
                        value: value,
                        right: Some(Box::new(BinaryTree::Leaf {
                            metadata: 0,
                            value: new_value
                        })),
                        left: None
                    }
                } else if new_value == value {
                    // we don't allow duplicates.
                } else {
                    *self = BinaryTree::Branch {
                        metadata: 1,
                        value: value,
                        left: Some(Box::new(BinaryTree::Leaf {
                            metadata: 0,
                            value: new_value
                        })),
                        right: None
                    }
                }
                ()
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, left: Some(ref mut left ), right: _} if new_value < *value => {
                left.insert(new_value)
            }
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, right: _} if new_value < *value => {
                *left = Some(Box::new(BinaryTree::Leaf {value: new_value, metadata: 0}))
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, left: _, right: Some(ref mut right)} if new_value > *value => {
                right.insert(new_value)
            }
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, left: _, right: ref mut right} if new_value > *value => {
                *right = Some(Box::new(BinaryTree::Leaf {value: new_value, metadata: 0}))
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} if *value == new_value => {
                () // this is a duplicate value, do nothing.
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} => unreachable!()
        }
    }
}
