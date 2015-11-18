extern crate quickcheck;
extern crate rand;
extern crate std;

use std::mem;
use quickcheck::Arbitrary;
use quickcheck::Gen;

struct NAryTree<T: Clone, M: Clone> {
    capacity: usize,
    children: [Option<
            (T, // the maximal element for this child.
             Box<NAryTree<T, M>>)>; 10],
    data: Option<Box<T>>,
    metadata: M
}

#[derive(Clone)]
enum BPlusNodeType {
    Root,
    Internal,
    Leaf
}

type BPlusTree<T: Clone> = NAryTree<T, BPlusNodeType>;

impl <T: Clone, M: Clone> Clone for NAryTree<T, M> {
    fn clone(&self) -> Self {
        NAryTree {
            capacity: self.capacity.clone(),
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

impl <T: Clone+Send> BPlusTree<T> {
    fn empty() -> Self {
        NAryTree {
            capacity: 10,
            children: [None, None, None, None, None, None, None, None, None, None],
            data: None,
            metadata: BPlusNodeType::Root
        }
    }

    fn insert(&mut self, new_value: T) {
        for i in (0..self.children.len()-1) {
            match (&self.children[i], &self.children[i+1]) {
                (&Some(( ref max_1, ref child )), &Some(( ref max_2, _ ))) => (),
                (&Some(( ref max, ref child )), &None) => (),
                (&None, _) => unreachable!()
            }
      }
    }
}

impl Arbitrary for BPlusTree<i32> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut tree: BPlusTree<i32> = BPlusTree::empty();
        tree
    }
}
