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
    let mut example_tree: BinaryTree<i32, (i8, i8)> = BinaryTree {
        metadata: (1, 1),
        value: 4,
        left: Some(Box::new(BinaryTree {
            metadata: (0, 0),
            value: 3,
            left: None,
            right: None
        })),
        right: Some(Box::new(BinaryTree {
            metadata: (0, 0),
            value: 5,
            left: None,
            right: None
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
        BinaryTree {metadata: _, value, left: Some(ref left), right: Some(ref right)} => {
            return left.iter().all(|t| value > t.value) && right.iter().all(|t| value < t.value)
        },
        BinaryTree {metadata: _, value, left: None, right: Some(ref right)} => {
            return right.iter().all(|t| value < t.value)
        },
        BinaryTree {metadata: _, value, left: Some(ref left), right: None} => {
            return left.iter().all(|t| value > t.value)
        },
        _ => true
    }
}

#[quickcheck]
fn height_is_maintained(bt: AvlTree<i32>) -> bool {
    match bt {
        BinaryTree {
            metadata: (ref left_height, ref right_height), value,
            left: Some(box BinaryTree {metadata: (ref left_left_height, ref left_right_height), value: _, left: _, right: _}),
            right: Some(box BinaryTree {metadata: (ref right_left_height, ref right_right_height), value: _, left: _, right: _})}
        => {
            *left_height == std::cmp::max(*left_left_height, *left_right_height) + 1 && *right_height == std::cmp::max(*right_left_height, *right_right_height) + 1
        },
        BinaryTree {
            metadata: (ref left_height, ref right_height), value,
            right: Some(box BinaryTree {metadata: (ref right_left_height, ref right_right_height), value: _, left: _, right: _}),
            left: None}
        => {
            *right_height == std::cmp::max(*right_left_height, *right_right_height) + 1 && *left_height == 0
        },
        BinaryTree {
            metadata: (ref left_height, ref right_height), value,
            left: Some(box BinaryTree {metadata: (ref left_left_height, ref left_right_height), value: _, left: _, right: _}),
            right: None}
        => {
            *left_height == std::cmp::max(*left_left_height, *left_right_height) + 1 && *right_height == 0
        },
        BinaryTree {metadata: (ref left_height, ref right_height), value: _, left: None, right: None} => {
            *left_height == 0 && *right_height == 0
        }
    }
}

#[quickcheck]
fn balance_property(bt: BinaryTree<i32, (i8, i8)>) -> bool {
    match bt {
        BinaryTree {metadata, value: _, left: _, right: _} => ( metadata.0 - metadata.1 ) <= 1 && ( metadata.0 - metadata.1 ) >= -1
    }
}

#[derive(Debug,Clone)]
struct BinaryTree<V: Ord+Copy, M> {
        metadata: M,
        value: V,
        left: Option<Box<BinaryTree<V, M>>>,
        right: Option<Box<BinaryTree<V, M>>>
}


impl Arbitrary for BinaryTree<i32, (i8, i8)> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut tree = BinaryTree {metadata: (0, 0), value: g.gen_range(-1000,1000), left: None, right: None};
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
}

struct BinaryTreeIterator<'a, V: 'a+Ord+Copy+Clone+Send, M: 'a+Copy+Clone+Send> {
    to_visit: Vec<&'a BinaryTree<V, M>>
}

impl <'a, V: 'a+Ord+Copy+Clone+Send, M: 'a+Copy+Clone+Send> Iterator for BinaryTreeIterator<'a, V, M> {
    type Item = &'a BinaryTree<V, M>;

    fn next(&mut self) -> Option<&'a BinaryTree<V, M>> {
        let ret = self.to_visit.pop();
        match ret {
            Some(&BinaryTree {metadata: _, value: _, left: Some(ref left), right: None}) => {self.to_visit.push(left)},
            Some(&BinaryTree {metadata: _, value: _, left: None, right: Some(ref right)}) => {self.to_visit.push(right)},
            Some(&BinaryTree {metadata: _, value: _, left: Some(ref left), right: Some(ref right)}) => {
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
        let ret = match *self {
            BinaryTree {value, metadata: _, left: None, right: None} => {
                if new_value > value {
                    *self = BinaryTree {
                        metadata: (0, 1),
                        value: value,
                        right: Some(Box::new(BinaryTree {
                            metadata: (0, 0),
                            value: new_value,
                            left: None,
                            right: None
                        })),
                        left: None
                    }
                } else if new_value == value {
                   return 0 // we don't allow duplicates.
                } else {
                    *self = BinaryTree {
                        metadata: (1, 0),
                        value: value,
                        left: Some(Box::new(BinaryTree {
                            metadata: (0, 0),
                            value: new_value,
                            left: None,
                            right: None
                        })),
                        right: None
                    }
                }
                1
            }
            BinaryTree {metadata: (ref mut left_height, right_height), ref mut value, left: Some(ref mut left ), right: _} if new_value < *value => {
                let incr = left.insert(new_value);
                *left_height += incr;
                assert!(incr < 2);
                if *left_height > right_height { incr } else { 0 }
            }
            BinaryTree {metadata: (ref mut left_height, right_height), ref mut value, ref mut left, right: _} if new_value < *value => {
                assert_eq!(0, *left_height);

                *left = Some(Box::new(BinaryTree {value: new_value, metadata: (0, 0), left: None, right: None}));
                *left_height += 1;
                if *left_height > right_height { 1 } else { 0 }
            }
            BinaryTree {metadata: (left_height, ref mut right_height), ref mut value, left: _, right: Some(ref mut right)} if new_value > *value => {
                let incr = right.insert(new_value);
                *right_height += incr;
                assert!(incr < 2);
                if *right_height > left_height { incr } else { 0 }
            }
            BinaryTree {metadata: (left_height, ref mut right_height), ref mut value, left: _, right: ref mut right} if new_value > *value => {
                assert_eq!(0, *right_height);

                *right = Some(Box::new(BinaryTree {value: new_value, metadata: (0, 0), left: None, right: None}));
                *right_height += 1;
                if *right_height > left_height { 1 } else { 0 }
            }
            BinaryTree {metadata: _, ref mut value, left: _, right: _} if *value == new_value => {
                0 // this is a duplicate value, do nothing.
            }
            BinaryTree {metadata: _, value: _, left: _, right: _} => unreachable!()
        };
        self.balance();
        self.fix_metadata();
        ret
    }

    fn fix_metadata(&mut self) {
        match self {
            &mut BinaryTree {metadata: (left, right), value: _,
                        left: Some(box BinaryTree {metadata: (left_left, left_right), value: _, left: _, right: _}),
                        right: Some(box BinaryTree {metadata: (right_left, right_right), value: _, left: _, right: _})}
            => {
                self.metadata = (std::cmp::max(left_left, left_right) + 1, std::cmp::max(right_left, right_right) + 1);
                println!("{:?} {:?}", self.metadata, (left, right));
            }
            &mut BinaryTree {metadata: (left, right), value: _,
                        left: None,
                        right: Some(box BinaryTree {metadata: (right_left, right_right), value: _, left: _, right: _})}
            => {
                self.metadata = (0, std::cmp::max(right_left, right_right) + 1);
                println!("{:?} {:?}", self.metadata, (left, right));
            }
            &mut BinaryTree {metadata: (left, right), value: _,
                        left: Some(box BinaryTree {metadata: (left_left, left_right), value: _, left: _, right: _}),
                        right: None}
            => {
                self.metadata = (std::cmp::max(left_left, left_right) + 1, 0);
                println!("{:?} {:?}", self.metadata, (left, right));
            }
            &mut BinaryTree {metadata: (left, right), value: _,
                        left: None,
                        right: None}
            => {
                self.metadata = (0, 0);
                println!("{:?} {:?}", self.metadata, (left, right));
            }
        }
    }

    fn balance(&mut self) {
        let difference: i8 = self.metadata.0 - self.metadata.1;

        if difference == 2  {
            let mut tmp = self.clone();
            let left = self.left.clone().expect("trying to rotate left subtree up");
            *self = *left;
            tmp.left = self.right.clone();
            tmp.fix_metadata();
            self.fix_metadata();
            self.right = Some(box tmp);
        } else if difference == -2 {
            let mut tmp = self.clone();
            let right = self.right.clone().expect("trying to rotate right subtree up");
            *self = *right;
            tmp.right = self.left.clone();
            tmp.fix_metadata();
            self.fix_metadata();
            self.left = Some(box tmp);
        } else if difference < -2 && difference > 2 {
            unreachable!()
        }
    }
}
