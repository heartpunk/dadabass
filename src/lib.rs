#[test]
fn it_works() {
    let example_tree: BinaryTree<i32, i32> = BinaryTree::Branch {
        metadata: 0,
        value: 4,
        left: Box::new(BinaryTree::Leaf {
            metadata: 0,
            value: 3
        }),
        right: Box::new(BinaryTree::Leaf {
            metadata: 0,
            value: 5
        })
    };
}

enum BinaryTree<V, M> {
    Branch {
        metadata: M,
        value: V,
        left: Box<BinaryTree<V, M>>,
        right: Box<BinaryTree<V, M>>
    },
    Leaf {
        value: V,
        metadata: M
    }
}

type AvlTree<'a, V: 'a> = BinaryTree<V, i32>;


impl <'a, V> AvlTree<'a, V> {
    fn insert(&mut self) {
        match self {
           &mut BinaryTree::Leaf {ref mut value, metadata: _} => (),
           &mut BinaryTree::Branch {metadata: ref mut branching_factor, ref mut value, ref mut left, ref mut right} => ()
        }
    }
}
