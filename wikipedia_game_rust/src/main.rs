use crate::page::Page;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

mod page;

fn main() {
    let source = "/wiki/Make_Your_Wish";
    let dest = "/wiki/Oh_Ji-eun";
    if let Some(value) = calculate_shortest_path(source, dest) {
        println!("Shortest Path of Cost {}", value);
    } else {
        println!("Error: Shortest Path Not Found");
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    cost: u32,
    id: &'a str,
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.id.cmp(&other.id))
    }
}
impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_shortest_path(source: &'static str, destination: &'static str) -> Option<u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    let mut min_heap = BinaryHeap::new();
    dist.insert(source, 0);
    min_heap.push(State {
        id: source,
        cost: 0,
    });

    while let Some(State { id, cost }) = min_heap.pop() {
        if id == destination {
            return Some(cost);
        }

        if let Some(distance) = dist.get(id) {
            if cost > *distance {
                continue;
            }g
        }
        let current_page: Page = Page::new(source);
        for s in current_page.links {
            let next = State {
                id: s.clone().as_str(),
                cost: cost + 1,
            };
            if dist.contains_key(s.as_str()) {
                let next_distance = *dist.get(s.as_str()).unwrap();
                if next.cost < next_distance {
                    *dist.get_mut(next.id).unwrap() = next.cost.clone();
                    min_heap.push(next);
                }
            } else {
                dist.insert(next.id, next.cost.clone());
                min_heap.push(next);
            }
        }
    }
    None
}
