use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{Ordering,Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn merge(&mut self, x: usize, y: usize) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                std::mem::swap(&mut root_x, &mut root_y);
            }

            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Keyed((i64, usize, usize)); // 包一层

impl Ord for Keyed {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0 .0.cmp(&other.0 .0)   // 只比较 tuple.0
    }
}

impl PartialOrd for Keyed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn part1(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let mut result: i64 = 0;
    let points : Vec<(i64, i64, i64)> = lines.iter().map(|line| {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        let z = parts.next().unwrap().parse::<i64>().unwrap();
        (x, y, z)
    }).collect();

    let mut union_find = UnionFind::new(points.len());

    let mut edges = BinaryHeap::new();

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let distance = (points[i].0 - points[j].0).pow(2) + (points[i].1 - points[j].1).pow(2) + (points[i].2 - points[j].2).pow(2);
            edges.push(Reverse(Keyed((distance, i, j))));
        }
    }

    for count in 0..1000 {
        let Some(Reverse(edge)) = edges.pop() else {
            break;
        };
        union_find.merge(edge.0.1, edge.0.2);
    }

    // Find the sizes of all sets in union_find, sort them, and get the largest 3
    let mut sizes = Vec::new();
    for i in 0..points.len() {
        if union_find.parent[i] == i {
            sizes.push(union_find.size[i]);
        }
    }
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // descending
    result = sizes[0] as i64 * sizes[1] as i64 * sizes[2] as i64;

    result
}

pub fn part2(input: &str) -> i64 {
    let file = File::open(input).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut result:i64 = 0;
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let points : Vec<(i64, i64, i64)> = lines.iter().map(|line| {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        let z = parts.next().unwrap().parse::<i64>().unwrap();
        (x, y, z)
    }).collect();

    let mut union_find = UnionFind::new(points.len());

    let mut edges = BinaryHeap::new();

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let distance = (points[i].0 - points[j].0).pow(2) + (points[i].1 - points[j].1).pow(2) + (points[i].2 - points[j].2).pow(2);
            edges.push(Reverse(Keyed((distance, i, j))));
        }
    }

    while true {
        let Some(Reverse(edge)) = edges.pop() else {
            break;
        };
        union_find.merge(edge.0.1, edge.0.2);
        let root = union_find.find(edge.0.1);
        if union_find.size[root] == points.len() {
            let point1 = points[edge.0.1];
            let point2 = points[edge.0.2];
            result = point1.0 * point2.0;
            break;
        }
    }
    result
}
