use std::collections::HashMap;

struct Edge<Node: Clone> {
    pub from: Node,
    pub to: Node,
}
pub fn has_cycle<Node: Eq + std::hash::Hash + Clone>(
    edges: Vec<Edge<Node>>,
) -> anyhow::Result<bool> {
    let mut flag: anyhow::Result<bool> = Ok(false);
    let edges_size = edges.len();
    let mut NodeIndex: HashMap<Node, usize> = HashMap::new();
    let mut new_index: usize = 0;
    for edge in &edges {
        let From: Node = edge.from.clone();
        let To: Node = edge.to.clone();
        if NodeIndex.get(&From) == None {
            NodeIndex.insert(From, new_index);
            new_index += 1;
        }
        if NodeIndex.get(&To) == None {
            NodeIndex.insert(To, new_index);
            new_index += 1;
        }
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![]; new_index];
    for edge in &edges {
        let From_clone: Node = edge.from.clone();
        let Option_from: Option<&usize> = NodeIndex.get(&From_clone);
        let mut index_from = match Option_from {
            Some(n) => Ok(n),
            None => Err(anyhow::anyhow!("Node not found")),
        }?;
        let From: usize = index_from.clone();
        let To_clone: Node = edge.to.clone();
        let Option_to: Option<&usize> = NodeIndex.get(&To_clone);
        let mut index_to = match Option_to {
            Some(n) => Ok(n),
            None => Err(anyhow::anyhow!("Node not found")),
        }?;
        let To: usize = index_to.clone();
        graph[From].push(To);
    }
    let Node_size = new_index;
    let mut visited: Vec<bool> = vec![false; Node_size];
    let mut visiting: Vec<bool> = vec![false; Node_size];
    for start in 0..Node_size {
        if visited[start] {
            continue;
        }
        if dfs(start, &graph, &mut visiting, &mut visited) {
            flag = Ok(true);
            break;
        }
    }
    return flag;
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
