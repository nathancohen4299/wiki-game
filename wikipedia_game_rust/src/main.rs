use crate::page::Page;
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

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    id: String,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.id.cmp(&other.id))
    }
}
impl PartialOrd for State{
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_shortest_path(source: &'static str, destination: &'static str) -> Option<u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    let mut pages: HashMap<&str, Page> = HashMap::new();
    let mut min_heap = BinaryHeap::new();
    dist.insert(String::from(source), 0);
    min_heap.push(State {
        id: source.to_string(),
        cost: 0,
    });

    while let Some(State { id, cost }) = min_heap.pop() {
        if id == destination {
            return Some(cost);
        }

        if let Some(distance) = dist.get(id) {
            if cost > *distance {
                continue;
            }
        }
        let current_page: Page = Page::new(id);
        pages.insert(id, current_page);
        for s in pages.get(id).unwrap().links {
            let next = State {
                id: s.clone(),
                cost: cost + 1,
            };
            if dist.contains_key(s.as_str()) {
                let next_distance = *dist.get(s.as_str()).unwrap();
                if next.cost < next_distance {
                    *dist.get_mut(next.id.as_str()).unwrap() = next.cost.clone();
                    min_heap.push(next);
                }
            } else {
                dist.insert(next.id.to_string(), next.cost.clone());
                min_heap.push(next);
            }
        }
    }
    None
}
