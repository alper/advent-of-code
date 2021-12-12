use itertools::Itertools;
use petgraph::algo;
use petgraph::graph::Node;
use petgraph::prelude::*;
use std::collections::HashMap;
use std::fs;

// type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    let graph = parse_graph(&input);

    // let mut paths: Vec<Vec<&str>> = Vec::new();
    // depth_first(vec!["A"], vec![], &mut paths, &graph);

    println!("Input: {:?}", graph);

    let start = graph.node_indices().find(|i| graph[*i] == "start").unwrap();
    let end = graph.node_indices().find(|i| graph[*i] == "end").unwrap();

    println!("Start: {:?} end: {:?}", start, end);

    let ways = algo::all_simple_paths::<Vec<_>, _>(&graph, start, end, 0, None).collect::<Vec<_>>();

    println!("Ways: {:?}", ways.len());
}

fn parse_graph(inp: &str) -> UnGraph<&str, ()> {
    let parsed: Vec<(_, _)> = inp
        .lines()
        .map(|l| (l.split('-').next().unwrap(), l.split('-').nth(1).unwrap()))
        .collect();

    // let mut graph: Graph = HashMap::new();

    // for el in parsed {
    //     let dests = graph.entry(el.0).or_insert(vec![]);
    //     dests.push(el.1);
    // }

    // graph

    let mut g = UnGraph::<&str, ()>::default();

    let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();

    // // Nodes
    // let nodes: Vec<&str> = parsed.iter().map(|(f, t)| *f).unique().collect();
    // for node in nodes {
    //     if !node_map.contains_key(node)
    //     let ni = g.add_node(node);

    //     node_map.insert(node, ni.index());
    // }

    for edge in parsed {
        if !node_map.contains_key(edge.0) {
            let ni = g.add_node(edge.0);
            node_map.insert(edge.0, ni);
        }

        if !node_map.contains_key(edge.1) {
            let ni = g.add_node(edge.1);
            node_map.insert(edge.1, ni);
        }

        let from_index = *node_map.get(edge.0).unwrap();
        let to_index = *node_map.get(edge.1).unwrap();

        g.add_edge(from_index, to_index, ());
    }

    g
}

// fn depth_first(path: Vec<&str>, visited: Vec<&str>, paths: &mut Vec<Vec<&str>>, graph: &Graph) {
//     // Do logic
//     let last_node = *path.last().unwrap_or(&&"");

//     if last_node != "end" {
//         let dests = graph.get(last_node).unwrap();

//         for dest in dests {
//             if !visited.contains(dest) {
//                 let new_path = path.clone();
//                 new_path.push(dest);

//                 let new_visited = visited.clone();
//                 if !dest.chars().all(|c| c.is_uppercase()) {
//                     new_visited.push(dest);
//                 }

//                 depth_first(new_path, new_visited, paths, graph)
//             }
//         }
//     } else {
//         paths.push(path);
//     }
// }
