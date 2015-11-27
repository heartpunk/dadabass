extern crate quickcheck;
extern crate rand;
extern crate std;

use std::mem;
use std::fmt::Debug;
use quickcheck::Arbitrary;
use quickcheck::Gen;

#[derive(Debug,Eq,PartialEq)]
struct NAryTree<T: Clone+Ord, M: Clone> {
    capacity: usize,
    children: [Option<
            (T, // the maximal element for this child.
             Box<NAryTree<T, M>>)>; 10],
    data: Option<Box<T>>,
    metadata: M
}

#[derive(Clone,Debug,Eq,PartialEq)]
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

    #[allow(dead_code)]
    fn path_iter(&self) -> BPlusTreePathIterator<T> {
        BPlusTreePathIterator {to_visit: vec![vec![self.clone()]]}
    }

    fn search_iter(&'a self, value: T) -> BPlusTreeSearchIterator<'a, T> {
        BPlusTreeSearchIterator {value: value, current: &self}
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

#[allow(dead_code)]
struct BPlusTreePathIterator<T: Ord+Copy+Clone+Send> {
    to_visit: Vec<Vec<BPlusTree<T>>>
}

impl <T: Ord+Copy+Clone+Send+Bounded+Debug> Iterator for BPlusTreePathIterator<T> {
    type Item = Vec<BPlusTree<T>>;

    fn next(&mut self) -> Option<Vec<BPlusTree<T>>> {
        let maybe_path = self.to_visit.pop();
        match maybe_path {
            Some(ref path) => {
                let last_index = path.len() - 1;
                let next = &path[last_index];
                for child in next.populated_children().iter().rev() {
                    let mut new_path = path.clone();
                    let unwrapped_child = *child.as_ref().expect("populated_children filters nones").1.clone();
                    new_path.push(unwrapped_child);
                    self.to_visit.push(new_path);
                }
            }
            None => ()
        }
        maybe_path
    }
}

struct BPlusTreeSearchIterator<'a, T: 'a+Ord+Copy+Clone+Send> {
    value: T,
    current: &'a BPlusTree<T>
}

impl <'a, T: 'a+Ord+Copy+Clone+Send+Bounded+Debug> Iterator for BPlusTreeSearchIterator<'a, T> {
    type Item = &'a mut BPlusTree<T>;

    fn next(&mut self) -> Option<&'a mut BPlusTree<T>> {
        let ret = self.current;
        for pair in ret.populated_children().windows(2) {
            match pair {
                [Some((ref last_val, _)), Some((ref current_val, ref current_child))] => {
                    if &self.value > last_val && &self.value < current_val {
                        self.current = current_child;//.search_for_closest_node(self.value)
                    } else {
                        continue
                    }
                }
                [Some((ref val, ref child)), None] | [Some((ref val, ref child))] => {
                    self.current = child;//.search_for_closest_node(self.value)
                }
                _ => unreachable!()
            }
        };
        if ret == self.current {
            None
        } else {
            Some(ret)
        }
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

    fn path_to_node(&self, value: T) -> Vec<&mut BPlusTree<T>> {
        self.search_iter(value).collect::<Vec<&mut BPlusTree<T>>>()
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

    fn insert(&mut self, new_value: T) {
        unreachable!()//self.search_for_closest_node(new_value).insert_at(new_value);
    }

    fn insert_at(&mut self, new_value: T) -> Option<(T, BPlusTree<T>)> {
        // this is a bug. should search for appropriate node and insert there. if that node doesn't
        // have capacity, then we split (and propagate splits upward as needed).
        if self.used_capacity() < self.capacity {
            self.insert_node(Some((new_value, box BPlusTree::leaf_containing(new_value))));
            None
        } else {
            let go_left = new_value < self.median();
            let (mut left, mut right) = self.split();

            if go_left {
                match left.insert_at(new_value) {
                    Some((t, result)) => {
                        left.insert_node(Some((t, box result)))
                    }
                    _ => ()
                }
            } else {
                match right.insert_at(new_value) {
                    Some((t, result)) => {
                        right.insert_node(Some((t, box result)))
                    }
                    _ => ()
                }
            };

            match self.metadata {
                BPlusNodeType::Root => {
                    let max_value_in_left = left.populated_children()[left.used_capacity() - 1].as_ref().expect("").0;
                    let max_value_in_right = T::max_value();
                    left.metadata = BPlusNodeType::Internal;
                    right.metadata = BPlusNodeType::Internal;
                    self.children = [Some((max_value_in_left, box left)), Some((max_value_in_right, box right)), None, None, None, None, None, None, None, None];
                    None
                }
                BPlusNodeType::Internal => {
                    left.metadata = BPlusNodeType::Internal;
                    right.metadata = BPlusNodeType::Internal;
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
        while g.gen() {
            for i in 0..15 { tree.insert(g.gen()); }
        }
        tree
    }
}

#[quickcheck]
fn lol(_: BPlusTree<i32>) -> bool {
    true
}

#[quickcheck]
fn only_one_root(bt: BPlusTree<i32>) -> bool {
    bt.iter().filter(|bt| bt.metadata == BPlusNodeType::Root).count() == 1
}

#[quickcheck]
fn leaves_have_no_children(bt: BPlusTree<i32>) -> bool {
    bt.iter().filter(|bt| bt.metadata == BPlusNodeType::Leaf).all(|bt| bt.used_capacity() == 0)
}

#[quickcheck]
fn internal_nodes_have_only_leaves_or_internal_nodes_for_children(bt: BPlusTree<i32>) -> bool {
    bt.iter()
        //.filter(|bt| bt.metadata == BPlusNodeType::Internal)
        .all(|bt|
             bt.populated_children().iter()
             .map(|child| &child.as_ref().expect("this should always be Some or populated_children is broken").1)
             .all(|child|
                  child.metadata == BPlusNodeType::Internal
                  || child.metadata == BPlusNodeType::Leaf)
             )
}

#[quickcheck]
fn internal_nodes_have_either_all_leaves_or_all_internal_nodes_for_children(bt: BPlusTree<i32>) -> bool {
    bt.iter()
        .filter(|bt| bt.metadata == BPlusNodeType::Internal)
        .all(|bt|
             bt.populated_children().iter()
             .map(|child| &child.as_ref().expect("this should always be Some or populated_children is broken").1)
             .all(|child|
                  child.metadata == BPlusNodeType::Internal)
             ||
             bt.populated_children().iter()
             .map(|child| &child.as_ref().expect("this should always be Some or populated_children is broken").1)
             .all(|child|
                  child.metadata == BPlusNodeType::Leaf)
             )
}

//#[quickcheck]
//fn root_node_always_covers_entire_range(bt: BPlusTree<i32>) -> bool {
//    let children = bt.iter()
//        .filter(|bt| bt.metadata == BPlusNodeType::Root)
//        .collect::<Vec<&BPlusTree<i32>>>()
//        .pop().expect("all trees must have roots")
//        .populated_children();
//    children[children.len() - 1].as_ref().expect("we trust populated_children").0 == std::i32::MAX
//}

#[quickcheck]
fn all_leaves_are_at_same_height(bt: BPlusTree<i32>) -> bool {
    let number_of_heights =
        bt.path_iter()
        .filter(|path| path[path.len() - 1].metadata == BPlusNodeType::Leaf)
        .map(|path| path.len())
        .collect::<std::collections::HashSet<usize>>().len();
    number_of_heights == 0 || number_of_heights == 1
}

#[test]
fn splits_preserve_count() {
    let mut tree: BPlusTree<i32> = BPlusTree::empty();
    let mut insertions = 0;
    for i in 0..10 {
        tree.insert(i);
        insertions += 1;
    }
    assert!(tree.iter().filter(|bt| bt.metadata == BPlusNodeType::Leaf).count() == 10);
    assert!(insertions == 10);
    for i in 10..12 {
        tree.insert(i);
        insertions += 1;
    }
    assert!(insertions == 12);
    assert!(tree.iter().filter(|bt| bt.metadata == BPlusNodeType::Leaf).count() == 12);
}

#[test]
fn path_iterator_example_test() {
    let mut tree: BPlusTree<i32> = BPlusTree::empty();

    for i in 0..12 {
        tree.insert(i);
    }

    assert_eq!(15, tree.path_iter().count());
    assert_eq!(1, tree.path_iter().filter(|path| path.len() == 1).count());
    println!("{:?}", tree.path_iter().filter(|path| path.len() == 2).collect::<Vec<Vec<BPlusTree<i32>>>>());
    assert_eq!(2, tree.path_iter().filter(|path| path.len() == 2).count());
    assert_eq!(12, tree.path_iter().filter(|path| path.len() == 3).count());
}
