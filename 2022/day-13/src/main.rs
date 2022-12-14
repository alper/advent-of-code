use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};
use std::{cmp::min, fs};
use std::{cmp::Ordering, fmt::Debug};

#[derive(PartialEq, Eq)]
struct Node<T> {
    data: Option<T>,
    children: Vec<Node<T>>,
}

impl<T> Debug for Node<T>
where
    T: Debug + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data {
            Some(d) => write!(f, "Node({:?})", d),
            None => write!(
                f,
                "Node [ {} ]",
                self.children
                    .iter()
                    .map(|c: &Node<T>| format!("{:?}\n", c))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

fn right_order<T>(l: &Node<T>, r: &Node<T>) -> bool
where
    T: PartialOrd + Copy + Debug,
{
    if let Some(l_val) = l.data {
        if let Some(r_val) = r.data {
            // println!("Comparing: {:?}, {:?}", l_val, r_val);
            // both integers
            if l_val < r_val {
                return true;
            }

            if l_val > r_val {
                return false;
            }
        } else {
            // left integer, right list
            // println!("Comparing lR ({:?}) {:?}", l_val, r.children);
            return right_order(
                &Node {
                    data: None,
                    children: vec![Node {
                        data: Some(l_val),
                        children: vec![],
                    }],
                },
                r,
            );
        }
    } else {
        if let Some(r_val) = r.data {
            // left is list, right is int
            // println!("Comparing Lr {:?} ({:?})", l.children, r_val);
            return right_order(
                l,
                &Node {
                    data: None,
                    children: vec![Node {
                        data: Some(r_val),
                        children: vec![],
                    }],
                },
            );
        } else {
            // both lists
            // println!("Comparing 2l: \nl: {:?}\nr: {:?}\n", l.children, r.children);

            for i in 0..min(l.children.len(), r.children.len()) {
                if l.children[i] != r.children[i] {
                    return right_order(&l.children[i], &r.children[i]);
                }
            }

            if l.children.len() < r.children.len() {
                return true;
            }

            if l.children.len() > r.children.len() {
                return false;
            }
        }
    }

    true
}

fn parse_node(input: &str) -> IResult<&str, Node<u32>> {
    alt((
        map(
            delimited(tag("["), separated_list0(tag(","), parse_node), tag("]")),
            |v| Node {
                data: None,
                children: v,
            },
        ),
        map_res(digit1, |s| {
            let res = str::parse::<u32>(s);

            res.map(|d| Node {
                data: Some(d),
                children: vec![],
            })
        }),
    ))(input)
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    let (i, (_, l, _)) = tuple((
        tag("["),
        separated_list0(tag(","), map_res(digit1, str::parse)),
        tag("]"),
    ))(input)?;

    Ok((i, l))
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    let mut packets = vec![];

    let mut counter = 1;
    let mut sum = 0;
    for chunk in &input.lines().chunks(3) {
        // println!("Doing counter: {counter}");

        let (l, r, _) = chunk.collect_tuple().unwrap();
        // println!("Line 1: {}", l);
        // println!("Line 2: {}", r);

        let l_node = parse_node(l).unwrap().1;
        let r_node = parse_node(r).unwrap().1;

        if right_order(&l_node, &r_node) {
            sum += counter;
        }

        packets.push(l_node);
        packets.push(r_node);

        counter += 1;
    }

    println!("Answer part 1: {:?}", sum);

    // Part 2
    println!("Part 2");

    packets.push(parse_node("[[2]]").unwrap().1);
    packets.push(parse_node("[[6]]").unwrap().1);

    packets.sort_by(|l, r| {
        if right_order(l, r) {
            return Ordering::Less;
        } else if !right_order(l, r) {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    });

    println!(
        "Answer part 2: {:?}",
        (packets
            .iter()
            .position(|el| *el == parse_node("[[2]]").unwrap().1)
            .unwrap()
            + 1)
            * (packets
                .iter()
                .position(|el| *el == parse_node("[[6]]").unwrap().1)
                .unwrap()
                + 1)
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{parse_node, right_order, Node};

    #[test_case("1", Node { data: Some(1), children: vec![]})]
    #[test_case("[1,2,3]", Node { data: None, children: vec![
        Node { data: Some(1), children: vec![] },
        Node { data: Some(2), children: vec![] },
        Node { data: Some(3), children: vec![] }]
    })]
    #[test_case("[1,[4,5],3]", Node { data: None, children: vec![
        Node { data: Some(1), children: vec![] },
        Node { data: None, children: vec![
            Node { data: Some(4), children: vec![] },
            Node { data: Some(5), children: vec![] }
        ] },
        Node { data: Some(3), children: vec![] }]
    })]
    #[test_case("[]", Node {data: None, children: vec![]})]
    #[test_case("[[]]", Node {data: None, children: vec![
        Node{ data: None, children: vec![] }
    ]})]
    fn test_parse_node(input: &str, nodes: Node<u32>) {
        assert_eq!(parse_node(input).unwrap().1, nodes);
    }

    #[test_case("[1,1,3,1,1]", "[1,1,5,1,1]", true)]
    #[test_case("[[1],[2,3,4]]", "[[1],4]", true)]
    #[test_case("[9]", "[[8,7,6]]", false)]
    #[test_case("[[4,4],4,4]", "[[4,4],4,4,4]", true)]
    #[test_case("[7,7,7,7]", "[7,7,7]", false)]
    #[test_case("[]", "[3]", true)]
    #[test_case("[[[]]]", "[[]]", false)]
    #[test_case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", false)]
    fn test_right_order(l: &str, r: &str, right: bool) {
        println!("Testing: {} // {}", l, r);
        assert_eq!(
            right_order(&parse_node(l).unwrap().1, &parse_node(r).unwrap().1),
            right
        );
    }
}
