extern crate regex;
extern crate petgraph;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

use petgraph::prelude::*;
use petgraph::algo::has_path_connecting;
use petgraph::dot::Dot;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");

    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = Graph::<String, u32>::new();

    for line in input.split('\n') {
        println!("Input line: {}", line);

        let (container, colors) = parse_line(line);

        let node = match nodes.get(container.as_str()) {
            Some(n) => *n,
            None => {
                let ni = graph.add_node(container.clone());
                nodes.insert(container, ni);
                ni
            }
        };

        for (col, count) in colors {
            let contained_node = match nodes.get(col.as_str()) {
                Some(n) => *n,
                None => {
                    let ni = graph.add_node(col.clone());
                    nodes.insert(col, ni);
                    ni
                }
            };

            graph.add_edge(node, contained_node, count);
        }
    }

    println!("Generated graph: \n{:?}", Dot::new(&graph));

    let target = nodes.get(&String::from("shiny gold")).unwrap();

    let mut count = 0;
    for (_color, node) in nodes.iter() {
        if has_path_connecting(&graph, *node, *target, None) {
            count += 1;
        }
    }

    println!("Colors that can contain shiny gold: {}", count-1);

    println!("Total number of bags: {}", bag_count(&graph, *target)-1);
}

fn bag_count(graph: &Graph<String, u32>, start: NodeIndex) -> u32 {
    let mut count = 1;
    for e in graph.edges(start) {
        count += e.weight() * bag_count(graph, e.target())
    }
    count
}

fn parse_line(line: &str) -> (String, Vec<(String, u32)>) {
    let re = Regex::new(r"(.+?) bags contain(.+?)\.").unwrap();

    let mut color = String::from("");
    let mut contained_colors = vec![];

    if let Some(caps) = re.captures(line) {
        if let (Some(container_bag_color), Some(contained_bags)) = (caps.get(1), caps.get(2)) {
            color = container_bag_color.as_str().to_owned();

            let re_contained_single = Regex::new("1 (.+?) bag").unwrap();
            for cap in re_contained_single.captures_iter(contained_bags.as_str()) {
                contained_colors.push((cap[1].to_owned(), 1));
            }

            let re_contained_several = Regex::new(r"([2-9]|[1-9][0-9]) (.+?) bags").unwrap();
            for cap in re_contained_several.captures_iter(contained_bags.as_str()) {
                contained_colors.push((cap[2].to_owned(), cap[1].parse::<u32>().unwrap()));
            }
        }
    }

    println!("Color: /{}/, contains {:?}", color, contained_colors);

    (color, contained_colors)
}