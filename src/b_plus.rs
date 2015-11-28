extern crate quickcheck;
extern crate rand;
extern crate std;

use std::mem;
use std::fmt::Debug;
use quickcheck::Arbitrary;
use quickcheck::Gen;

#[derive(Debug,Eq,PartialEq)]
struct BPlusTree<T: Clone+Debug+Ord> {
    capacity: usize,
    keys: [Option<T>; 9],
    children: [Option<Box<BPlusTree<T>>>; 10],
    data: [Option<Box<T>>; 9],
    kind: BPlusNodeType
}

impl <T: Clone+Debug+Ord> Clone for BPlusTree<T> {
    fn clone(&self) -> Self {
        BPlusTree {
            capacity: self.capacity.clone(),
            // the following should be replaced with a macro.
            keys: [
                self.keys[0].clone(),
                self.keys[1].clone(),
                self.keys[2].clone(),
                self.keys[3].clone(),
                self.keys[4].clone(),
                self.keys[5].clone(),
                self.keys[6].clone(),
                self.keys[7].clone(),
                self.keys[8].clone(),
                ],
            children: [
                self.children[0].clone(),
                self.children[1].clone(),
                self.children[2].clone(),
                self.children[3].clone(),
                self.children[4].clone(),
                self.children[5].clone(),
                self.children[6].clone(),
                self.children[7].clone(),
                self.children[8].clone(),
                self.children[9].clone()
                ],
            data: [
                self.data[0].clone(),
                self.data[1].clone(),
                self.data[2].clone(),
                self.data[3].clone(),
                self.data[4].clone(),
                self.data[5].clone(),
                self.data[6].clone(),
                self.data[7].clone(),
                self.data[8].clone(),
                ],
            kind: self.kind.clone()
        }
    }
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum BPlusNodeType {
    Root,
    Internal,
    Leaf
}

impl <T: Clone+Debug+Ord> BPlusTree<T> {
    fn leaf(data: T) -> Self {
        BPlusTree {
            capacity: 10,
            kind: BPlusNodeType::Leaf,
            keys: [None, None, None, None, None, None, None, None, None],
            children: [None, None, None, None, None, None, None, None, None, None],
            data: [None, None, None, None, None, None, None, None, None],
        }
    }

    fn search(&mut self, value: T) -> &mut Self {
        let mut final_index: Option<usize> = None;

        if self.kind == BPlusNodeType::Leaf {
            return self
        }

        if self.keys[0].is_some() && &value < self.keys[0].as_ref().unwrap() {
            return self.children[0].as_mut().expect("there should be a thinger here").search(value)
        }

        if self.keys[0].is_some() && &value == self.keys[0].as_ref().unwrap() {
            return self.children[1].as_mut().expect("there should be a thinger here").search(value)
        }

        for window in self.keys.windows(2).enumerate() {
            match window {
                (i, [Some(ref left), Some(ref right)]) => {
                    if left < &value && &value <= right {
                        return self.children[i].as_mut().expect("there should be a thinger here").search(value)
                    }
                }
                (i, [Some(_), None]) => {
                    final_index = Some(i);
                    break
                }
                _ => unreachable!()
            }
        }

        match final_index {
            Some(i) => {
                if self.keys[i].is_some() && &value >= self.keys[i].as_ref().unwrap() {
                    return self.children[i+1].as_mut().expect("there should be a thinger here").search(value)
                }
            }
            None => ()
        }

        println!("{:?} {:?}", value, self);
        unreachable!()
    }

    fn split(&mut self) {}

    fn insert(&mut self, value: T) {}
}

#[test]
fn search_basically_works() {
    //example b+ tree cribbed from http://www.cburch.com/cs/340/reading/btree/, first example under insertion section.
    let mut bt: BPlusTree<i32> = BPlusTree {
        capacity: 10,
        kind: BPlusNodeType::Root,
        keys: [Some(16), None, None, None, None, None, None, None, None],
        data: [None, None, None, None, None, None, None, None, None],
        children: [
            Some(box BPlusTree {
                capacity: 10,
                kind: BPlusNodeType::Leaf,
                keys: [Some(1), Some(4), Some(9), None, None, None, None, None, None],
                children: [None, None, None, None, None, None, None, None, None, None],
                data: [Some(box 1), Some(box 4), Some(box 9), None, None, None, None, None, None],
            }),
            Some(box BPlusTree {
                capacity: 10,
                kind: BPlusNodeType::Leaf,
                keys: [Some(16), Some(25), None, None, None, None, None, None, None],
                children: [None, None, None, None, None, None, None, None, None, None],
                data: [Some(box 16), Some(box 25), None, None, None, None, None, None, None],
            }), None, None, None, None, None, None, None, None]
    };

    assert_eq!(bt.clone().children[1], Some(box bt.search(1000).clone()));
    assert_eq!(bt.clone().children[0], Some(box bt.search(-1000).clone()));
    assert_eq!(bt.clone().children[0], Some(box bt.search(0).clone()));
    assert_eq!(bt.clone().children[1], Some(box bt.search(16).clone()));
}
