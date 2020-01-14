use crate::page::Page;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

mod page;

fn main() {
    let source = "/wiki/Pulp_Fiction";
    let dest = "/wiki/Evil_Dead_II";
    if let Some(order) = calculate_shortest_path(source, dest) {
        println!("Path:");
        for x in order {
            println!("{}", x);
        }
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

fn calculate_shortest_path(
    source: &'static str,
    destination: &'static str,
) -> Option<Vec<Box<String>>> {
    let mut dist: HashMap<u64, u32> = HashMap::new();
    let mut pages: HashMap<u64, Page> = HashMap::new();
    let mut edges: HashMap<String, String> = HashMap::new();
    let mut min_heap = BinaryHeap::new();
    let mut count = 1;
    let source_page = Page::new(source);
    dist.insert(source_page.calculate_hash(), 0);
    min_heap.push(State {
        id: source_page.calculate_hash(),
        cost: 0,
    });
    pages.insert(source_page.calculate_hash(), source_page);

    while let Some(State { id, cost }) = min_heap.pop() {
        if pages.get(&id).unwrap().path.as_str() == destination {
            return Some(build_path(source, destination, &edges));
        }

        if let Some(x) = pages.get(&id) {
            println!("{}: Scraping {}", count, x.path);
            count += 1;
        }

        if let Some(distance) = dist.get(&id) {
            if cost > *distance {
                continue;
            }
        }

        if pages.get(&id).is_some() {
            let links = pages.get(&id).unwrap().get_urls();
            for s in links {
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
                        //set edge
                        *edges
                            .get_mut(pages.get(&next.id).unwrap().path.clone().as_str())
                            .unwrap() = pages.get(&id).unwrap().path.clone();
                        min_heap.push(next);
                    }
                } else {
                    dist.insert(next.id, next.cost.clone());
                    edges.insert(
                        pages.get(&next.id).unwrap().path.clone(),
                        pages.get(&id).unwrap().path.clone(),
                    );
                    min_heap.push(next);
                }
            }
        }
    }
    None
}

fn build_path(
    source: &str,
    destination: &str,
    edges: &HashMap<String, String>,
) -> Vec<Box<String>> {
    let mut order: Vec<Box<String>> = Vec::new();
    order.push(Box::new(destination.to_string()));
    while edges.contains_key(order[order.len()-1].as_str()) {
        let next = edges.get(order[order.len() - 1].as_str()).unwrap().clone();
        order.push(Box::new(next));
        if order[order.len() - 1].as_str() == source {
            order.reverse();
            return order;
        }
    }
    order
}
