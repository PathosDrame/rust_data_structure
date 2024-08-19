use crate::AGraph;

#[allow(unused)]
impl AGraph {
    fn topological_sort(&self) -> i32 {
        let mut in_degree = vec![0; self.node_num];
        for i in 0..self.node_num {
            if let Some(node) = &self.nodes[i] {
                let mut t = &node.first;
                while let Some(next) = t {
                    in_degree[next.no as usize] += 1;
                    t = &next.next;
                }
            }
        }

        let mut vec = Vec::new();

        for (i, v) in in_degree.iter().enumerate() {
            if v == &0 {
                vec.push(i);
            }
        }

        let mut count = 0;
        while let Some(idx) = vec.pop() {
            count += 1;
            if let Some(node) = &self.nodes[idx] {
                print!("{}\t", node.show);
                let mut t = &node.first;
                while let Some(next) = t {
                    in_degree[next.no as usize] -= 1;
                    if in_degree[next.no as usize] == 0 {
                        vec.push(next.no as usize);
                    }
                    t = &next.next;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let shows = vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
        ];
        let mut g = AGraph::new(shows, false);
        g.add(0, 1, 1);
        g.add(0, 2, 1);
        g.add(0, 3, 1);
        g.add(1, 2, 1);
        g.add(1, 4, 1);
        g.add(2, 4, 1);
        g.add(2, 5, 1);
        g.add(3, 5, 1);
        g.add(4, 6, 1);
        g.add(5, 4, 1);
        g.add(5, 6, 1);

        let result = g.topological_sort();
        assert_eq!(7, result);
    }
}
