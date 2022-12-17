use cached::proc_macro::cached;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    thread::current,
};

// https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: usize,
    tunnels_to: Vec<&'a str>,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (i, number) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;

    Ok((i, number))
}

fn parse_line(input: &str) -> IResult<&str, Valve> {
    let (i, (_, name, _, flow_rate, _, tunnels)) = tuple((
        tag("Valve "),
        alpha1,
        tag(" has flow rate="),
        parse_usize,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), alpha1),
    ))(input)?;

    Ok((
        i,
        Valve {
            name: name,
            flow_rate: flow_rate,
            tunnels_to: tunnels.iter().map(|&s| s).collect::<Vec<_>>(),
        },
    ))
}

fn max_flow(
    cur: &str,
    flows: &BTreeMap<&str, usize>,
    adj: &BTreeMap<&str, Vec<&str>>,
    opened: &Vec<&str>,
    min_left: usize,
) -> usize {
    if min_left <= 0 {
        return 0;
    }

    println!("Opened: {:?} minutes left: {}", opened, min_left);

    let mut best = 0;

    if !opened.contains(&cur) {
        let val = (min_left - 1) * flows.get(cur).unwrap();

        let mut cur_opened = opened.clone();
        cur_opened.push(cur);

        for a in adj.get(cur).unwrap() {
            if val != 0 {
                best = max(
                    best,
                    val + max_flow(&a, flows, adj, &cur_opened, min_left - 2),
                )
            }

            best = max(best, max_flow(a, flows, adj, opened, min_left - 1))
        }
    }

    best
}

fn score_pathway(
    perm: &Vec<&str>,
    flows: &BTreeMap<&str, usize>,
    dist: &BTreeMap<(&str, &str), usize>,
) -> usize {
    let mut perm = VecDeque::from(perm.clone());
    perm.push_front("AA");

    // let mut current_pos = "AA".to_owned();

    let path_cost = perm
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| dist.get(&(*a, *b)).unwrap())
        .sum::<usize>();

    println!("Perm: {:?}, path cost: {}", perm, path_cost + perm.len());
    return 0;

    let mut total_pressure = 0;
    let mut open_flows: Vec<usize> = Vec::new();
    let mut minutes = 0;

    // while let Some(next_pos) = perm.pop_front() {
    //     // println!("Next pos: {}", next_pos);
    //     // println!("Open flows: {:?}", open_flows);

    //     let path = pathfinding::prelude::dijkstra(
    //         &current_pos,
    //         |p| graph.get(p).unwrap().iter().map(|p| (p.clone(), 1)),
    //         |p| *p == next_pos,
    //     )
    //     .unwrap()
    //     .0;
    //     // println!("Path: {:?}", path);

    //     let minutes_needed = path.len(); // len() - 1 minutes to get there, 1 minute to open the valve
    //     for flow in &open_flows {
    //         total_pressure += flow * minutes_needed;
    //     }

    //     minutes += minutes_needed;

    //     if minutes > 30 {
    //         break;
    //     }

    // open_flows.push(valves.get(&next_pos).unwrap().flow_rate);

    // current_pos = next_pos;
    // }

    // if minutes < 30 {
    //     total_pressure += (30 - minutes) * open_flows.iter().sum::<usize>();
    // }

    // println!("Final minutes {}", minutes);
    // println!("Total pressure {}", total_pressure);

    // total_pressure
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    let mut nodes: BTreeSet<&str> = BTreeSet::new();
    let mut flows: BTreeMap<&str, usize> = BTreeMap::new();
    let mut dist: BTreeMap<(&str, &str), usize> = BTreeMap::new();
    let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

    for l in input.lines() {
        let valve = parse_line(l).unwrap().1;
        println!("{:?}", valve);

        nodes.insert(valve.name);
        flows.insert(valve.name, valve.flow_rate);

        adj.insert(valve.name, valve.tunnels_to.clone());

        for tunnel in valve.tunnels_to {
            dist.insert((valve.name, tunnel), 1);
        }
    }

    for dist_filler in nodes
        .iter()
        .cloned()
        .cartesian_product(nodes.iter().cloned())
    {
        if !dist.contains_key(&dist_filler) {
            dist.insert(dist_filler, 1000);
        }
    }

    // Floyd Warshall
    for prod in (0..3)
        .map(|i| nodes.iter().cloned())
        .multi_cartesian_product()
    {
        let ij = dist.get(&(prod[1], prod[2])).unwrap();
        let ik = dist.get(&(prod[1], prod[0])).unwrap();
        let kj = dist.get(&(prod[0], prod[2])).unwrap();

        dist.insert((prod[1], prod[2]), min(*ij, ik + kj));
    }

    // let v: Vec<_> = input.lines().collect();
    println!("{:?}", dist);

    println!("Max flow: {}", max_flow("AA", &flows, &adj, &vec![], 30));

    // let non_zero_rooms = flows
    //     .iter()
    //     .filter(|(n, f)| **f != 0)
    //     .map(|(n, _)| *n)
    //     .collect::<Vec<_>>();
    // println!("Non zero rooms: {:?}", non_zero_rooms);

    // let perms = non_zero_rooms
    //     .clone()
    //     .into_iter()
    //     .permutations(non_zero_rooms.len());
    // // println!("{:?}", perms.collect::<Vec<_>>());

    // for perm in perms {
    //     score_pathway(&perm, &flows, &dist);
    // }

    // let most_pressure = perms
    //     .par_bridge()
    //     .map(|p| score_pathway(p, &valves, &graph))
    //     .max();

    // score_pathway(
    //     vec![
    //         "DD".to_owned(),
    //         "BB".to_owned(),
    //         "JJ".to_owned(),
    //         "HH".to_owned(),
    //         "EE".to_owned(),
    //         "CC".to_owned(),
    //     ],
    //     &valves,
    //     &graph,
    // );

    // Part 1
    println!("Part 1");

    // println!("Answer part 1: {:?}", most_pressure);

    // Part 2
    println!("Part 2");

    println!("Answer part 2: {:?}", "");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
