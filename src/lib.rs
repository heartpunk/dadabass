#[test]
fn it_works() {
    let example_tree: BinaryTree<i32, i32> = BinaryTree::Branch {
        metadata: 0,
        value: 4,
        left: Some(Box::new(BinaryTree::Leaf {
            metadata: 0,
            value: 3
        })),
        right: Some(Box::new(BinaryTree::Leaf {
            metadata: 0,
            value: 5
        }))
    };
}

enum BinaryTree<V: Ord, M> {
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


impl <'a, V: Ord> AvlTree<'a, V> {
    fn insert(&mut self, new_value: V) {
        match self {
            &mut BinaryTree::Leaf {ref mut value, metadata: _} => {
                if new_value > *value {
                    *self = BinaryTree::Branch {
                        metadata: 1,
                        value: *value, // this should be reconsidered
                        left: Some(Box::new(BinaryTree::Leaf {
                            metadata: 0,
                            value: new_value // other thing that should be reconsidered
                        })),
                        right: None
                    }
                } else {
                    unreachable!()
                }
            },
            &mut BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} if new_value < *value => {
                match left {
                    &mut Some(ref mut inner_left) => inner_left.insert(new_value),
                    &mut None => ()
                }
            },
            &mut BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} if *value > new_value => {
                match right {
                    &mut Some(ref mut inner_right) => inner_right.insert(new_value),
                    &mut None => ()
                }
            },
            &mut BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} if *value == new_value => {
                () // this is a duplicate value, do nothing.
            },
            &mut BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} => unreachable!()
        }
    }
}
