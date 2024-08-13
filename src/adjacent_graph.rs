use std::collections::VecDeque;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct ArcEdge {
    no: i32,
    weight: i32,
    next: Option<Box<ArcEdge>>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Node {
    no: i32,
    show: String,
    first: Option<Box<ArcEdge>>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct AGraph {
    node_num: i32,
    edge_num: i32,
    directed: bool,
    visited: Vec<i32>,
    nodes: Vec<Option<Box<Node>>>,
}

#[allow(unused)]
impl AGraph {
    fn new(shows: Vec<String>, directed: bool) -> Self {
        let node_num = shows.len();
        let mut nodes = Vec::new();
        for (no, show) in shows.into_iter().enumerate() {
            nodes.push(Some(Box::new(Node::new(no as i32, show))));
        }
        Self {
            node_num: node_num as i32,
            edge_num: 0,
            directed,
            visited: vec![0; node_num],
            nodes,
        }
    }

    fn add(&mut self, x: i32, y: i32, w: i32) {
        if x < 0 || x > self.node_num || y < 0 || y > self.node_num || x == y {
            return;
        }
        let mut node = ArcEdge::new(y, w);
        if let Some(mut n) = self.nodes[x as usize].take() {
            if let Some(first) = n.first {
                node.next = Some(first);
            }
            n.first = Some(Box::new(node));
            self.nodes[x as usize] = Some(n);
            self.edge_num += 1;
        }
        if self.directed {
            let mut node = ArcEdge::new(x, w);
            if let Some(mut n) = self.nodes[y as usize].take() {
                if let Some(first) = n.first {
                    node.next = Some(first);
                }
                n.first = Some(Box::new(node));
                self.nodes[y as usize] = Some(n);
                self.edge_num += 1;
            }
        }
    }

    fn dfs(&mut self, v: i32) -> Vec<String> {
        self.clear();
        let mut ans = Vec::new();
        if v < 0 || v > self.node_num {
            return ans;
        }
        self.dfs_travel(v as usize, &mut ans);
        ans
    }

    fn clear(&mut self) {
        for i in 0..self.node_num {
            self.visited[i as usize] = 0;
        }
    }

    fn dfs_travel(&mut self, v: usize, ans: &mut Vec<String>) {
        let mut p = None;
        self.visited[v] = 1;
        if let Some(n) = &self.nodes[v] {
            ans.push(n.show.clone());
            p.clone_from(&n.first);
            while let Some(t) = p {
                if self.visited[t.no as usize] == 0 {
                    self.dfs_travel(t.no as usize, ans);
                }
                p = t.next;
            }
        }
    }

    fn bfs(&mut self, v: i32) -> Vec<String> {
        let mut ans = Vec::new();
        if v < 0 || v > self.node_num {
            return ans;
        }
        self.clear();
        let mut q = VecDeque::new();
        let mut p = None;
        q.push_back(v);
        self.visited[v as usize] = 1;
        while let Some(v) = q.pop_front() {
            if let Some(n) = &self.nodes[v as usize] {
                ans.push(n.show.clone());
                p.clone_from(&n.first);
                while let Some(t) = p {
                    if self.visited[t.no as usize] == 0 {
                        q.push_back(t.no);
                        self.visited[t.no as usize] = 1;
                    }
                    p = t.next;
                }
            }
        }
        ans
    }
}

impl ArcEdge {
    fn new(no: i32, weight: i32) -> Self {
        Self {
            no,
            weight,
            next: None,
        }
    }
}

impl Node {
    fn new(no: i32, show: String) -> Self {
        Self {
            no,
            show,
            first: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let shows = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ];
        let mut g = AGraph::new(shows, false);
        g.add(0, 4, 1);
        g.add(0, 3, 1);
        g.add(0, 1, 1);
        g.add(1, 4, 1);
        g.add(2, 0, 1);
        g.add(3, 4, 1);
        g.add(3, 2, 1);
        assert_eq!(
            AGraph {
                node_num: 5,
                edge_num: 7,
                directed: false,
                visited: vec![0; 5],
                nodes: vec![
                    Some(Box::new(Node {
                        no: 0,
                        show: "A".to_string(),
                        first: Some(Box::new(ArcEdge {
                            no: 1,
                            weight: 1,
                            next: Some(Box::new(ArcEdge {
                                no: 3,
                                weight: 1,
                                next: Some(Box::new(ArcEdge {
                                    no: 4,
                                    weight: 1,
                                    next: None
                                }))
                            }))
                        }))
                    })),
                    Some(Box::new(Node {
                        no: 1,
                        show: "B".to_string(),
                        first: Some(Box::new(ArcEdge {
                            no: 4,
                            weight: 1,
                            next: None
                        }))
                    })),
                    Some(Box::new(Node {
                        no: 2,
                        show: "C".to_string(),
                        first: Some(Box::new(ArcEdge {
                            no: 0,
                            weight: 1,
                            next: None
                        }))
                    })),
                    Some(Box::new(Node {
                        no: 3,
                        show: "D".to_string(),
                        first: Some(Box::new(ArcEdge {
                            no: 2,
                            weight: 1,
                            next: Some(Box::new(ArcEdge {
                                no: 4,
                                weight: 1,
                                next: None
                            }))
                        }))
                    })),
                    Some(Box::new(Node {
                        no: 4,
                        show: "E".to_string(),
                        first: None
                    }))
                ]
            },
            g
        );
    }

    #[test]
    fn dfs_should_work() {
        let shows = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ];
        let mut g = AGraph::new(shows, false);
        g.add(0, 4, 1);
        g.add(0, 3, 1);
        g.add(0, 1, 1);
        g.add(1, 4, 1);
        g.add(2, 0, 1);
        g.add(3, 4, 1);
        g.add(3, 2, 1);

        let vec = vec![
            "A".to_string(),
            "B".to_string(),
            "E".to_string(),
            "D".to_string(),
            "C".to_string(),
        ];
        let ans = g.dfs(0);
        assert_eq!(vec, ans);
    }

    #[test]
    fn bfs_should_work() {
        let shows = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ];
        let mut g = AGraph::new(shows, false);
        g.add(0, 4, 1);
        g.add(0, 3, 1);
        g.add(0, 1, 1);
        g.add(1, 4, 1);
        g.add(2, 0, 1);
        g.add(3, 4, 1);
        g.add(3, 2, 1);

        let vec = vec![
            "A".to_string(),
            "B".to_string(),
            "D".to_string(),
            "E".to_string(),
            "C".to_string(),
        ];
        let ans = g.bfs(0);
        assert_eq!(vec, ans);
    }
}
