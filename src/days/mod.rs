pub fn all_days() -> Vec<(&'static str, fn(&str) -> (String, String))> {
    vec![
        ("day01", day01::solve as fn(&str) -> (String, String)),
        ("day02", day02::solve as fn(&str) -> (String, String)),
        ("day03", day03::solve as fn(&str) -> (String, String)),
        ("day04", day04::solve as fn(&str) -> (String, String)),
        ("day05", day05::solve as fn(&str) -> (String, String)),
        ("day06", day06::solve as fn(&str) -> (String, String)),
        ("day07", day07::solve as fn(&str) -> (String, String)),
    ]
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub mod day08;
