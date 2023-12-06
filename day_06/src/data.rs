#[derive(Eq, PartialEq, Debug)]
pub struct Race {
    pub time_ms: usize,
    pub distance_record_mm: usize,
}

impl Race {
    pub fn distance_for_charge(&self, charge_time_ms: usize) -> usize {
        if charge_time_ms >= self.time_ms {
            0
        } else {
            let speed = charge_time_ms;
            let remaining_time_ms = self.time_ms - charge_time_ms;
            speed * remaining_time_ms
        }
    }

    pub fn ways_to_beat(&self) -> usize {
        let mut ways_to_beat = 0;

        for charge_time in 0..=self.time_ms {
            let distance = self.distance_for_charge(charge_time);
            if distance > self.distance_record_mm {
                ways_to_beat += 1;
            }
        }

        ways_to_beat
    }
}

pub fn build_races(lines: &[String]) -> Vec<Race> {
    let parse_ints = |s: &String| -> Vec<usize> {
        s.split_whitespace()
            .filter_map(|t| t.parse().ok())
            .collect()
    };

    let times = parse_ints(&lines[0]);
    let distance_records = parse_ints(&lines[1]);

    times
        .iter()
        .zip(distance_records)
        .map(|(time_ms, distance_record_mm)| Race {
            time_ms: time_ms.to_owned(),
            distance_record_mm: distance_record_mm.to_owned(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_distance_for_charge() {
        let races = build_races(&get_input::<String>(Input::Test1));
        assert_eq!(races[0].distance_for_charge(0), 0);
        assert_eq!(races[0].distance_for_charge(1), 6);
        assert_eq!(races[0].distance_for_charge(2), 10);
        assert_eq!(races[0].distance_for_charge(3), 12);
        assert_eq!(races[0].distance_for_charge(4), 12);
        assert_eq!(races[0].distance_for_charge(5), 10);
        assert_eq!(races[0].distance_for_charge(6), 6);
        assert_eq!(races[0].distance_for_charge(7), 0);
    }

    #[test]
    fn test_build_races() {
        assert_eq!(
            build_races(&get_input::<String>(Input::Test1)),
            vec![
                Race {
                    time_ms: 7,
                    distance_record_mm: 9,
                },
                Race {
                    time_ms: 15,
                    distance_record_mm: 40,
                },
                Race {
                    time_ms: 30,
                    distance_record_mm: 200,
                }
            ]
        );
    }
}
