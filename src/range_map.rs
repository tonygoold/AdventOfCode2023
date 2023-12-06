use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidRange,
    InternalError,
}

pub struct RangeMap {
    source: usize,
    dest: usize,
    len: usize,
}

impl RangeMap {
    pub fn new(source: usize, dest: usize, len: usize) -> Self {
        Self { source, dest, len }
    }

    pub fn map(&self, n: usize) -> Option<usize> {
        if (self.source..self.source+self.len).contains(&n) {
            Some(self.dest + n - self.source)
        } else {
            None
        }
    }
}

impl FromStr for RangeMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+) (\d+) (\d+)")
            .map_err(|_| ParseError::InternalError)?;
        let captures = re.captures(s)
            .ok_or(ParseError::InvalidRange)?;
        let dest: usize = captures[1].parse().map_err(|_| ParseError::InvalidRange)?;
        let source: usize = captures[2].parse().map_err(|_| ParseError::InvalidRange)?;
        let len: usize = captures[3].parse().map_err(|_| ParseError::InvalidRange)?;
        Ok(RangeMap::new(source, dest, len))
    }
}

pub struct RangeMapSet {
    maps: Vec<RangeMap>,
}

impl RangeMapSet {
    pub fn new() -> Self {
        Self { maps: Vec::new() }
    }

    pub fn add(&mut self, map: RangeMap) {
        self.maps.push(map);
        self.maps.sort_unstable_by_key(|map| map.source);
    }

    pub fn map(&self, n: usize) -> usize {
        let map = self.maps.binary_search_by(|map| {
            if n < map.source {
                Ordering::Greater
            } else if n >= map.source + map.len {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        map.map_or(n, |i| self.maps[i].map(n).expect("Should be in range"))
    }
}

pub struct RangeMapChain {
    sets: HashMap<String, RangeMapSet>,
    chains: HashMap<String, String>,
}

impl RangeMapChain {
    pub fn new() -> Self {
        Self { sets: HashMap::new(), chains: HashMap::new() }
    }

    pub fn add(&mut self, source: &str, dest: &str, map: RangeMap) {
        self.chains.insert(source.to_owned(), dest.to_owned());
        let set = self.sets.entry(source.to_owned())
            .or_insert(RangeMapSet::new());
        set.add(map);
    }

    pub fn map(&self, source: &str, n: usize) -> usize {
        let mut n = n;
        let mut source = source;
        while let Some(dest) = self.chains.get(source) {
            let set = self.sets.get(source)
                .expect("Unknown source");
            n = set.map(n);
            source = dest;
        }
        n
    }

    pub fn map_range(&self, source: &str, ns: &Range<usize>) -> Vec<usize> {
        let mut ns: Vec<usize> = ns.clone().collect();
        let mut source = source;
        while let Some(dest) = self.chains.get(source) {
            let set = self.sets.get(source)
                .expect("Unknown source");
            ns = ns.iter().map(|n| set.map(*n)).collect();
            source = dest;
        }
        ns
    }
}
