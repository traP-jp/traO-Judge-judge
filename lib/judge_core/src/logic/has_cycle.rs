use std::collections::HashMap;

pub struct Edge<Node: Clone> {
    pub from: Node,
    pub to: Node,
}
pub fn has_cycle<Node: Eq + std::hash::Hash + Clone>(
    edges: Vec<Edge<Node>>,
) -> anyhow::Result<bool> {
    let mut flag: bool = false;
    let mut node_index: HashMap<Node, usize> = HashMap::new();
    let mut new_index: usize = 0;
    for edge in &edges {
        let from: Node = edge.from.clone();
        let to: Node = edge.to.clone();
        if node_index.get(&from) == None {
            node_index.insert(from, new_index);
            new_index += 1;
        }
        if node_index.get(&to) == None {
            node_index.insert(to, new_index);
            new_index += 1;
        }
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![]; new_index];
    for edge in &edges {
        let from_clone: Node = edge.from.clone();
        let option_from: Option<&usize> = node_index.get(&from_clone);
        let index_from = match option_from {
            Some(n) => Ok(n),
            None => Err(anyhow::anyhow!("Node not found")),
        }?;
        let from: usize = index_from.clone();
        let to_clone: Node = edge.to.clone();
        let option_to: Option<&usize> = node_index.get(&to_clone);
        let index_to = match option_to {
            Some(n) => Ok(n),
            None => Err(anyhow::anyhow!("Node not found")),
        }?;
        let to: usize = index_to.clone();
        graph[from].push(to);
    }
    let node_size = new_index;
    let mut visited: Vec<bool> = vec![false; node_size];
    let mut visiting: Vec<bool> = vec![false; node_size];
    for start in 0..node_size {
        if visited[start] {
            continue;
        }
        if dfs(start, &graph, &mut visiting, &mut visited) {
            flag = true;
            break;
        }
    }
    return Ok(flag);
}

fn dfs(
    pos: usize,
    graph: &Vec<Vec<usize>>,
    visiting: &mut Vec<bool>,
    visited: &mut Vec<bool>,
) -> bool {
    if visiting[pos] == true {
        return true;
    }
    visiting[pos] = true;
    let mut re = false;
    for next in graph[pos].clone() {
        if visited[next] {
            continue;
        }
        if dfs(next, graph, visiting, visited) {
            re = true;
            break;
        }
    }
    visiting[pos] = false;
    visited[pos] = true;
    return re;
}

#[cfg(test)]
mod tests {
    #[rstest]
    
    fn test() {
        let edges1 = vec![ // has cycle
            Edge { from: "A", to: "B" },
            Edge { from: "B", to: "C" },
            Edge { from: "C", to: "D" },
            Edge { from: "D", to: "B" },
        ];
        assert_eq!(has_cycle(edge1), true);
        let edges2 = vec![ // has cycle
            Edge { from: -1000, to: 3526 },
            Edge { from: 6750, to: -4567 },
            Edge { from: -4567, to: 3526 },
            Edge { from: -1000, to: -9999 },
            Edge { from: 987654321, to: -1234567890 },
            Edge { from: 987654321, to: 6750 },
            Edge { from: 6750, to: 0 },
            Edge { from: 0, to: 987654321 },
        ];
        assert_eq!(has_cycle(edge2), true);
        let edges3 = vec![ // has cycle 非連結グラフ
            Edge { from: 1, to: 2 },
            Edge { from: 2, to: 3 },
            Edge { from: 4, to: 5 },
            Edge { from: 5, to: 6 },
            Edge { from: 6, to: 4 },
            Edge { from: 5, to: 7 },
        ];
        assert_eq!(has_cycle(edge3), true);
        let edges4 = vec![ // No cycle
            Edge { from: 1, to: 2 },
            Edge { from: 2, to: 4 },
            Edge { from: 3, to: 4 },
            Edge { from: 4, to: 7 },
            Edge { from: 5, to: 6 },
            Edge { from: 6, to: 7 },
            Edge { from: 1, to: 8 },
            Edge { from: 8, to: 9 },
        ];
        assert_eq!(has_cycle(edge4), false);
        let edges5 = vec![ // No cycle 非連結グラフ
            Edge { from: 'a', to: 'b' },
            Edge { from: 'c', to: 'b' },
            Edge { from: 'd', to: 'b' },
            Edge { from: 'c', to: 'e' },
            Edge { from: 'f', to: 'g' },
            Edge { from: 'g', to: 'h' },
            Edge { from: 'h', to: 'i' },
        ];
        assert_eq!(has_cycle(edge5), false);
    }
}