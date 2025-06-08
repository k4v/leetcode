use std::{collections::HashSet, path::Path};

use crate::common::solutions::{Error, Solution};

type Input = (Vec<u8>, u8);
type Output = Option<(u8, u8)>;

pub struct P1 {
    test_cases: Vec<Input>
}

impl Solution for P1 {
    type Input = Input;
    type Output = Output;

    fn read(input_file: &Path) -> Result<Vec<Input>, Error> {
        Ok(Vec::<Input>::new())
    }

    fn solve(&self) -> Result<Vec<Output>, Error> {
        let mut results = vec![];
        for input in &self.test_cases {
            results.push(self.two_sum(input));
        }

        Ok(results)
    }
}

impl P1 {
    pub fn new(input_file: &Path) -> P1 {
        return Self {
            test_cases: P1::read(input_file).unwrap()
        }
    }

    fn two_sum(&self, input: &Input) -> Output {
        if input.0.is_empty() {
            return None;
        }

        // Set of all numbers found so far
        let mut found_nums = HashSet::<u8>::new();
        // Pair of numbers that match the target, if found
        let mut result = None;

        // Loop over input until we find a match
        for num in input.0.iter() {
            let num = *num;
            found_nums.insert(num);
            if found_nums.contains(&(input.1 - num)) {
                result = Some((input.1 - num, num));

                // Unique result expected
                break;
            }
        };

        result
    }
}