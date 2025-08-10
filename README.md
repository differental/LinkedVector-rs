# LinkedVector-rs

An experimental data structure that aims to increase cache hits when using linked lists.

## Background

There are two common containers for "dynamic chunks of data of the same type":

- Vectors (dynamic arrays where data are stored in contiguous chunks)
- Linked Lists (multiple nodes connected via pointers)

One advantage of linked lists is that it supports O(1) deletion - deletion only requires changing one pointer (and freeing the deleted item). For vectors, deletion requires O(n) shifting of all subsequent elements. 

Even with the deletion costs, typically we always prefer vectors simply due to caching - each node of a linked list resides in different chunks in the heap and is likely to result in a cache miss. This cost is often worse than the O(n) cost that comes with shifting elements.

## Experiment

This experiment is a random idea: What happens if we combine the advantages of linked lists and vectors?

We use the same model as linked lists, where each element is accompanied by a pointer to the next item. However, instead of allocating each node on the heap, we allocate them inside a vector (contiguous chunks). We then maintain a stack (using a separate vector) called freelist. Whenever a new item is added, we check the freelist for free spaces - if not, we allocate it inside the new space of the vector. Whenever an item is deleted, we change the pointers just like in a linked list, then add its original index into the freelist.


## Benchmark

```bash
cargo run --bin bench --release >> results.txt
```

The benchmark code compares this `LinkedVector` implementation against the `Vec` and `LinkedList` from standard library. For both a large struct and a simple u64, it records the time taken for construction, random access (at midpoint) and deletion (at midpoint). The structs are designed so that the required values are not calculated by the compiler at compile-time.


## Results

By varying the parameters (sizes of MyStruct and numbers of MyStruct and u64 in `src/bench.rs`), the conclusion seems to be:

- Vectors are always significantly faster at random access (due to obvious reasons) and usually slightly faster at construction.
- For primitive types or small structs (<1KB per object), vectors are faster than anything else including in deletion. In this case, one should simply prefer vectors to `LinkedList`s or `LinkedVector`s.
- For larger structs (>10KB per object), linked lists start to show an advantage in deletion. In these cases, often `LinkedVector`s are only slightly slower than linked lists at deletion (still significantly faster than vectors), while it is noticeably faster than linked lists at random access.

In summary, for structs larger than 10KB up to over 10MB, for tasks that require efficiencies in both random access and deletion, `LinkedVector` *can* be a better choice than "vanilla" vectors or linked lists.

## Notes

It is worth noting that the results vary quite significantly between different runs, so it might be a good idea to run multiple tests on a machine with sufficient memory. (Results will be updated here when completed)

