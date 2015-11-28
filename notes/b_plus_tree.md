B+ Trees
========

B+ trees are one of the fundamental datastructures for implementing databases. We often think of database storage as being basically flat, sequential files. That is, in fact, sometimes what is done (it's called ISAM). This has some pretty serious flaws, though. If the records are not fixed size, there's no way to find the record without searching the whole table. 

If records are fixed size, but you want to insert new records in the middle, then you have to move O(n) data to do an insert (because on average half of the data will be to the right of where you want to insert, and you'll have to shift that data over). That's also very slow.

B+ trees let you find data fast, and insert without shifting data around.

What Are They?
--------------

B+ trees are a kind of multi-way tree, where there are many children at each node. While this doesn't change the big O performance, it does make disk reads more efficient! Database operations are often IO-bound so optimizing disk accesses is very important. It costs about the same to read a whole block as to read part of one in most disks, so if you can make your nodes take up exactly one block, and maximize the branching that happens per block you read from disk, then you can minimize the number of nodes you have to visit and therefore minimize your IO. As a result, B+ trees often end up having about 100-1,000 children per node.

To start, let's talk about invariants: what to expect when looking at a B+ tree. 

Invariants
----------

A few definitions:
* There are 3 kinds of nodes: the root node, internal nodes, and the leaves
* Every node has the same capacity (normally between 100 and 1000)
* Every root and internal node has an array of **keys** and an array of **children** at each node. At the leaves, the children are replaced by values (the *data* in database!)

Now our invariants!

* All leaves are always on the same level. As a result, a B+ tree is self balancing *without rotations*, at least for inserts. This is because B+ trees grow from the top, by splitting the root node.
* Root nodes may have as few as two children after sufficient insertions have been performed. They start empty. They must have fewer children than their overall capacity.
* Internal nodes must have `ceil(capacity/2) <= num_children <= capacity` children.
* Keys are
 * Unique amongst root/internal nodes.
 * Also unique across leaves.
 * All keys in root/internal nodes will also be in the set of keys in leaf nodes. Put differently, the set of keys for root and internal nodes is a proper subset of the set of keys for leaves.
  * When there is only one node (the root), this is not yet true.

Algorithms
----------

Here's how we maintain those invariants, and how we search and insert things into the tree!

* If a node reaches capacity, it is split into three chunks: a median value, a new node with all the keys less than the median, and a new node with all the keys greater than the median. What happens next depends on if it's the root or an internal node.
  * When a root node is split, both sides of the split are retained, and a new root is created that points to the two sides of the split. This is how the tree grows up from the top, as mentioned previously.
  * When an internal node is split, both post-split nodes are inserted into the internal node's parent, and the median value is used as the key to separate them.
* Search:
  * Starting from the from the root:
    * If the value we're searching for is less than the first node, recurse to the leftmost child.
    * Iterate over all consecutive pairs of values, while maintaining a zero indexed counter for which pair we're on. If the value we're inserting is greater than the left value, and less than or equal to the right value, then recurse on the child corresponding to the iteration index.
    * If we haven't been able to recurse in the previous two cases, then recurse on the rightmost child. (FIXME: I think this is likely to be the child at the spot corresponding to the iteration index's final value plus two. Should check this.)
* Insertion starts by searching for the leaf node that would hold the node, if it were already in the tree.
  * If the value is already in the leaf, do nothing.
  * If the leaf is at capacity, follow the steps for splitting, but consider the new value as if it were already part of the node when creating the two new nodes/propagating the median upwards.

Descriptions
------------
* They grow from the top. In other words, when a B+ tree gets full, you add a new root, and split the existing root. *This is why they are self-balancing and all leaves are at the same level*.
* There are three node types:
  * Root: there's only one, it's at the top.
  * Internal: only used to embody the search structure.
  * Leaf: terminal nodes that hold pointers to data. Both other node types have no data, only keys.
* Insertions always happen at the lowest level of internal nodes.
* Internal and root nodes both have a list of keys of length one shorter than the maximum allowable number of children per node.
  * This is an annoyingly subtle point, so it bears some emphasis. If you have an equal number of keys and children, insertion gets complicated. Why? Because the tree must express the full range of possible values. At the root, this means that the rightmost *populated* entry in the child array must always be the maximal value for the type in question (even if this is infinity). That's a little annoying, and requires a tacky special case, but isn't too hard. For internal nodes, however, it gets pretty messy, because you have to propagate the constraints of the rightmost child down through all the rightmost children, so you don't end up with gaps in what data your tree has space for. This wouldn't be so bad, except for splits, which are covered further down this list.
* Leaves can either contain the data directly, or have a pointer (whether memory, disk, or whatever based, so long as it provides O(1) access to the data).
* Keys are used differently in leaves than internal/root nodes.
  * In internal/root nodes, the value of `key[i]` specifies that all keys in `children[j]` where `j <= i` are less than `key[i]`, as well as that all keys in `children[j]` where `j > i` are greater than or equal to `key[i]`.
  * In leaves, `key[i]` denotes that there is a corresponding element in `values[i]`. As a result, leaf nodes can only store `n-1` values/pointers to values in them, where `n` is the capacity root/internal nodes have for children..

