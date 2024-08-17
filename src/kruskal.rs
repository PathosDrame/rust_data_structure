use crate::MGraph;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct Edge {
    begin: usize,
    end: usize,
    weight: i32,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct Set {
    k: usize,
    edge: Vec<Edge>,
}

#[allow(unused)]
impl Set {
    fn new(g: &MGraph) -> Self {
        let mut k = 0;
        let mut edge = Vec::new();
        for i in 0..g.node_num {
            for j in i + 1..g.node_num {
                if g.edges[i][j] > 0 {
                    edge.push(Edge::new(i, j, g.edges[i][j]));
                    k += 1;
                }
            }
        }
        Self { k, edge }
    }

    fn sort(&mut self) {
        for i in 0..self.k {
            for j in i + 1..self.k {
                if self.edge[i].weight > self.edge[j].weight {
                    self.edge.swap(i, j);
                }
            }
        }
    }

    fn get_root(set: &[usize], mut a: usize) -> usize {
        while a != set[a] {
            a = set[a];
        }
        a
    }

    fn kruskal(&self, g: &MGraph) -> (i32, Vec<Edge>) {
        let (mut a, mut b, mut ans) = (0, 0, 0);
        let mut set = (0..g.node_num).collect::<Vec<usize>>();
        let mut vec = Vec::new();
        for i in 0..self.k {
            a = Self::get_root(&set, self.edge[i].begin);
            b = Self::get_root(&set, self.edge[i].end);
            if a != b {
                set[a] = b;
                let num = self.edge[i].weight;
                vec.push(Edge::new(self.edge[i].begin, self.edge[i].end, num));
                ans += num;
            }
        }
        (ans, vec)
    }
}

impl Edge {
    fn new(begin: usize, end: usize, weight: i32) -> Self {
        Self { begin, end, weight }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let show = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
            "G".to_string(),
        ];

        let mut g = MGraph::new(show, false, 0);
        g.add(0, 1, 12);
        g.add(0, 6, 14);
        g.add(0, 5, 16);
        g.add(1, 2, 10);
        g.add(1, 5, 7);
        g.add(2, 3, 3);
        g.add(2, 4, 5);
        g.add(2, 5, 6);
        g.add(3, 4, 4);
        g.add(4, 5, 2);
        g.add(4, 6, 8);
        g.add(5, 6, 9);

        let mut set = Set::new(&g);
        set.sort();
        let (k, vec) = set.kruskal(&g);
        assert_eq!(36, k);
        assert_eq!(
            vec![
                Edge {
                    begin: 4,
                    end: 5,
                    weight: 2
                },
                Edge {
                    begin: 2,
                    end: 3,
                    weight: 3
                },
                Edge {
                    begin: 3,
                    end: 4,
                    weight: 4,
                },
                Edge {
                    begin: 1,
                    end: 5,
                    weight: 7,
                },
                Edge {
                    begin: 4,
                    end: 6,
                    weight: 8
                },
                Edge {
                    begin: 0,
                    end: 1,
                    weight: 12
                }
            ],
            vec
        );
    }
}
