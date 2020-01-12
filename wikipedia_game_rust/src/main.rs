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
    id: u64,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.id.cmp(&other.id))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_shortest_path(source: &'static str, destination: &'static str) -> Option<u32> {
    let mut dist: HashMap<u64, u32> = HashMap::new();
    let mut pages: HashMap<u64, Page> = HashMap::new();
    let mut min_heap = BinaryHeap::new();
    let source_page = Page::new(source);
    dist.insert(source_page.calculate_hash(), 0);
    min_heap.push(State {
        id: source_page.calculate_hash(),
        cost: 0,
    });
    pages.insert(source_page.calculate_hash(), source_page);

    while let Some(State { id, cost }) = min_heap.pop() {
        if pages.get(&id).unwrap().path.as_str() == destination {
            return Some(cost);
        }

        if let Some(distance) = dist.get(&id) {
            if cost > *distance {
                continue;
            }
        }
        println!("Current Page: id = {}, cost = {}, path = {}", id, cost, pages.get(&id).unwrap().path);
        if pages.get(&id).is_some() {
            for s in pages.get(&id).unwrap().links.clone() {
                println!("Looking at {}", s);
                let connecting_page = Page::new(s.clone().as_str());
                let next = State {
                    id: connecting_page.calculate_hash(),
                    cost: cost + 1,
                };
                pages.insert(connecting_page.calculate_hash(), connecting_page);

                if dist.contains_key(&next.id) {
                    let next_distance = *dist.get(&next.id).unwrap();
                    if next.cost < next_distance {
                        *dist.get_mut(&next.id).unwrap() = next.cost.clone();
                        min_heap.push(next);
                    }
                } else {
                    dist.insert(next.id, next.cost.clone());
                    min_heap.push(next);
                }
            }
        }
    }
    None
}
