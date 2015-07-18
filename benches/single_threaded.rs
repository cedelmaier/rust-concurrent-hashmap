#![feature(test)]
#![feature(std_misc)]
#![feature(hashmap_hasher)]

extern crate rand;
extern crate test;
extern crate concurrent_hashmap;

use std::default::Default;
use std::cmp::max;
use test::Bencher;
use rand::{Rng, weak_rng};
use concurrent_hashmap::*;

const INTEGERS: u32 = 100_000;

macro_rules! new_map (
    ($typ: ty) => ({
        let mut options: Options<::std::collections::hash_map::RandomState> = Default::default();
        options.concurrency = 1;
        ConcHashMap::<$typ, i8, _>::with_options(options)
    })
);

#[bench]
#[inline(never)]
fn insert_sequential_integers(b: &mut Bencher) {
    b.iter(|| {
        let map = new_map!(u32);
        for i in 0..INTEGERS {
            map.insert(i, 0);
        }
        map
    });
    b.bytes = INTEGERS as u64;
}

#[bench]
#[inline(never)]
fn insert_random_integers(b: &mut Bencher) {
    let mut integers: Vec<_> = (0..INTEGERS).collect();
    weak_rng().shuffle(&mut integers);
    b.iter(|| {
        let map = new_map!(u32);
        for &i in integers.iter() {
            map.insert(i, 0);
        }
        map
    });
    b.bytes = INTEGERS as u64;
}

#[bench]
#[inline(never)]
fn insert_sequential_strings(b: &mut Bencher) {
    let strings: Vec<_> = (0..INTEGERS).map(|i| i.to_string()).collect();
    b.iter(|| {
        let map = new_map!(&str);
        for i in strings.iter() {
            map.insert(i, 0);
        }
        map
    });
    b.bytes = INTEGERS as u64;
}

#[bench]
#[inline(never)]
fn insert_random_strings(b: &mut Bencher) {
    let mut strings: Vec<_> = (0..INTEGERS).map(|i| i.to_string()).collect();
    weak_rng().shuffle(&mut strings);
    b.iter(|| {
        let map = new_map!(&str);
        for i in strings.iter() {
            map.insert(i, 0);
        }
        map
    });
    b.bytes = INTEGERS as u64;
}

// TODO Replace these with a macro when #12249 is solved
#[bench]
#[inline(never)]
fn random_integer_lookup_100(b: &mut Bencher) {
    random_integer_lookup(100.0, b);
}

#[bench]
#[inline(never)]
fn random_integer_lookup_95(b: &mut Bencher) {
    random_integer_lookup(95.0, b);
}

#[bench]
#[inline(never)]
fn random_integer_lookup_50(b: &mut Bencher) {
    random_integer_lookup(50.0, b);
}

#[bench]
#[inline(never)]
fn random_integer_lookup_5(b: &mut Bencher) {
    random_integer_lookup(5.0, b);
}

#[bench]
#[inline(never)]
fn random_integer_lookup_0(b: &mut Bencher) {
    random_integer_lookup(0.0, b);
}

fn random_integer_lookup(hit_rate: f64, b: &mut Bencher) {
    let mut rng = weak_rng();
    let map = new_map!(u32);
    for i in 0..INTEGERS {
        map.insert(i, 0);
    }
    let base_n = 1000;
    let n = max(1, base_n - (0.99 * base_n as f64 * (1.0 - hit_rate / 100.0)) as u32);
    let (min, max) = if hit_rate > 0.0 {
        (0, (INTEGERS as f64 / (hit_rate / 100.0)) as u32)
    } else {
        (INTEGERS, 2 * INTEGERS)
    };
    let keys: Vec<_> = (0..n).map(|_| rng.gen_range(min, max)).collect();
    b.iter(||
        for key in keys.iter() {
            test::black_box(map.find(key));
        }
    );
    b.bytes = n as u64 as u64;
}

#[ignore]
#[bench]
#[inline(never)]
fn random_integer_lookup_100_std(b: &mut Bencher) {
    random_integer_lookup_std(100.0, b);
}

#[ignore]
#[bench]
#[inline(never)]
fn random_integer_lookup_95_std(b: &mut Bencher) {
    random_integer_lookup_std(95.0, b);
}

#[ignore]
#[bench]
#[inline(never)]
fn random_integer_lookup_50_std(b: &mut Bencher) {
    random_integer_lookup_std(50.0, b);
}

#[ignore]
#[bench]
#[inline(never)]
fn random_integer_lookup_5_std(b: &mut Bencher) {
    random_integer_lookup_std(5.0, b);
}

#[ignore]
#[bench]
#[inline(never)]
fn random_integer_lookup_0_std(b: &mut Bencher) {
    random_integer_lookup_std(0.0, b);
}

fn random_integer_lookup_std(hit_rate: f64, b: &mut Bencher) {
    let mut rng = weak_rng();
    let mut map = ::std::collections::HashMap::new();
    for i in 0..INTEGERS {
        map.insert(i, 0);
    }
    let base_n = 1000;
    let n = max(1, base_n - (0.99 * base_n as f64 * (1.0 - hit_rate / 100.0)) as u32);
    let (min, max) = if hit_rate > 0.0 {
        (0, (INTEGERS as f64 / (hit_rate / 100.0)) as u32)
    } else {
        (INTEGERS, 2 * INTEGERS)
    };
    let keys: Vec<_> = (0..n).map(|_| rng.gen_range(min, max)).collect();
    b.iter(||
        for key in keys.iter() {
            test::black_box(map.get(key));
        }
    );
    b.bytes = n as u64 as u64;
}
