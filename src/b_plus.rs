extern crate quickcheck;
extern crate rand;
extern crate std;
extern crate generic_array;

use std::mem;
use quickcheck::Arbitrary;
use quickcheck::Gen;
use generic_array::GenericArray;
use generic_array::ArrayLength;

// here be type-dragons!
struct NAryTree<T, M, N: ArrayLength<Option<(T, Box<NAryTree<T, M, N>>)>>> {
    capacity: usize,
    children: GenericArray<Option<
            (T, // the maximal element for this child.
             Box<NAryTree<T, M, N>>)>, N>,
    data: Option<Box<T>>,
    metadata: M
}

enum BPlusNodeType {
    Root,
    Internal,
    Leaf
}

type BPlusTree<T, N: ArrayLength<Option<(T, Box<NAryTree<T, BPlusNodeType, N>>)>>> =
               NAryTree<T, BPlusNodeType, N>;

impl <T, N: ArrayLength<Option<(T, Box<BPlusTree<T, N>>)>>> BPlusTree<T, N> {
    fn empty(degree: usize) -> Self {
        NAryTree {
            capacity: degree,
            children: GenericArray::new(),
            data: None,
            metadata: BPlusNodeType::Root
        }
    }
}
