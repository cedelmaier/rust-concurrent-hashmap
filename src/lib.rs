#![feature(std_misc)]
#![feature(alloc)]

#![feature(heap_api)]
#![feature(oom)]
#![feature(hashmap_hasher)]

extern crate alloc;
extern crate num;

mod table;
mod map;

pub use map::*;
pub use table::Accessor;
