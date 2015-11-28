B+ Trees
========

B+ trees are one of the fundamental datastructures for implementing databases. We often think of database storage as being basically flat, sequential files. That is, in fact, sometimes what is done (it's called ISAM). This has some pretty serious flaws, though. For example, if you want to modify a particular record, and they aren't fixed size, you have to guess about where that record is, and then scooch back and forth until you find it. If records are fixed size, that particular problem goes away. However, if you have a nonsequential primary key, want to sort the data by primary key (this is a pretty good idea), then inserts mean you have to move O(n) data to do an insert (because on average half of the data will be to the right of where you want to insert, and you'll have to shift that data over). That's clearly not good.

B+ trees still use relatively few seeks to find the data, but you can insert without shifting data around. In common cases, this avoids the scooching around problem, too, but Knuth mentions it may sometimes be desirable to have that at the lower levels of the tree.

What Are They?
--------------

The short (and not terribly useful) answer is that they are B-trees where all the data is stores in leaves. But unless you know what a B-tree is, that doesn't help much, and unfortunately, as many people have noted, the literature is inconsistent on how to implement B-trees as well as what terminology to use for aspects of their implementations.

Rather than get hung up on the differences, I'll paint a rough picture first of what properties they all share:

Descriptions
------------
* They grow from the top. In other words, when a B+ tree gets full, you add a new root, and split the existing root.
* There are three node types:
  * Root: there's only one, it's at the top.
  * Internal: only used to embody the search structure.
  * Leaf: terminal nodes that hold pointers to data. Both other node types have no data, only keys.
* Insertions always happen at the lowest level of internal nodes.
* Internal and root nodes both have a list of keys of length one shorter than the maximum allowable number of children per node.
  * This is an annoyingly subtle point, so it bears some emphasis. If you have an equal number of keys and children, insertion gets complicated. Why? Because the tree must express the full range of possible values. At the root, this means that the rightmost *populated* entry in the child array must always be the maximal value for the type in question (even if this is infinity). That's a little annoying, and requires a tacky special case, but isn't too hard. For internal nodes, however, it gets pretty messy, because you have to propagate the constraints of the rightmost child down through all the rightmost children, so you don't end up with gaps in what data your tree has space for. This wouldn't be so bad, except for splits, which are covered further down this list.
* Leaves can either contain the data directly, or have a pointer (whether memory, disk, or whatever based, so long as it provides O(1) access to the data).

Invariants
----------

* All leaves are always on the same level. As a result, a B+ tree is self balancing *without rotations*, at least for inserts.
* Root nodes may have as few as two children after sufficient insertions have been performed. They start empty. They must have fewer children than their overall capacity.
* Internal nodes must have `ceil(n/2) <= num_children <= n` children.

Algorithms
----------

* If a node reaches capacity, it is split into three chunks: a median value, a new node with all the keys less than the median, and a new node with all the keys greater than the median. What happens next depends on if it's the root or an internal node.
  * When a root node is split, both sides of the split are retained, and a new root is created that points to the two sides of the split. This is how the tree grows up from the top, as mentioned previously.
  * When an internal node is split, both post-split nodes are inserted into the internal node's parent, and the median value is used as the key to separate them.
