use std::collections::HashMap;
use std::fs;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let graph = parse_graph(&input);

    // println!("Graph: {:?}", graph);

    let mut path: Vec<String> = Vec::new();
    let mut visited: Vec<String> = Vec::new();
    paths(&graph, "start", &mut path, &mut visited, false);
}

fn parse_graph(inp: &str) -> Graph {
    let parsed: Vec<(_, _)> = inp
        .lines()
        .map(|l| (l.split('-').next().unwrap(), l.split('-').nth(1).unwrap()))
        .collect();

    let mut graph: Graph = HashMap::new();

    for el in parsed {
        let dests_lr = graph.entry(el.0.clone()).or_insert(vec![]);
        dests_lr.push(el.1.clone());

        let dests_rl = graph.entry(el.1.clone()).or_insert(vec![]);
        dests_rl.push(el.0.clone());
    }

    graph
}

fn paths(g: &Graph, v: &str, path: &mut Vec<String>, visited: &mut Vec<String>, mut twice: bool) {
    if v == "end" {
        path.push("end".to_string());
        println!("{:?}", path);
        return;
    }

    if v.chars().all(|c| c.is_lowercase()) && visited.contains(&v.to_owned()) {
        if twice || v == "start" || v == "end" {
            return;
        } else {
            twice = true;
        }
    }

    for e in &g[v] {
        let mut pat = path.clone();
        pat.push(v.to_owned());

        let mut vis = visited.clone();
        vis.push(v.to_owned());

        paths(g, *e, &mut pat, &mut vis, twice);
    }
}
