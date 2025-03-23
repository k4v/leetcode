use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Arguments to run solved Leetcode problems
pub(crate) struct LcArgs {
    /// problem ID in Leetcode
    #[argh(option, short='p')]
    pub problem: usize,

    /// input file to read use cases
    #[argh(option, short='f')]
    pub input_file: String
}
