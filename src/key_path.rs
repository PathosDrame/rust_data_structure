use crate::AGraph;

#[allow(unused)]
impl AGraph {
    fn key_path(&self) -> Option<(Vec<i32>, Vec<i32>)> {
        let mut etv = vec![0; self.node_num];
        let mut ltv = vec![0; self.node_num];
        let mut in_degree = vec![0; self.node_num];

        for i in 0..self.node_num {
            if let Some(node) = &self.nodes[i] {
                let mut first = &node.first;
                while let Some(edge) = first {
                    in_degree[edge.no as usize] += 1;
                    first = &edge.next;
                }
            }
        }

        let mut stack = Vec::new();
        let mut out = Vec::new();

        for (i, &v) in in_degree.iter().enumerate().take(self.node_num) {
            if v == 0 {
                stack.push(i);
                break;
            }
        }
        while let Some(idx) = stack.pop() {
            out.push(idx);
            if let Some(node) = &self.nodes[idx] {
                let mut first = &node.first;
                while let Some(edge) = first {
                    in_degree[edge.no as usize] -= 1;
                    if in_degree[edge.no as usize] == 0 {
                        stack.push(edge.no as usize);
                    }
                    if etv[idx] + edge.weight > etv[edge.no as usize] {
                        etv[edge.no as usize] = etv[idx] + edge.weight;
                    }
                    first = &edge.next;
                }
            }
        }
        if out.len() < self.node_num {
            return None;
        }
        for v in ltv.iter_mut().take(self.node_num) {
            *v = etv[out.len()];
        }
        while let Some(idx) = out.pop() {
            if let Some(node) = &self.nodes[idx] {
                let mut first = &node.first;
                while let Some(edge) = first {
                    if ltv[edge.no as usize] - edge.weight < ltv[idx] {
                        ltv[idx] = ltv[edge.no as usize] - edge.weight;
                    }
                    first = &edge.next;
                }
            }
        }
        Some((etv, ltv))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let shows = vec![
            "V0".to_string(),
            "V1".to_string(),
            "V2".to_string(),
            "V3".to_string(),
            "V4".to_string(),
            "V5".to_string(),
            "V6".to_string(),
            "V7".to_string(),
            "V8".to_string(),
        ];
        let mut g = AGraph::new(shows, true);
        g.add(0, 1, 6);
        g.add(0, 2, 4);
        g.add(0, 3, 5);
        g.add(1, 4, 1);
        g.add(2, 4, 1);
        g.add(3, 5, 2);
        g.add(4, 6, 9);
        g.add(4, 7, 7);
        g.add(5, 7, 4);
        g.add(6, 8, 2);
        g.add(7, 8, 4);
        if let Some((etv, ltv)) = g.key_path() {
            assert_eq!(vec![0, 6, 4, 5, 7, 7, 16, 14, 18], etv);
            assert_eq!(vec![0, 6, 6, 8, 7, 10, 16, 14, 18], ltv);
        }
    }
}
