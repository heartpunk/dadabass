#[test]
fn it_works() {
}

enum BinaryTree<'a, V: 'a, M: 'a> {
    Branch {
        metadata: M,
        value: V,
        left: &'a BinaryTree<'a, V, M>,
        right: &'a BinaryTree<'a, V, M>
    },
    Leaf {
        value: V,
        metadata: M
    }
}

type AvlTree<'a, V: 'a> = BinaryTree<'a, V, i32>;

static EXAMPLE_TREE: BinaryTree<'static, i32, i32> = BinaryTree::Branch {
    metadata: 0,
    value: 4,
    left: &BinaryTree::Leaf {
        metadata: 0,
        value: 3
    },
    right: &BinaryTree::Leaf {
        metadata: 0,
        value: 5
    }
};

impl <'a, V> AvlTree<'a, V> {
    fn insert(&mut self) {
        match self {
           BinaryTree::Leaf {value, metadata: _} => (),
           BinaryTree::Branch {metadata: branching_factor, value, left, right} => ()
        }
    }
}
