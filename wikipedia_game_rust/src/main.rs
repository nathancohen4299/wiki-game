mod shortest_path;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("USAGE: {} <source> <destination>", args[0]);
        exit(1);
    }
    if let Some(order) = shortest_path::calculate_shortest_path(args[1].as_str(), args[2].as_str())
    {
        println!("Shortest Path:");
        for x in order {
            println!("{}", x);
        }
    } else {
        println!("Error: Shortest Path Not Found");
    }
}
