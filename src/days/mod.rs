pub fn all_days() -> Vec<(&'static str, fn(&str) -> (String, String))> {
    vec![("day01", day01::solve as fn(&str) -> (String, String))]
}

pub mod day01;
pub mod day02;
