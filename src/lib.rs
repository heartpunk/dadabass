#![feature(box_syntax, box_patterns)]
#![feature(rand)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]
extern crate quickcheck;
extern crate rand;

use std::iter::Map;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use quickcheck::Arbitrary;
use quickcheck::Gen;

#[test]
fn it_works() {
    let mut example_tree: BinaryTree<i32, (i8, i8)> = BinaryTree::Branch {
        metadata: (1, 1),
        value: 4,
        left: Some(Box::new(BinaryTree::Leaf {
            metadata: (0, 0),
            value: 3
        })),
        right: Some(Box::new(BinaryTree::Leaf {
            metadata: (0, 0),
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
fn ordering_property(bt: BinaryTree<i32, (i8, i8)>) -> bool {
    match bt {
        BinaryTree::Branch {metadata: _, value, left: Some(ref left), right: Some(ref right)} => {
            return left.iter().all(|t| value > t.value()) && right.iter().all(|t| value < t.value())
        },
        BinaryTree::Branch {metadata: _, value, left: None, right: Some(ref right)} => {
            return right.iter().all(|t| value < t.value())
        },
        BinaryTree::Branch {metadata: _, value, left: Some(ref left), right: None} => {
            return left.iter().all(|t| value > t.value())
        },
        _ => true
    }
}

#[quickcheck]
fn height_is_maintained(bt: AvlTree<i32>) -> bool {
    match bt {
        BinaryTree::Branch {
            metadata: (ref left_height, ref right_height), value,
            left: Some(box BinaryTree::Branch {metadata: (ref left_left_height, ref left_right_height), value: _, left: _, right: _}),
            right: Some(box BinaryTree::Branch {metadata: (ref right_left_height, ref right_right_height), value: _, left: _, right: _})}
        |
        BinaryTree::Branch {
            metadata: (ref left_height, ref right_height), value,
            left: Some(box BinaryTree::Branch {metadata: (ref left_left_height, ref left_right_height), value: _, left: _, right: _}),
            right: Some(box BinaryTree::Leaf {metadata: (ref right_left_height, ref right_right_height), value: _})}
        |
        BinaryTree::Branch {
            metadata: (ref left_height, ref right_height), value,
            left: Some(box BinaryTree::Leaf {metadata: (ref left_left_height, ref left_right_height), value: _}),
            right: Some(box BinaryTree::Branch {metadata: (ref right_left_height, ref right_right_height), value: _, left: _, right: _})}
        |
        BinaryTree::Branch {
            metadata: (ref left_height, ref right_height), value,
            left: Some(box BinaryTree::Leaf {metadata: (ref left_left_height, ref left_right_height), value: _}),
            right: Some(box BinaryTree::Leaf {metadata: (ref right_left_height, ref right_right_height), value: _})}
        => {
            *left_height == std::cmp::max(*left_left_height, *left_right_height) + 1 && *right_height == std::cmp::max(*right_left_height, *right_right_height) + 1
        },
        BinaryTree::Branch {
            metadata: (ref left_height, ref right_height), value,
            right: Some(box BinaryTree::Branch {metadata: (ref right_left_height, ref right_right_height), value: _, left: _, right: _}),
            left: None}
        |
        BinaryTree::Branch {
            metadata: (ref left_height, ref right_height), value,
            right: Some(box BinaryTree::Leaf {metadata: (ref right_left_height, ref right_right_height), value: _}),
            left: None}
        => {
            *right_height == std::cmp::max(*right_left_height, *right_right_height) + 1 && *left_height == 0
        },
        BinaryTree::Branch {
            metadata: (ref left_height, ref right_height), value,
            left: Some(box BinaryTree::Branch {metadata: (ref left_left_height, ref left_right_height), value: _, left: _, right: _}),
            right: None}
        |
        BinaryTree::Branch {
            metadata: (ref left_height, ref right_height), value,
            left: Some(box BinaryTree::Leaf {metadata: (ref left_left_height, ref left_right_height), value: _}),
            right: None}
        => {
            *left_height == std::cmp::max(*left_left_height, *left_right_height) + 1 && *right_height == 0
        },
        BinaryTree::Branch {metadata: (ref left_height, ref right_height), value: _, left: None, right: None} |
        BinaryTree::Leaf {metadata: (ref left_height, ref right_height), value: _} => {
            *left_height == 0 && *right_height == 0
        }
    }
}

#[derive(Debug,Clone)]
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


impl Arbitrary for BinaryTree<i32, (i8, i8)> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut tree = BinaryTree::Leaf {metadata: (0, 0), value: g.gen_range(-1000,1000)};
        while g.gen() {
            tree.insert(g.gen_range(-1000,1000));
        }
        tree
    }
}

impl <'a, V: Ord+Copy+Clone+Send, M: Copy+Clone+Send> BinaryTree<V, M> {
    fn iter(&'a self) -> BinaryTreeIterator<'a, V, M> {
        BinaryTreeIterator {to_visit: vec![&self]}
    }

    fn value(&'a self) -> V {
        match self {
            &BinaryTree::Branch {metadata: _, left: _, right: _, value: v} => v,
            &BinaryTree::Leaf {metadata: _, value: v} => v
        }
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

type AvlTree<'a, V: 'a> = BinaryTree<V, (i8, i8)>;


impl <'a, V: Ord+Copy> AvlTree<'a, V> {
    #[allow(non_shorthand_field_patterns)]
    fn insert(&mut self, new_value: V) -> i8 {
        match *self {
            BinaryTree::Leaf {value, metadata: _} => {
                if new_value > value {
                    *self = BinaryTree::Branch {
                        metadata: (0, 1),
                        value: value, // this should be reconsidered
                        right: Some(Box::new(BinaryTree::Leaf {
                            metadata: (0, 0),
                            value: new_value // other thing that should be reconsidered
                        })),
                        left: None
                    }
                } else if new_value == value {
                   return 0 // we don't allow duplicates.
                } else {
                    *self = BinaryTree::Branch {
                        metadata: (1, 0),
                        value: value, // this should be reconsidered
                        left: Some(Box::new(BinaryTree::Leaf {
                            metadata: (0, 0),
                            value: new_value // other thing that should be reconsidered
                        })),
                        right: None
                    }
                }
                1
            }
            BinaryTree::Branch {metadata: (ref mut left_height, right_height), ref mut value, left: Some(ref mut left ), right: _} if new_value < *value => {
                let incr = left.insert(new_value);
                *left_height += incr;
                assert!(incr < 2);
                if *left_height > right_height { incr } else { 0 }
            }
            BinaryTree::Branch {metadata: (ref mut left_height, right_height), ref mut value, ref mut left, right: ref right} if new_value < *value => {
                assert_eq!(0, *left_height);

                *left = Some(Box::new(BinaryTree::Leaf {value: new_value, metadata: (0, 0)}));
                *left_height += 1;
                if *left_height > right_height { 1 } else { 0 }
            }
            BinaryTree::Branch {metadata: (left_height, ref mut right_height), ref mut value, left: _, right: Some(ref mut right)} if new_value > *value => {
                let incr = right.insert(new_value);
                *right_height += incr;
                assert!(incr < 2);
                if *right_height > left_height { incr } else { 0 }
            }
            BinaryTree::Branch {metadata: (left_height, ref mut right_height), ref mut value, left: ref left, right: ref mut right} if new_value > *value => {
                assert_eq!(0, *right_height);

                *right = Some(Box::new(BinaryTree::Leaf {value: new_value, metadata: (0, 0)}));
                *right_height += 1;
                if *right_height > left_height { 1 } else { 0 }
            }
            BinaryTree::Branch {metadata: _, ref mut value, left: _, right: _} if *value == new_value => {
                0 // this is a duplicate value, do nothing.
            }
            BinaryTree::Branch {metadata: _, value: _, left: _, right: _} => unreachable!()
        }
    }
}
