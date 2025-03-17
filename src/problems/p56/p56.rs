use std::path::Path;

type Input = Vec<(u8, u8)>;
type Output = Vec<(u8, u8)>;

pub struct P56 {
    test_cases: Vec<Input>
}

impl P56 {
    pub fn new(input_file: &Path) -> P56 {
        return Self {
            test_cases: vec![]
        }
    }

    pub fn run(&self) {
        for test_case in &self.test_cases {
            self.merge(test_case);
        }
    }

    fn merge(&self, intervals: &Input) -> Output {
        if intervals.is_empty() {
            return vec![];
        }

        // Sort given intervals by start interval
        let mut sorted_intervals = intervals.to_vec();
        sorted_intervals.sort_by(|i1, i2| {
            i1.0.cmp(&i2.0)
        });

        // Position in sorted_intervals to insert (overwrite) merged interval
        let mut insert_idx = 0;

        for i in 1..sorted_intervals.len() {
            let earlier = &sorted_intervals[insert_idx];
            let current = &sorted_intervals[i];

            if earlier.1 >= current.0 || earlier.1 >= current.1 {
                sorted_intervals[insert_idx] = (earlier.0, std::cmp::max(earlier.1, current.1));
            } else {
                insert_idx += 1;
                sorted_intervals[insert_idx] = *current;
            }
        }

        let result = sorted_intervals[..insert_idx+1].to_vec();

        return result;
    }
}