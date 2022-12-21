use gcollections::ops::*;
use gcollections::VectorStack;
use interval::interval_set::*;
use interval::ops::Range;
use pcp::concept::IntVariable;
use pcp::concept::Var;
use pcp::kernel::*;
use pcp::model::*;
use pcp::propagators::cumulative::Cumulative;
use pcp::propagators::*;
use pcp::search::branching::*;
use pcp::search::debugger::*;
use pcp::search::engine::one_solution::*;
use pcp::search::propagation::*;
use pcp::search::*;
use pcp::term::ops::*;
use pcp::term::*;
use pcp::variable::Iterable;
use std::collections::BTreeMap;
use std::fmt::{Display, Error, Formatter};
use std::fs;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    let mut assignments = vec![];
    let mut operations = vec![];

    for l in input.lines() {
        let s: Vec<&str> = l.split(" ").collect();

        if s.len() == 2 {
            let identifier = &s[0][..s[0].len() - 1];
            let value: isize = str::parse(s[1]).unwrap();

            println!("Got {} with {}", identifier, value);
            assignments.push((identifier, value));
        } else if s.len() == 4 {
            let identifier = &s[0][..s[0].len() - 1];
            let left = s[1];
            let operator = s[2];
            let right = s[3];

            println!("Got {identifier}/{left}/{operator}/{right}");
            operations.push((identifier, left, operator, right));
        }
    }

    let mut space = FDSpace::empty();

    let mut assignment_vars = BTreeMap::new();
    for (ass_id, ass_value) in assignments {
        let a =
            Box::new(space.vstore.alloc(IntervalSet::new(0, isize::MAX / 2 - 1))) as Var<VStore>;

        space.cstore.alloc(Box::new(XEqY::new(
            a.bclone(),
            Box::new(Constant::new(ass_value)),
        )));

        assignment_vars.insert(ass_id.clone(), a);
    }

    for (op_target, op_left, op_op, op_right) in operations {
        if !assignment_vars.contains_key(op_target) {
            let v = Box::new(space.vstore.alloc(IntervalSet::new(0, isize::MAX / 2 - 1)))
                as Var<VStore>;
            assignment_vars.insert(op_target, v);
        }
        let op_target_var = assignment_vars.get(op_target).unwrap().bclone();

        if !assignment_vars.contains_key(op_left) {
            let v = Box::new(space.vstore.alloc(IntervalSet::new(0, isize::MAX / 2 - 1)))
                as Var<VStore>;
            assignment_vars.insert(op_left, v);
        }
        let op_left_var = assignment_vars.get(op_left).unwrap().bclone();

        if !assignment_vars.contains_key(op_right) {
            let v = Box::new(space.vstore.alloc(IntervalSet::new(0, isize::MAX / 2 - 1)))
                as Var<VStore>;
            assignment_vars.insert(op_right, v);
        }
        let op_right_var = assignment_vars.get(op_right).unwrap().bclone();

        match op_op {
            "+" => {
                space.cstore.alloc(Box::new(XEqYPlusZ::new(
                    op_target_var.bclone(),
                    op_left_var.bclone(),
                    op_right_var.bclone(),
                )));
            }
            "-" => {
                // target = left - right;
                space.cstore.alloc(Box::new(XEqYPlusZ::new(
                    op_left_var.bclone(),
                    op_target_var.bclone(),
                    op_right_var.bclone(),
                )));
            }
            "/" => {
                // target = left / right
                space.cstore.alloc(Box::new(XEqYMulZ::new(
                    op_left_var.bclone(),
                    op_target_var.bclone(),
                    op_right_var.bclone(),
                )));
            }
            "*" => {
                space.cstore.alloc(Box::new(XEqYMulZ::new(
                    op_target_var.bclone(),
                    op_left_var.bclone(),
                    op_right_var.bclone(),
                )));
            }
            _ => todo!(),
        }
    }

    // let a = Box::new(space.vstore.alloc(IntervalSet::new(1, 1000))) as Var<VStore>;
    // let b = Box::new(space.vstore.alloc(IntervalSet::new(1, 1000))) as Var<VStore>;

    // space.cstore.alloc(Box::new(XEqY::new(
    //     a.bclone(),
    //     Box::new(Addition::new(b.bclone(), 10)),
    // )));

    let mut search = one_solution_engine();
    search.start(&space);

    let (frozen_space, status) = search.enter(space);
    let space = frozen_space.unfreeze();

    println!("Status: {:?}", status);
    for dom in space.vstore.iter() {
        println!("{}", dom.lower());
    }

    let v: Vec<_> = input.lines().collect();
    println!("{:?}", v);

    // Part 1
    println!("Part 1");

    println!("Answer part 1: {:?}", "");

    // Part 2
    println!("Part 2");

    println!("Answer part 2: {:?}", "");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
