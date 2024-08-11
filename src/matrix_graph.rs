#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct MatrixVertex {
    no: i32,
    show: String,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct MGraph {
    vec: Vec<MatrixVertex>,
    node_num: usize,
    edges: Vec<Vec<i32>>,
    edge_num: usize,
    directed: bool,
}

#[allow(unused)]
impl MGraph {
    fn new(show: Vec<String>, directed: bool) -> Self {
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

    fn add(&mut self, x: usize, y: usize, w: i32) {
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
}

impl MatrixVertex {
    fn new(no: i32, show: String) -> Self {
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
}
