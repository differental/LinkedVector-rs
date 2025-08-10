# LinkedVector-rs

An experimental data structure that might be extremely dumb

## Background

There are two common containers for "dynamic chunks of data of the same type":

- Vectors (dynamic arrays where data are stored in contiguous chunks)
- Linked Lists (multiple nodes connected via pointers)

One advantage of linked lists is that it supports O(1) deletion - deletion only requires changing one pointer (and freeing the deleted item). For vectors, deletion requires O(n) shifting of all subsequent elements. 

Even with the deletion costs, typically we always prefer vectors simply due to caching - each node of a linked list resides in different chunks in the heap and is likely to result in a cache miss. This cost is often worse than the O(n) cost that comes with shifting elements.

## Experiment

This experiment is a random idea: What happens if we combine the advantages of linked lists and vectors?

We use the same model as linked lists, where each element is accompanied by a pointer to the next item. However, instead of allocating each node on the heap, we allocate them inside a vector (contiguous chunks). We then maintain a stack (using a separate vector) called freelist. Whenever a new item is added, we check the freelist for free spaces - if not, we allocate it inside the new space of the vector. Whenever an item is deleted, we change the pointers just like in a linked list, then add its original index into the freelist.


