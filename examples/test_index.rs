extern crate rand;
extern crate time;
extern crate getopts;
extern crate timely;
extern crate graph_map;
extern crate differential_dataflow;

use std::collections::HashMap;

use rand::{Rng, SeedableRng, StdRng};

use differential_dataflow::collection_trace::index::{Index, OrdIndex};

fn main() {
    for i in 25..28 {
        test_index(i);
    }
}

fn test_index(log_size: usize) {

    let mut index = OrdIndex::new();
    let mut hash = HashMap::new();

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut tally = 0;

    let mut vec = vec![];
    let mut remaining = 1 << log_size;
    while remaining > 0 {
        for _ in 0.. (1 << (log_size - 4)) {
            vec.push(((rng.next_u64()),(rng.next_u64())));
        }
        vec.sort();
        remaining -= vec.len();
        let start = time::precise_time_ns();
        index.for_each(&mut vec, |k,v| { *v = *k });
        tally += time::precise_time_ns() - start;
        vec.clear();
    }

    println!("loaded index 2e{}:\t{:.2e} ns", log_size, tally as f64);

    let start = time::precise_time_ns();

    for i in 0..(1 << log_size) {
        hash.insert((i,i),(i,i));
    }

    println!("loaded hash 2e{}:\t{:.2e} ns", log_size, (time::precise_time_ns() - start) as f64);

    for query in 0..log_size {

        let mut index_tally = 0;
        let mut hash_tally = 0;

        for _ in 0..10 {
            for _ in 0..(1 << query) {
                vec.push(((rng.next_u64()),(rng.next_u64())));
            }
            vec.sort();
            let start = time::precise_time_ns();
            index.find_each(&mut vec, |k,v| { *v = *k });
            vec.clear();
            index_tally += time::precise_time_ns() - start;

            let start = time::precise_time_ns();
            for i in 0..(1 << query) {
                assert!(hash.get(&(i,i)).unwrap() == &(i,i));
            }
            hash_tally += time::precise_time_ns() - start;

        }

        println!("\t2e{}:\t{}\t{:2.3}x\t{} ns", query, (index_tally / 10) >> query, index_tally as f64 / hash_tally as f64, (index_tally as i64 - hash_tally as i64) / 10);
    }
}
