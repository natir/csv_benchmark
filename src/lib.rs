extern crate csv;
extern crate itertools;
extern crate rayon;

#[macro_use]
extern crate serde_derive;

use std::io::BufRead;
use itertools::Itertools;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

mod paf;
mod utils;

use paf::Reader;
use utils::*;

pub fn basic() {
    let mut result: Read2Mapping = std::collections::HashMap::new();
    let file = std::io::BufReader::new(std::fs::File::open("test.csv").unwrap());
    
    for r in Reader::new(file).records() {
        let record = r.unwrap();

        let key_a = NameLen {
            name: record.read_a,
            len: record.length_a,
        };
        let val_a = Interval {
            begin: record.begin_a,
            end: record.end_a,
        };

        let key_b = NameLen {
            name: record.read_b,
            len: record.length_b,
        };
        let val_b = Interval {
            begin: record.begin_b,
            end: record.end_b,
        };

        result.entry(key_a).or_insert(Vec::new()).push(val_a);
        result.entry(key_b).or_insert(Vec::new()).push(val_b);
    }
}

pub fn mutex(nb_record: usize) {
    let result: Arc<Mutex<Read2Mapping>> = Arc::new(Mutex::new(HashMap::new()));
    let file = std::io::BufReader::new(std::fs::File::open("test.csv").unwrap());

    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();

    pool.install(|| {
        for chunk in file.lines().chunks(nb_record*1).into_iter() {
            let result = Arc::clone(&result);
            let buffer = chunk.map(|x| x.unwrap()).collect::<Vec<String>>().concat().into_bytes();
            rayon::spawn(move || {
                for r in Reader::new(buffer.as_slice()).records() {
                    let record = r.unwrap();

                    let key_a = NameLen {
                        name: record.read_a,
                        len: record.length_a,
                    };
                    let val_a = Interval {
                        begin: record.begin_a,
                        end: record.end_a,
                    };

                    let key_b = NameLen {
                        name: record.read_b,
                        len: record.length_b,
                    };
                    let val_b = Interval {
                        begin: record.begin_b,
                        end: record.end_b,
                    };

                    {
                        let mut r = result.lock().unwrap();
                        r.entry(key_a).or_insert(Vec::new()).push(val_a);
                        r.entry(key_b).or_insert(Vec::new()).push(val_b);
                    }
                }
            });
        }
    });
}

use std::sync::mpsc;

fn message(nb_record: usize) {
    let mut result: Read2Mapping = HashMap::new();
    let file = std::io::BufReader::new(std::fs::File::open("test.csv").unwrap());
    let (sender, receiver) = mpsc::sync_channel(nb_record);
    
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    
    pool.install(|| {
        for chunk in file.lines().chunks(nb_record*1).into_iter() {
            let buffer = chunk.map(|x| x.unwrap()).collect::<Vec<String>>().concat().into_bytes();
            let sender = sender.clone();
            rayon::spawn(move || {
                for r in Reader::new(buffer.as_slice()).records() {
                    let record = r.unwrap();

                    let key_a = NameLen {
                        name: record.read_a,
                        len: record.length_a,
                    };
                    let val_a = Interval {
                        begin: record.begin_a,
                        end: record.end_a,
                    };

                    let key_b = NameLen {
                        name: record.read_b,
                        len: record.length_b,
                    };
                    let val_b = Interval {
                        begin: record.begin_b,
                        end: record.end_b,
                    };

                    sender.send((key_a, val_a)).unwrap();
                }
            });
        }
    });

    for (k, v) in receiver.iter() {
        result.entry(k).or_insert(Vec::new()).push(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        basic();
        mutex(10);
        assert_eq!(2 + 2, 4);
    }
}
