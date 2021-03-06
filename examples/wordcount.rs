#![feature(scoped)]

extern crate concurrent_hashmap;

use std::cmp::max;
use std::io::Read;
use std::io;
use std::thread;
use std::default::Default;
use concurrent_hashmap::*;

fn main() {
    let words = read_words();
    let word_counts: ConcHashMap<String, u32> = Default::default();
    count_words(&words, &word_counts, 4);
    let mut counts: Vec<(String, u32)> = word_counts.iter().map(|(s, &n)| (s.clone(), n)).collect();
    counts.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    for &(ref word, count) in counts.iter() {
        println!("{}\t{}", word, count);
    }
}

fn read_words() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.split_whitespace()
        .map(|w| w.trim_matches(|c| ['.', '"', ':', ';', ',', '!', '?', ')', '(', '_']
                  .contains(&c)))
        .map(|w| w.to_lowercase())
        .filter(|w| !w.is_empty())
        .collect()
}

fn count_words(words: &[String], word_counts: &ConcHashMap<String, u32>, nthreads: usize) {
    let mut threads = Vec::with_capacity(nthreads);
    for chunk in words.chunks(max(10, words.len() / nthreads)) {
        threads.push(thread::scoped(move || {
            for word in chunk.iter() {
                // It would be nice to be able to pass a &K to .upsert()
                // and have it clone as needed instead of passing a K.
                word_counts.upsert(word.to_owned(), 1, &|count| *count += 1);
            }
        }));
    }
}
