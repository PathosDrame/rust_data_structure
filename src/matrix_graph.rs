use std::collections::VecDeque;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub struct MatrixVertex {
    pub no: i32,
    pub show: String,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub struct MGraph {
    pub vec: Vec<MatrixVertex>,
    pub node_num: usize,
    pub edges: Vec<Vec<i32>>,
    pub edge_num: usize,
    pub directed: bool,
}

#[allow(unused)]
impl MGraph {
    pub fn new(show: Vec<String>, directed: bool) -> Self {
        let mut vec = Vec::new();
        let node_num = show.len();
        for (no, show) in show.into_iter().enumerate() {
            vec.push(MatrixVertex::new(no as i32, show));
        }
        let edges = vec![vec![0; node_num]; node_num];
        Self {
            vec,
            node_num,
            edges,
            edge_num: 0,
            directed,
        }
    }

    pub fn add(&mut self, x: usize, y: usize, w: i32) {
        if x > self.node_num || y > self.node_num {
            return;
        }
        if w > 0 && w < i32::MAX {
            self.edges[x][y] = w;
            if !self.directed {
                self.edges[y][x] = w;
            }
            self.edge_num += 1;
        }
    }

    pub fn dfs(&self, v: usize) -> Vec<String> {
        if v > self.node_num {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut visited = vec![0; self.node_num];
        self.dfs_travel(v, &mut ans, &mut visited);
        ans
    }

    pub fn dfs_travel(&self, v: usize, ans: &mut Vec<String>, visited: &mut Vec<i32>) {
        ans.push(self.vec[v].show.clone());
        visited[v] = 1;
        for i in 0..self.node_num {
            let w = self.edges[v][i];
            if w > 0 && w < i32::MAX && visited[i] == 0 {
                Self::dfs_travel(self, i, ans, visited);
            }
        }
    }

    pub fn bfs(&self, v: usize) -> Vec<String> {
        if v > self.node_num {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut visited = vec![0; self.node_num];
        let mut q = VecDeque::new();
        q.push_back(v);
        while let Some(v) = q.pop_front() {
            ans.push(self.vec[v].show.clone());
            visited[v] = 1;
            for (i, val) in visited.iter_mut().enumerate().take(self.node_num) {
                let w = self.edges[v][i];
                if w > 0 && w < i32::MAX && *val == 0 {
                    q.push_back(i);
                    *val = 1;
                }
            }
        }
        ans
    }
}

impl MatrixVertex {
    pub fn new(no: i32, show: String) -> Self {
        Self { no, show }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let show = vec![
            "V1".to_string(),
            "V2".to_string(),
            "V3".to_string(),
            "V4".to_string(),
            "V5".to_string(),
            "V6".to_string(),
            "V7".to_string(),
            "V8".to_string(),
        ];
        let mut g = MGraph::new(show, false);
        g.add(0, 1, 1);
        g.add(0, 2, 1);
        g.add(1, 3, 1);
        g.add(1, 4, 1);
        g.add(2, 5, 1);
        g.add(2, 6, 1);
        g.add(5, 6, 1);
        g.add(3, 7, 1);
        g.add(4, 7, 1);
        assert_eq!(9, g.edge_num);
        assert_eq!(
            MGraph {
                vec: vec![
                    MatrixVertex {
                        no: 0,
                        show: "V1".to_string()
                    },
                    MatrixVertex {
                        no: 1,
                        show: "V2".to_string()
                    },
                    MatrixVertex {
                        no: 2,
                        show: "V3".to_string()
                    },
                    MatrixVertex {
                        no: 3,
                        show: "V4".to_string()
                    },
                    MatrixVertex {
                        no: 4,
                        show: "V5".to_string()
                    },
                    MatrixVertex {
                        no: 5,
                        show: "V6".to_string()
                    },
                    MatrixVertex {
                        no: 6,
                        show: "V7".to_string()
                    },
                    MatrixVertex {
                        no: 7,
                        show: "V8".to_string()
                    }
                ],
                node_num: 8,
                edges: vec![
                    vec![0, 1, 1, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 1, 1, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 1, 1, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, 1],
                    vec![0, 1, 0, 0, 0, 0, 0, 1],
                    vec![0, 0, 1, 0, 0, 0, 1, 0],
                    vec![0, 0, 1, 0, 0, 1, 0, 0],
                    vec![0, 0, 0, 1, 1, 0, 0, 0]
                ],
                edge_num: 9,
                directed: false
            },
            g
        );
    }

    #[test]
    fn dfs_should_work() {
        let show = vec![
            "V1".to_string(),
            "V2".to_string(),
            "V3".to_string(),
            "V4".to_string(),
            "V5".to_string(),
            "V6".to_string(),
            "V7".to_string(),
            "V8".to_string(),
        ];
        let mut g = MGraph::new(show, false);
        g.add(0, 1, 1);
        g.add(0, 2, 1);
        g.add(1, 3, 1);
        g.add(1, 4, 1);
        g.add(2, 5, 1);
        g.add(2, 6, 1);
        g.add(5, 6, 1);
        g.add(3, 7, 1);
        g.add(4, 7, 1);

        let vec = g.dfs(0);
        assert_eq!(
            vec![
                "V1".to_string(),
                "V2".to_string(),
                "V4".to_string(),
                "V8".to_string(),
                "V5".to_string(),
                "V3".to_string(),
                "V6".to_string(),
                "V7".to_string()
            ],
            vec
        );
    }

    #[test]
    fn bfs_should_work() {
        let show = vec![
            "V1".to_string(),
            "V2".to_string(),
            "V3".to_string(),
            "V4".to_string(),
            "V5".to_string(),
            "V6".to_string(),
            "V7".to_string(),
            "V8".to_string(),
        ];
        let mut g = MGraph::new(show, false);
        g.add(0, 1, 1);
        g.add(0, 2, 1);
        g.add(1, 3, 1);
        g.add(1, 4, 1);
        g.add(2, 5, 1);
        g.add(2, 6, 1);
        g.add(5, 6, 1);
        g.add(3, 7, 1);
        g.add(4, 7, 1);

        let vec = g.bfs(0);
        assert_eq!(
            vec![
                "V1".to_string(),
                "V2".to_string(),
                "V3".to_string(),
                "V4".to_string(),
                "V5".to_string(),
                "V6".to_string(),
                "V7".to_string(),
                "V8".to_string()
            ],
            vec
        );
    }
}
