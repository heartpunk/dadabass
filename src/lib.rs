#![feature(box_syntax, box_patterns)]
#![feature(rand)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]
extern crate quickcheck;
extern crate rand;

use quickcheck::Arbitrary;
use quickcheck::Gen;

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
    assert_eq!(8, example_tree.iter().count())
}

#[quickcheck]
fn ordering_property(bt: BinaryTree<i32, i32>) -> bool {
    true
}

#[derive(Clone,Debug)]
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

impl Arbitrary for BinaryTree<i32, i32> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let base_val: i32 = g.gen();
        let tree = BinaryTree::Leaf {metadata: 0, value: base_val};
        tree
    }
}

impl <'a, V: Ord+Copy+Clone+Send, M: Copy+Clone+Send> BinaryTree<V, M> {
    fn iter(&'a self) -> BinaryTreeIterator<'a, V, M> {
        BinaryTreeIterator {to_visit: vec![&self]}
    }
}

struct BinaryTreeIterator<'a, V: 'a+Ord+Copy+Clone+Send, M: 'a+Copy+Clone+Send> {
    to_visit: Vec<&'a BinaryTree<V, M>>
}

impl <'a, V: 'a+Ord+Copy+Clone+Send, M: 'a+Copy+Clone+Send> Iterator for BinaryTreeIterator<'a, V, M> {
    type Item = &'a BinaryTree<V, M>;

    fn next(&mut self) -> Option<&'a BinaryTree<V, M>> {
        let ret = self.to_visit.pop();
        match ret {
            Some(&BinaryTree::Branch {metadata: _, value: _, left: Some(ref left), right: None}) => {self.to_visit.push(left)},
            Some(&BinaryTree::Branch {metadata: _, value: _, left: None, right: Some(ref right)}) => {self.to_visit.push(right)},
            Some(&BinaryTree::Branch {metadata: _, value: _, left: Some(ref left), right: Some(ref right)}) => {
                self.to_visit.push(left);
                self.to_visit.push(right)
            },
            _ => ()
        }
        ret
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
                    *self = BinaryTree::Branch {
                        metadata: 1,
                        value: new_value, // this should be reconsidered
                        left: None,
                        right: Some(Box::new(BinaryTree::Leaf {
                            metadata: 0,
                            value: value // other thing that should be reconsidered
                        }))
                    }
                }
                ()
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, left: Some(ref mut left ), right: _} if new_value > *value => {
                left.insert(new_value)
            }
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, right: _} if new_value > *value => {
                *left = Some(Box::new(BinaryTree::Leaf {value: new_value, metadata: 0}))
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, left: _, right: Some(ref mut right)} if new_value < *value => {
                right.insert(new_value)
            }
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, left: _, right: ref mut right} if new_value < *value => {
                *right = Some(Box::new(BinaryTree::Leaf {value: new_value, metadata: 0}))
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} if *value == new_value => {
                () // this is a duplicate value, do nothing.
            },
            BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} => unreachable!()
        }
    }
}
