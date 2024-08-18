use crate::MGraph;

#[allow(unused)]
impl MGraph {
    fn dijkstra(&self, dist: &mut [i32], path: &mut [i32], start: usize) {
        let mut mark = vec![0; self.node_num];
        for i in 0..self.node_num {
            dist[i] = self.edges[start][i];
            if dist[i] < i32::MAX {
                path[i] = start as i32;
            } else {
                path[i] = -1;
            }
        }

        mark[start] = 1;
        path[start] = -1;
        dist[start] = 0;

        for _ in 0..self.node_num {
            let mut min = i32::MAX;
            let mut idx = 0;
            for j in 0..self.node_num {
                if mark[j] == 0 && dist[j] < min {
                    min = dist[j];
                    idx = j;
                }
            }
            mark[idx] = 1;
            for j in 0..self.node_num {
                if self.edges[idx][j] != i32::MAX {
                    let v = dist[idx] + self.edges[idx][j];
                    if mark[j] == 0 && v < dist[j] {
                        dist[j] = v;
                        path[j] = idx as i32;
                    }
                }
            }
        }
    }

    fn show(path: &[i32], mut pos: i32) -> Vec<i32> {
        let mut vec = Vec::new();
        while path[pos as usize] != -1 {
            vec.push(pos);
            pos = path[pos as usize];
        }
        vec.push(pos);
        let mut ans = Vec::new();
        while let Some(v) = vec.pop() {
            ans.push(v);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let show = vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
        ];
        let mut g = MGraph::new(show, true, i32::MAX);
        g.add(0, 1, 4);
        g.add(0, 2, 6);
        g.add(0, 3, 6);
        g.add(1, 4, 7);
        g.add(1, 2, 1);
        g.add(2, 4, 6);
        g.add(2, 5, 4);
        g.add(3, 2, 2);
        g.add(3, 5, 5);
        g.add(4, 6, 6);
        g.add(5, 4, 1);
        g.add(5, 6, 8);
        let mut dist = vec![0; g.node_num];
        let mut path = vec![0; g.node_num];
        g.dijkstra(&mut dist, &mut path, 0);
        let vec = MGraph::show(&path, 6);
        assert_eq!(vec![0, 1, 2, 5, 4, 6], vec)
    }
}
