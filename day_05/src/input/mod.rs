use std::fmt::Debug;
use std::str::FromStr;

#[allow(dead_code)]
pub enum Input {
    Real,
    Test1,
}

pub fn get_input<ParseAs>(input: Input) -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    let input = match input {
        Input::Real => include_str!("real.txt"),
        Input::Test1 => include_str!("test_1.txt"),
    };

    input
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}
