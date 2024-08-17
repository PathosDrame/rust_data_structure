use crate::MGraph;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct Edge {
    begin: usize,
    end: usize,
    weight: i32,
}

#[allow(unused)]
impl Edge {
    fn new(begin: usize, end: usize, weight: i32) -> Self {
        Self { begin, end, weight }
    }

    fn prim(g: MGraph, start: usize) -> (i32, Vec<Edge>) {
        let (mut cost, mut mark, mut visit, mut ans) = (
            vec![0; g.node_num],
            vec![0; g.node_num],
            vec![0i32; g.node_num],
            Vec::new(),
        );
        let (mut sum, mut k) = (0, 0);
        for i in 0..g.node_num {
            cost[i] = g.edges[start][i];
            mark[i] = 0;
            if cost[i] < i32::MAX {
                visit[i] = start as i32;
            } else {
                visit[i] = -1;
            }
        }
        mark[start] = 1;
        for _ in 0..g.node_num - 1 {
            let mut min = i32::MAX;
            for j in 0..g.node_num {
                if mark[j] == 0 && cost[j] < min {
                    min = cost[j];
                    k = j;
                }
            }
            mark[k] = 1;
            ans.push(Edge::new(visit[k] as usize, k, min));
            sum += min;
            for j in 0..g.node_num {
                if mark[j] == 0 && g.edges[k][j] < cost[j] {
                    cost[j] = g.edges[k][j];
                    visit[j] = k as i32;
                }
            }
        }
        (sum, ans)
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

        let mut g = MGraph::new(show, false, i32::MAX);
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

        let (sum, vec) = Edge::prim(g, 0);
        assert_eq!(36, sum);
        assert_eq!(
            vec![
                Edge {
                    begin: 0,
                    end: 1,
                    weight: 12
                },
                Edge {
                    begin: 1,
                    end: 5,
                    weight: 7
                },
                Edge {
                    begin: 5,
                    end: 4,
                    weight: 2,
                },
                Edge {
                    begin: 4,
                    end: 3,
                    weight: 4,
                },
                Edge {
                    begin: 3,
                    end: 2,
                    weight: 3
                },
                Edge {
                    begin: 4,
                    end: 6,
                    weight: 8
                }
            ],
            vec
        );
    }
}
