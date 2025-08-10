/// Benchmark Binary Target
use std::collections::LinkedList;
use std::mem::size_of;
use std::time::{Duration, Instant};

use linkedvector::LinkedVector;

const N_STRUCT: usize = 20; // number of MyStruct elements
const SIZE_STRUCT: usize = 50_000; // size of MyStruct is roughly 8*(24 + SIZE_STRUCT) bytes
const N_PRIMITIVE: usize = 100_000; // number of u64 elements

#[derive(Clone, Debug)]
struct MyStruct {
    item: String,
    nums: [u64; SIZE_STRUCT],
    extra: u128,
}

impl MyStruct {
    fn new(i: usize) -> Self {
        let s = format!("Hello, world! item #{i}");
        let mut nums = [0u64; SIZE_STRUCT];
        for (j, n) in nums.iter_mut().enumerate() {
            *n = (i as u64) * 10 + j as u64;
        }
        Self {
            item: s,
            nums,
            extra: u128::MAX - (i as u128),
        }
    }
}

fn human_bytes(b: usize) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut i = 0usize;
    let mut val = b as f64;
    while val >= 1024.0 && i < UNITS.len() - 1 {
        val /= 1024.0;
        i += 1;
    }
    format!("{:.2} {}", val, UNITS[i])
}

fn linkedlist_mem<T>(len: usize) -> (usize, usize) {
    let per_node = size_of::<T>() + 2 * size_of::<usize>();
    let used = per_node * len;
    (used, used)
}

fn bench_construction_vec<T: Clone>(count: usize, element: T) -> (Vec<T>, Duration) {
    let mut v: Vec<T> = Vec::with_capacity(count);
    let start = Instant::now();
    for _ in 0..count {
        v.push(element.clone());
    }
    let dur = start.elapsed();
    (v, dur)
}

fn bench_construction_linkedlist<T: Clone>(count: usize, element: T) -> (LinkedList<T>, Duration) {
    let mut l: LinkedList<T> = LinkedList::new();
    let start = Instant::now();
    for _ in 0..count {
        l.push_back(element.clone());
    }
    let dur = start.elapsed();
    (l, dur)
}

fn bench_construction_linkedvector<T: Clone>(
    count: usize,
    element: T,
) -> (LinkedVector<T>, Duration) {
    let mut lv: LinkedVector<T> = LinkedVector::new();
    let start = Instant::now();
    for _ in 0..count {
        lv.push_back(element.clone());
    }
    let dur = start.elapsed();
    (lv, dur)
}

fn bench_random_access_vec<T: Clone>(v: &Vec<T>) -> (Option<T>, Duration) {
    let mid = v.len() / 2;
    let start = Instant::now();
    let out = Some(v[mid].clone());
    let dur = start.elapsed();
    (out, dur)
}

fn bench_random_access_linkedlist<T: Clone>(l: &LinkedList<T>) -> (Option<T>, Duration) {
    let mid = l.len() / 2;
    let start = Instant::now();
    let out = l.iter().nth(mid).cloned();
    let dur = start.elapsed();
    (out, dur)
}

fn bench_random_access_linkedvector<T: Clone>(lv: &LinkedVector<T>) -> (Option<T>, Duration) {
    let mid = lv.len() / 2;
    let start = Instant::now();
    let out = (&lv[mid]).item.as_ref().cloned();
    let dur = start.elapsed();
    (out, dur)
}

fn bench_delete_vec<T>(mut v: Vec<T>, idx: usize) -> (Vec<T>, Duration) {
    let start = Instant::now();
    v.remove(idx);
    let dur = start.elapsed();
    (v, dur)
}

fn bench_delete_linkedlist<T>(mut l: LinkedList<T>, idx: usize) -> (LinkedList<T>, Duration) {
    // Use split_off to split at idx, pop_front from the tail, then append back
    let start = Instant::now();
    let mut tail = l.split_off(idx);
    // remove the first element of tail
    tail.pop_front();
    l.append(&mut tail);
    let dur = start.elapsed();
    (l, dur)
}

fn bench_delete_linkedvector<T>(
    mut lv: LinkedVector<T>,
    idx: usize,
) -> (LinkedVector<T>, Duration) {
    let start = Instant::now();
    let _removed = lv.delete(idx);
    let dur = start.elapsed();
    (lv, dur)
}

fn report_mem_vec<T>(v: &Vec<T>) {
    let used = size_of::<T>() * v.len();
    let real = size_of::<T>() * v.capacity();
    println!("Vec: len = {}, capacity = {}", v.len(), v.capacity());
    println!("  used = {} ({})", used, human_bytes(used));
    println!("  real = {} ({})", real, human_bytes(real));
}

fn report_mem_linkedlist<T>(l: &LinkedList<T>) {
    let (used, real) = linkedlist_mem::<T>(l.len());
    println!("LinkedList: len = {}", l.len());
    println!("  used ≈ {} ({})", used, human_bytes(used));
    println!("  real ≈ {} ({})", real, human_bytes(real));
}

fn report_mem_linkedvector<T>(lv: &LinkedVector<T>) {
    let used = lv.mem_used();
    let real = lv.true_mem_used();
    println!(
        "LinkedVector: len = {}, true_len = {}, capacity = {}",
        lv.len(),
        lv.true_len(),
        lv.capacity()
    );
    println!("  used ≈ {} ({})", used, human_bytes(used));
    println!("  real ≈ {} ({})", real, human_bytes(real));
}

fn run_for_type_mystruct() {
    println!("==== Benchmark - Custom Struct ====");

    // prepare one big element
    let element = MyStruct::new(42);

    // Construction
    println!("\nConstruction (push_back) for {} elements: ", N_STRUCT);

    let (v, dur_v) = bench_construction_vec(N_STRUCT, element.clone());
    println!("Vec: construction took {:?}", dur_v);
    report_mem_vec(&v);

    let (ll, dur_ll) = bench_construction_linkedlist(N_STRUCT, element.clone());
    println!("LinkedList: construction took {:?}", dur_ll);
    report_mem_linkedlist(&ll);

    let (lv, dur_lv) = bench_construction_linkedvector(N_STRUCT, element.clone());
    println!("LinkedVector: construction took {:?}", dur_lv);
    report_mem_linkedvector(&lv);

    // Random access (midpoint)
    println!("\nRandom access at midpoint: ");

    let mid_v = v.len() / 2;
    let (val_v, dur_v_access) = bench_random_access_vec(&v);
    println!(
        "Vec: random access (index {}) took {:?}; nums[0] = {:?}",
        mid_v,
        dur_v_access,
        val_v.unwrap().nums[0]
    );

    let mid_ll = ll.len() / 2;
    let (val_ll, dur_ll_access) = bench_random_access_linkedlist(&ll);
    println!(
        "LinkedList: random access (iter to {}) took {:?}; item = {:?}",
        mid_ll,
        dur_ll_access,
        val_ll.unwrap().item
    );

    let mid_lv = lv.len() / 2;
    let (val_lv, dur_lv_access) = bench_random_access_linkedvector(&lv);
    println!(
        "LinkedVector: random access (index {}) took {:?}; extra = {:?}",
        mid_lv,
        dur_lv_access,
        val_lv.unwrap().extra
    );

    // Deletion (midpoint)
    println!("\nDeletion at midpoint:");

    let (v_after_del, dur_v_del) = bench_delete_vec(v, mid_v);
    println!("Vec: remove(mid) took {:?}", dur_v_del);
    report_mem_vec(&v_after_del);

    let (ll_after_del, dur_ll_del) = bench_delete_linkedlist(ll, mid_ll);
    println!(
        "LinkedList: delete(mid) via split_off took {:?}",
        dur_ll_del
    );
    report_mem_linkedlist(&ll_after_del);

    let (lv_after_del, dur_lv_del) = bench_delete_linkedvector(lv, mid_lv);
    println!("LinkedVector: delete(mid) took {:?}", dur_lv_del);
    report_mem_linkedvector(&lv_after_del);

    println!("\n\n")
}

fn run_for_type_u64() {
    println!("==== Benchmark - u64 ====");

    let element: u64 = 0xDEADBEEFDEADBEEF_u64;

    // Construction
    println!("\nConstruction (push_back) for {} elements:", N_PRIMITIVE);

    let (v, dur_v) = bench_construction_vec(N_PRIMITIVE, element);
    println!("Vec: construction took {:?}", dur_v);
    report_mem_vec(&v);

    let (ll, dur_ll) = bench_construction_linkedlist(N_PRIMITIVE, element);
    println!("LinkedList: construction took {:?}", dur_ll);
    report_mem_linkedlist(&ll);

    let (lv, dur_lv) = bench_construction_linkedvector(N_PRIMITIVE, element);
    println!("LinkedVector: construction took {:?}", dur_lv);
    report_mem_linkedvector(&lv);

    // Random access (midpoint)
    println!("\nRandom access at midpoint: ");

    let mid_v = v.len() / 2;
    let (val_v, dur_v_access) = bench_random_access_vec(&v);
    println!(
        "Vec: random access (index {}) took {:?}; value: {}",
        mid_v,
        dur_v_access,
        val_v.unwrap()
    );

    let mid_ll = ll.len() / 2;
    let (val_ll, dur_ll_access) = bench_random_access_linkedlist(&ll);
    println!(
        "LinkedList: random access (iter to {}) took {:?}; value: {}",
        mid_ll,
        dur_ll_access,
        val_ll.unwrap()
    );

    let mid_lv = lv.len() / 2;
    let (val_lv, dur_lv_access) = bench_random_access_linkedvector(&lv);
    println!(
        "LinkedVector: random access (index {}) took {:?}; value: {}",
        mid_lv,
        dur_lv_access,
        val_lv.unwrap()
    );

    // Deletion (midpoint)
    println!("\nDeletion at midpoint: ");

    let (_v_after_del, dur_v_del) = bench_delete_vec(v, mid_v);
    println!("Vec: remove(mid) took {:?}", dur_v_del);

    let (_ll_after_del, dur_ll_del) = bench_delete_linkedlist(ll, mid_ll);
    println!(
        "LinkedList: delete(mid) via split_off took {:?}",
        dur_ll_del
    );

    let (_lv_after_del, dur_lv_del) = bench_delete_linkedvector(lv, mid_lv);
    println!("LinkedVector: delete(mid) took {:?}", dur_lv_del);

    println!("\n\n");
}

fn main() {
    println!("Starting benches.");
    println!(
        "N_STRUCT = {}, N_PRIMITIVE = {}, size_of(MyStruct) = {}",
        N_STRUCT,
        N_PRIMITIVE,
        human_bytes(size_of::<MyStruct>())
    );

    run_for_type_mystruct();
    run_for_type_u64();

    println!("Done.\n\n\n");
}
