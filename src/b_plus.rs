extern crate quickcheck;
extern crate rand;
extern crate std;

use std::mem;
use std::fmt::Debug;
use quickcheck::Arbitrary;
use quickcheck::Gen;

#[derive(Debug)]
struct NAryTree<T: Clone+Ord, M: Clone> {
    capacity: usize,
    children: [Option<
            (T, // the maximal element for this child.
             Box<NAryTree<T, M>>)>; 10],
    data: Option<Box<T>>,
    metadata: M
}

#[derive(Clone,Debug)]
enum BPlusNodeType {
    Root,
    Internal,
    Leaf
}

impl <'a, T: Ord+Copy+Clone+Send> BPlusTree<T> {
    #[allow(dead_code)]
    fn iter(&'a self) -> BPlusTreeIterator<'a, T> {
        BPlusTreeIterator {to_visit: vec![&self]}
    }
}

#[allow(dead_code)]
struct BPlusTreeIterator<'a, T: 'a+Ord+Copy+Clone+Send> {
    to_visit: Vec<&'a BPlusTree<T>>
}

impl <'a, T: 'a+Ord+Copy+Clone+Send+Bounded+Debug> Iterator for BPlusTreeIterator<'a, T> {
    type Item = &'a BPlusTree<T>;

    fn next(&mut self) -> Option<&'a BPlusTree<T>> {
        let ret = self.to_visit.pop();
        match ret {
            Some(&ref next) => {
                for child in next.populated_children().iter().rev() {
                    self.to_visit.push(&child
                                       .as_ref()
                                       .expect("populated_children filters nones")
                                       .1
                                       )
                }
            }
            None => ()
        }
        ret
    }
}

trait Bounded {
    fn max_value() -> Self;
    fn min_value() -> Self;
}

impl Bounded for i32 {
    fn max_value() -> i32 {i32::max_value()}
    fn min_value() -> i32 {i32::min_value()}
}

type BPlusTree<T: Clone+Ord> = NAryTree<T, BPlusNodeType>;

impl <T: Clone+Ord, M: Clone> Clone for NAryTree<T, M> {
    fn clone(&self) -> Self {
        NAryTree {
            capacity: self.capacity.clone(),
            // the following should be replaced with a macro.
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
            data: self.data.clone(),
            metadata: self.metadata.clone()
        }
    }
}

impl <T: Clone+Send+Ord+Copy+Bounded+Debug> BPlusTree<T> {
    fn empty() -> Self {
        NAryTree {
            capacity: 10,
            // apparently the [value; count] syntax requires the Copy trait.
            children: [None, None, None, None, None, None, None, None, None, None],
            data: None,
            metadata: BPlusNodeType::Root
        }
    }

    fn leaf() -> Self {
        let mut tree = BPlusTree::empty();
        tree.metadata = BPlusNodeType::Leaf;
        tree
    }

    fn leaf_containing(value: T) -> Self {
        let mut tree = BPlusTree::leaf();
        tree.data = Some(box value);
        tree
    }

    fn internal_node(values: Vec<Option<(T, Box<BPlusTree<T>>)>>) -> Self {
        assert!(values.len() <= 10);
        let mut tree: BPlusTree<T> = BPlusTree {
            capacity: 10,
            children: [None, None, None, None, None, None, None, None, None, None],
            data: None,
            metadata: BPlusNodeType::Internal
        };
        for tuple in values.into_iter().enumerate().take(10) {
            tree.children[tuple.0] = tuple.1
        }
        tree
    }

    fn populated_children(&self) -> &[Option<(T, Box<BPlusTree<T>>)>] {
        let mut i = 0;
        for tuple in self.children.into_iter() {i += 1; if tuple.is_none() {i -= 1;break}}
        // split_at is exclusive of the index, so we don't adjust the index down by one.
        // in other words, i points at the first None, not the last Some.
        self.children.split_at(i).0
    }

    fn median_index(&self) -> usize {
        self.populated_children().len() / 2
    }

    fn median(&self) -> T {
        self.children[self.median_index()].as_ref().expect("this should be a Some by construction").0
    }

    fn split(&self) -> (Self, Self) {
        let (left, right) = self.children.split_at(self.median_index());
        (BPlusTree::internal_node(left.to_vec()), BPlusTree::internal_node(right.to_vec()))
    }

    fn used_capacity(&self) -> usize {
        self.populated_children().len()
    }

    fn insert_node(&mut self, node: Option<(T, Box<BPlusTree<T>>)>) {
        assert!(self.capacity > self.used_capacity());
        let final_index = self.children.len() - 1;
        self.children[final_index] = node;
        self.children.sort_by(|a, b|
                              match (a,b) {
                                  (&Some((max_1, _)), &Some((max_2, _))) => max_1.cmp(&max_2),
                                  (&None, &Some(_)) => std::cmp::Ordering::Greater,
                                  (&Some(_), &None) => std::cmp::Ordering::Less,
                                  (&None, &None) => std::cmp::Ordering::Equal,
                              })
    }

    fn insert(&mut self, new_value: T) -> Option<(T, BPlusTree<T>)> {
        if self.used_capacity() < self.capacity {
            self.insert_node(Some((new_value, box BPlusTree::leaf_containing(new_value))));
            None
        } else {
            let go_left = new_value < self.median();
            let (mut left, mut right) = self.split();

            if go_left {
                match left.insert(new_value) {
                    Some((t, result)) => {
                        left.insert_node(Some((t, box result)))
                    }
                    _ => ()
                }
            } else {
                match right.insert(new_value) {
                    Some((t, result)) => {
                        right.insert_node(Some((t, box result)))
                    }
                    _ => ()
                }
            };

            match self.metadata {
                BPlusNodeType::Root => {
                    let max_value_in_left = left.populated_children()[left.used_capacity() - 1].as_ref().expect("").0;
                    let max_value_in_right = right.populated_children()[right.used_capacity() - 1].as_ref().expect("").0;
                    self.children = [Some((max_value_in_left, box left)), Some((max_value_in_right, box right)), None, None, None, None, None, None, None, None];
                    None
                }
                BPlusNodeType::Internal => {
                    *self = left;
                    Some((right.children[0].as_ref().expect("must be a value because we just split").0, right))
                }
                BPlusNodeType::Leaf => {
                    panic!("haven't figured out what to do here yet.")
                }
            }
        }
    }
}

impl Arbitrary for BPlusTree<i32> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut tree: BPlusTree<i32> = BPlusTree::empty();
        println!("{:?}", tree);
        while g.gen() {
            tree.insert(g.gen());
            println!("{:?}", tree)
        }
        tree
    }
}

#[quickcheck]
fn lol(_: BPlusTree<i32>) -> bool {
    true
}
