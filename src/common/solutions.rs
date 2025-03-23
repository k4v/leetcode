use std::path::Path;

#[derive(Debug)]
pub struct Error {
    message: String
}

pub trait Solution {
    // Data type of each input test case
    type Input;
    type Output: std::fmt::Debug;

    // Read file containing list of input test cases, and returned parsed test cases
    fn read(input_file: &Path) -> Result<Vec<Self::Input>, Error>;
    // Solve all the provided test cases, and return list of results
    fn solve(&self) -> Result<Vec<Self::Output>, Error>;
}