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

    fn insert(&mut self, new_value: T) {
        for i in 1..self.children.len() - 1 {
            let (left, right) = self.children.split_at_mut(i);
            let last_left_index = left.len()-1;
            match (&mut left[last_left_index], &mut right[0]) {
                (&mut Some(( ref max, _)), &mut Some((_, _))) if &new_value <= max => continue,
                (&mut Some(( ref max_1, _)), &mut Some(( ref max_2, ref mut child))) if &new_value > max_1 && &new_value < max_2 => {
                    unreachable!();
                    child.insert(new_value)
                }
                (&mut Some(( ref max, box ref mut child)), &mut None) if &new_value <= max => {
                    println!("\n\nchild before insertion: {:?}\n", child);
                    child.insert(new_value);
                    println!("\n\nchild after insertion: {:?}\n", child);
                    assert!(i < 2);
                    break
                }
                (&mut Some(( ref max, _)), ref mut slot) if &new_value > max => {
                    unreachable!();
                    **slot = Some((*max,box BPlusTree::leaf()));
                    break
                }
                (ref mut slot, _) => {
                    **slot = Some(((T::max_value(), box BPlusTree::leaf())))
                }
                //(&mut Some(_), _) => unreachable!(),
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
