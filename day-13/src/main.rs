use std::collections::HashMap;
use std::fs;

fn main() {
    part2();
}

fn part1() {
    let mut _test_input = r"939
    7,13,x,x,59,x,31,19".lines();

        let file_contents = fs::read_to_string("input.txt").expect("Dead file");
        let mut real_input = file_contents.lines();

        let departure_time = real_input.next().unwrap().trim().parse::<u32>().unwrap();
        println!("Departure time: {}", departure_time);

        let buses: Vec<u32> = real_input.next().unwrap().trim().split(',').filter_map(|b| b.parse().ok()).collect();
        println!("Buses: {:?}", buses);

        let mut waiting_times = HashMap::new();

        for bus in buses {
            waiting_times.insert(bus, ((departure_time / bus) + 1) * bus);
        }

        println!("Waiting times: {:?}", waiting_times);

        let bus = waiting_times.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();
        println!("Bus: {:?}", bus);

        let wait = bus.1 - departure_time;
        println!("Wait: {:?}", wait);

        println!("Answer: {:?}", wait * bus.0);
}

fn part2() {

}