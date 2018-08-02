extern crate csv;
extern crate itertools;
extern crate rayon;

#[macro_use]
extern crate serde_derive;

use std::io::BufRead;
use itertools::Itertools;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::mpsc;

mod paf;
mod utils;

use paf::Reader;
use utils::*;

pub fn basic(filename: &str) -> Read2Mapping {
    let mut result: Read2Mapping = std::collections::HashMap::new();
    let file = std::io::BufReader::new(std::fs::File::open(filename).unwrap());
    
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

    return result;
}

pub fn mutex(filename: &str, nb_record: usize, nb_thread: usize) -> Read2Mapping {
    let result = Arc::new(Mutex::new(HashMap::new()));
    let file = std::io::BufReader::new(std::fs::File::open(filename).unwrap());

    let pool = rayon::ThreadPoolBuilder::new().num_threads(nb_thread).build().unwrap();

    pool.install(|| {
        rayon::scope(|s| {
            for chunk in file.lines().chunks(nb_record*1).into_iter() {
                let result_ = Arc::clone(&result);
                let buffer = chunk.map(|x| x.unwrap()).collect::<Vec<String>>().join("\n").into_bytes();
                
                s.spawn(move |_| {
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
                            let mut re = result_.lock().unwrap();
                            re.entry(key_a).or_insert(Vec::new()).push(val_a);
                            re.entry(key_b).or_insert(Vec::new()).push(val_b);
                        }
                    }
                });
            }
        })
    });

    let lock = Arc::try_unwrap(result).expect("Lock still has multiple owners");
    lock.into_inner().expect("Mutex cannot be locked")
}


pub fn message(filename: &str, nb_record: usize, nb_thread: usize) -> Read2Mapping {
    let mut result: Read2Mapping = HashMap::new();
    
    let file = std::io::BufReader::new(std::fs::File::open(filename).unwrap());
    let (sender, receiver) = mpsc::channel();
    
    let pool = rayon::ThreadPoolBuilder::new().num_threads(nb_thread).build().unwrap();
   
    pool.install(|| {
        rayon::scope(|s| {
            for chunk in file.lines().chunks(nb_record*1).into_iter() {
                let buffer = chunk.map(|x| x.unwrap()).collect::<Vec<String>>().join("\n").into_bytes();
                let sender = sender.clone();
                s.spawn(move |_| {
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

                        sender.send((Some(key_a), Some(val_a))).unwrap();
                        sender.send((Some(key_b), Some(val_b))).unwrap();
                    }
                });
            }
        drop(sender);
        })
    });

    for (k, v) in receiver.iter() {
        result.entry(k.unwrap()).or_insert(Vec::new()).push(v.unwrap());
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ba = basic("1.paf");
        let mut me = message("1.paf", 128, 4);
        let mut mu = mutex("1.paf", 128, 4);
        
        {
            let mut ba_key = ba.keys().collect::<Vec<&NameLen>>(); 
            let mut me_key = me.keys().collect::<Vec<&NameLen>>();
            let mut mu_key = mu.keys().collect::<Vec<&NameLen>>();
            
            ba_key.sort();
            me_key.sort();
            mu_key.sort();
                
            assert_eq!(ba_key, me_key);
            assert_eq!(me_key, mu_key);
        }
        
        for (k, v) in ba {
            let mut a = v;
            let mut b = me.get_mut(&k).unwrap();
            let mut c = mu.get_mut(&k).unwrap();

            a.sort();
            b.sort();
            c.sort();

            assert_eq!(&mut a, b);
            assert_eq!(b, c);
        }
    }
}
