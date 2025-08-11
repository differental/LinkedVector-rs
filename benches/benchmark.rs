use std::{collections::LinkedList,hint::black_box};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use linkedvector::LinkedVector;

const N_STRUCTS: [usize; 7] = [10, 20, 50, 100, 500, 1_000, 2_000];
const SIZE_STRUCT: usize = 50_000;

#[derive(Clone, Debug)]
struct MyStruct {
    nums: [u64; SIZE_STRUCT],
}

impl MyStruct {
    fn new(i: usize) -> Self {
        let mut nums = [0u64; SIZE_STRUCT];
        for (j, n) in nums.iter_mut().enumerate() {
            *n = (i as u64) * 10 + j as u64;
        }
        Self { nums }
    }
}

fn bench_construction_vec<T: Clone>(count: usize, element: T) -> Vec<T> {
    let mut v: Vec<T> = Vec::with_capacity(count);
    for _ in 0..count {
        v.push(element.clone());
    }
    v
}

fn bench_construction_linkedlist<T: Clone>(count: usize, element: T) -> LinkedList<T> {
    let mut l: LinkedList<T> = LinkedList::new();
    for _ in 0..count {
        l.push_back(element.clone());
    }
    l
}

fn bench_construction_linkedvector<T: Clone>(count: usize, element: T) -> LinkedVector<T> {
    let mut lv: LinkedVector<T> = LinkedVector::with_capacity(count);
    for _ in 0..count {
        lv.push_back(element.clone());
    }
    lv
}

fn bench_random_access_vec<T: Clone>(v: &Vec<T>) -> Option<T> {
    let mid = v.len() / 2;
    Some(v[mid].clone())
}

fn bench_random_access_linkedlist<T: Clone>(l: &LinkedList<T>) -> Option<T> {
    let mid = l.len() / 2;
    l.iter().nth(mid).cloned()
}

fn bench_random_access_linkedvector<T: Clone>(lv: &LinkedVector<T>) -> Option<T> {
    let mid = lv.len() / 2;
    lv[mid].item.clone()
}

fn bench_delete_vec<T>(mut v: Vec<T>, idx: usize) -> Vec<T> {
    v.remove(idx);
    v
}

fn bench_delete_linkedlist<T>(mut l: LinkedList<T>, idx: usize) -> LinkedList<T> {
    // Use split_off to split at idx, pop_front from the tail, then append back
    let mut tail = l.split_off(idx);
    // remove the first element of tail
    tail.pop_front();
    l.append(&mut tail);
    l
}

fn bench_delete_linkedvector<T>(mut lv: LinkedVector<T>, idx: usize) -> LinkedVector<T> {
    let _removed = lv.delete(idx);
    lv
}

fn bench_constructions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Construction");

    let element = MyStruct::new(42);

    for n in N_STRUCTS.iter() {
        group.bench_with_input(BenchmarkId::new("Vec", n), n, |b, n| {
            b.iter(|| black_box(bench_construction_vec(*n, element.clone())))
        });
        group.bench_with_input(BenchmarkId::new("LinkedList", n), n, |b, n| {
            b.iter(|| black_box(bench_construction_linkedlist(*n, element.clone())))
        });
        group.bench_with_input(BenchmarkId::new("LinkedVector", n), n, |b, n| {
            b.iter(|| black_box(bench_construction_linkedvector(*n, element.clone())))
        });
    }
    group.finish();
}

fn bench_random_accesses(c: &mut Criterion) {
    let mut group = c.benchmark_group("Random Access");

    let element = MyStruct::new(42);

    for n in N_STRUCTS.iter() {
        let vec_item = bench_construction_vec(*n, element.clone());
        let linkedlist_item = bench_construction_linkedlist(*n, element.clone());
        let linkedvector_item = bench_construction_linkedvector(*n, element.clone());

        group.bench_with_input(BenchmarkId::new("Vec", n), n, |b, _| {
            b.iter(|| black_box(bench_random_access_vec(&vec_item)))
        });
        group.bench_with_input(BenchmarkId::new("LinkedList", n), n, |b, _| {
            b.iter(|| black_box(bench_random_access_linkedlist(&linkedlist_item)))
        });
        group.bench_with_input(BenchmarkId::new("LinkedVector", n), n, |b, _| {
            b.iter(|| black_box(bench_random_access_linkedvector(&linkedvector_item)))
        });
    }
    group.finish();
}

fn bench_deletions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Deletion");

    let element = MyStruct::new(42);

    for n in N_STRUCTS.iter() {
        let vec_item = bench_construction_vec(*n, element.clone());
        let linkedlist_item = bench_construction_linkedlist(*n, element.clone());
        let linkedvector_item = bench_construction_linkedvector(*n, element.clone());

        group.bench_with_input(BenchmarkId::new("Vec", n), n, |b, n| {
            b.iter(|| black_box(bench_delete_vec(vec_item.clone(), n / 2)))
        });
        group.bench_with_input(BenchmarkId::new("LinkedList", n), n, |b, n| {
            b.iter(|| black_box(bench_delete_linkedlist(linkedlist_item.clone(), n / 2)))
        });
        group.bench_with_input(BenchmarkId::new("LinkedVector", n), n, |b, n| {
            b.iter(|| black_box(bench_delete_linkedvector(linkedvector_item.clone(), n / 2)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_constructions,
    bench_random_accesses,
    bench_deletions
);
criterion_main!(benches);
