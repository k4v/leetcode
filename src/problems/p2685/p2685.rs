use std::path::Path;

use crate::common::solutions::{Error, Solution};

type Input = (i32, Vec<Vec<i32>>);
type Output = i32;

pub struct P2685 {
    test_cases: Vec<Input>
}

impl Solution for P2685 {
    type Input = Input;
    type Output = Output;

    fn read(input_file: &Path) -> Result<Vec<Input>, Error> {
        Ok(vec![
            (6, vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4]]),
            (6, vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4],vec![3,5]])])
    }

    fn solve(&self) -> Result<Vec<Output>, Error> {
        let mut results = vec![];
        for input in &self.test_cases {
            results.push(self.count_complete_components(input));
        }

        Ok(results)
    }
}

impl P2685 {
    pub fn new(input_file: &Path) -> P2685 {
        return Self {
            test_cases: P2685::read(input_file).unwrap()
        }
    }

    fn count_complete_components(&self, graph_descriptor: &Input) -> Output {
        let (vertices, edges) = graph_descriptor;
        let mut adjacency_list = vec![vec![]; *vertices as usize];
        
        for edge in edges {
            adjacency_list[edge[0] as usize].push(edge[1]);
            adjacency_list[edge[1] as usize].push(edge[0]);
        }

        let mut num_complete_components = 0;

        let mut visited_vertices = vec![0; *vertices as usize];
        for (vertex, edges) in adjacency_list.iter().enumerate() {
            if visited_vertices[vertex] == 1 {
                continue;
            }

            // Track the number of vertices in the component, and the total number of edges
            // If |e| == (|v| * |v - 1|), then this component is completely connected
            let mut component_info = (0, 0);
            Self::dfs(vertex, &adjacency_list, &mut visited_vertices, &mut component_info);

            if component_info.1 == component_info.0 * (component_info.0 - 1) {
                num_complete_components += 1;
            }
        }
        
        num_complete_components
    }

    fn dfs(root: usize, adjacency_list: &Vec<Vec<i32>>, visited_vertices: &mut Vec<i32>, component_info: &mut(u32, u32)) {
        if visited_vertices[root] == 1 {
            return;
        }

        visited_vertices[root] = 1;

        component_info.0 += 1;
        component_info.1 += adjacency_list[root].len() as u32;

        for adj_vertex in &adjacency_list[root] {
            Self::dfs(*adj_vertex as usize, adjacency_list, visited_vertices, component_info);
        }
    }
}