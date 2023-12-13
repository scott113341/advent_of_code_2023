use std::fmt::Debug;
use std::str::FromStr;

#[allow(dead_code)]
pub enum Input {
    Real,
    Test1,
    Test2,
    Test3,
    Test4,
}

pub fn get_input<ParseAs>(input: Input) -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    let input = match input {
        Input::Real => include_str!("real.txt"),
        Input::Test1 => include_str!("test_1.txt"),
        Input::Test2 => include_str!("test_2.txt"),
        Input::Test3 => include_str!("test_3.txt"),
        Input::Test4 => include_str!("test_4.txt"),
    };

    input
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}
