use std::collections::{HashMap, HashSet};
use std::iter::Cycle;
use std::str::Chars;

use regex::Regex;

pub struct Routes {
    nodes: HashMap<String, (String, String)>,
}

impl Routes {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn get(&self, start: &str) -> Option<&(String, String)> {
        self.nodes.get(start)
    }

    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.keys().map(|s| s.to_owned()).collect()
    }

    pub fn insert(&mut self, start: &str, left: &str, right: &str) {
        self.nodes.insert(start.to_owned(), (left.to_owned(), right.to_owned()));
    }

    pub fn insert_str(&mut self, s: &str) {
        let re = Regex::new(r"^([[:alnum:]]+) = \(([[:alnum:]]+), ([[:alnum:]]+)\)$")
            .expect("Failed to compile regex");
        let captures = re.captures(s)
            .expect("Line does not match route regex");
        self.insert(&captures[1], &captures[2], &captures[3]);
    }

    pub fn trace(&self, path: &str) -> usize {
        let mut steps = 0;
        let mut position = "AAA";
        let mut path = path.chars().cycle();
        while position != "ZZZ" {
            let (left, right) = self.get(position)
                .expect("Nowhere to go");
            let c = path.next().unwrap();
            if c == 'L' {
                position = left;
            } else {
                position = right;
            }
            steps += 1;
        }
        steps
    }
}

pub struct Path <'a> {
    routes: &'a Routes,
    path: Cycle<Chars<'a>>,
    path_len: usize,
    position: &'a str,
}

impl <'a> Path <'a> {
    pub fn new(routes: &'a Routes, path: &'a str, start: &'a str) -> Self {
        Self { routes, path: path.chars().cycle(), path_len: path.len(), position: start }
    }

    pub fn step(&mut self) -> &str {
        let c = self.path.next().unwrap();
        let (left, right) = self.routes.get(self.position)
            .expect("Nowhere to go");
        self.position = if c == 'L' { left } else { right };
        self.position
    }

    pub fn steps_to_z(&mut self) -> Vec<usize> {
        // Find all distances to end nodes until a loop is detected, where a loop
        // is defined as reaching the same node at the same offset into the path
        let mut zs = Vec::new();
        let mut steps = 0;
        let mut seen: HashSet<(usize, String)> = HashSet::new();
        let path_len = self.path_len;
        seen.insert((0, self.position.to_owned()));
        loop {
            let position = self.step().to_owned();
            steps += 1;
            if position.ends_with("Z") {
                zs.push(steps);
            }
            let offset = steps % path_len;
            if !seen.insert((offset, position)) {
                break;
            }
        }
        zs
    }
}
