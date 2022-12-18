use cached::proc_macro::cached;
use cached::SizedCache;
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

#[cached(
    type = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}-{}-{}", cur, opened.join(""), min_left) }"#
)]
fn max_flow(
    cur: &str,
    flows: &BTreeMap<&str, usize>,
    adj: &BTreeMap<&str, Vec<&str>>,
    opened: &Vec<&str>,
    min_left: usize,
) -> usize {
    // From: https://github.com/nthistle/advent-of-code/blob/master/2022/day16/day16.py#L23
    // Kinda works but doesn't
    if min_left <= 0 {
        return 0;
    }

    // if (min_left % 5 == 0) {
    // println!("Opened: {:?} minutes left: {}", opened, min_left);
    // }

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

fn solve(flows: &BTreeMap<&str, usize>, adj: &BTreeMap<&str, Vec<&str>>) -> isize {
    let mut states = VecDeque::from(vec![(1, "AA", 0isize, BTreeSet::from([]))]);
    let mut seen: BTreeMap<(usize, &str), isize> = BTreeMap::new();
    let mut best = 0;
    // Taken from: https://github.com/carrdelling/AdventOfCode2022/blob/main/day16/silver.py#L3
    // Still doesn't give the right answer.
    // But fuck this.

    while let Some((time, location, score, opened_s)) = states.pop_front() {
        println!("{} {} {} {:?}", time, location, score, opened_s);

        let mut opened = opened_s.clone();

        if seen.get(&(time, location)).map_or(-1, |&s| s) >= score {
            continue;
        }
        seen.insert((time, location), score);

        if time == 30 {
            best = max(best, score);
            continue;
        }

        if *flows.get(location).unwrap() > 0 && !opened.contains(&location) {
            opened.insert(location);
            let new_score: isize = score
                + opened
                    .iter()
                    .map(|&l| flows.get(&l).map_or(0, |v| *v) as isize)
                    .sum::<isize>();
            let new_state = (time + 1, location, new_score, opened.clone());
            println!("New state: {:?}", new_state);
            states.push_front(new_state);

            opened.remove(location);
        }

        let new_score: isize = score
            + opened
                .iter()
                .map(|&l| flows.get(&l).map_or(0, |v| *v) as isize)
                .sum::<isize>();

        for option in adj.get(location).unwrap() {
            let new_state = (time + 1, *option, new_score, opened.clone());
            println!("New state: {:?}", new_state);
            states.push_front(new_state);
        }
    }
    best
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("full_input.txt").expect("File not readable");

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

    // println!("Max flow: {}", max_flow("AA", &flows, &adj, &vec![], 30));

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

    println!("Answer part 1: {:?}", solve(&flows, &adj));

    // Part 2
    println!("Part 2");

    println!("Answer part 2: {:?}", "");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
