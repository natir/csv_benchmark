use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;


#[derive(Debug)]
pub struct NameLen {
    pub name: String,
    pub len: u64,
}

impl PartialEq for NameLen {
    fn eq(&self, other: &NameLen) -> bool {
        self.name == other.name
    }
}

impl Eq for NameLen {}

impl Hash for NameLen {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct Interval {
    pub begin: u64,
    pub end: u64,
}

impl Ord for Interval {
    fn cmp(&self, other: &Interval) -> Ordering {
        let r = self.begin.cmp(&other.begin);

        return match r {
            Ordering::Equal => self.end.cmp(&other.end),
            _ => r,
        };
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Interval) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Interval) -> bool {
        self.begin == other.begin && self.end == other.end
    }
}

impl Eq for Interval {}


pub type Read2Mapping =  HashMap<NameLen, Vec<Interval>>;

