extern crate regex;
extern crate petgraph;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

use petgraph::prelude::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");

    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = Graph::<u32, u32>::new();

    for line in input.split('\n') {
        println!("Record: {}", line);

        let (container, colors) = parse_line(line);

        let node = match nodes.get(container.as_str()) {
            Some(n) => *n,
            None => {
                let ni = graph.add_node(0);
                nodes.insert(container, ni);
                ni
            }
        };

        for col in colors {
            let contained_node = match nodes.get(col.as_str()) {
                Some(n) => *n,
                None => {
                    let ni = graph.add_node(0);
                    nodes.insert(col, ni);
                    ni
                }
            };

            graph.add_edge(node, contained_node, 0);
        }
    }

    println!("Generated graph: {:?}", graph);
}

fn parse_line(line: &str) -> (String, Vec<String>) {
    let re = Regex::new(r"(.+?) bags contain(.+?)\.").unwrap();

    let mut color = String::from("");
    let mut contained_colors = vec![];

    if let Some(caps) = re.captures(line) {
        if let (Some(container_bag_color), Some(contained_bags)) = (caps.get(1), caps.get(2)) {
            color = container_bag_color.as_str().to_owned();

            let re_contained_single = Regex::new("1 (.+?) bag").unwrap();
            for cap in re_contained_single.captures_iter(contained_bags.as_str()) {
                contained_colors.push(cap[1].to_owned());
            }

            let re_contained_several = Regex::new(r"(\d+) (.+?) bags").unwrap();
            for cap in re_contained_several.captures_iter(contained_bags.as_str()) {
                contained_colors.push(cap[2].to_owned());
            }
        }
    }

    println!("Color: {}, contains {:?}", color, contained_colors);

    (color, contained_colors)
}