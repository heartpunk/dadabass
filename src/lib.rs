#![feature(box_syntax, box_patterns)]

#[test]
fn it_works() {
    let mut example_tree: BinaryTree<i32, i32> = BinaryTree::Branch {
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

    println!("{:?}", example_tree);

    example_tree.insert(7);
    println!("{:?}", example_tree);
    example_tree.insert(2);
    println!("{:?}", example_tree);
    example_tree.insert(700);
    println!("{:?}", example_tree);
    example_tree.insert(-200);
    println!("{:?}", example_tree);
    example_tree.insert(42);
    println!("{:?}", example_tree);
}

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
                        value: value, // this should be reconsidered
                        left: Some(Box::new(BinaryTree::Leaf {
                            metadata: 0,
                            value: new_value // other thing that should be reconsidered
                        })),
                        right: None
                    }
                } else {
                    unreachable!("wrong side of else")
                }
                ()
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} if new_value < *value => {
                match left {
                    &mut Some(ref mut inner_left) => inner_left.insert(new_value),
                    &mut None => ()
                }
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} if *value > new_value => {
                match right {
                    &mut Some(ref mut inner_right) => inner_right.insert(new_value),
                    &mut None => ()
                }
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} if *value == new_value => {
                () // this is a duplicate value, do nothing.
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} => unreachable!()
        }
    }
}
