use std::collections::HashMap;

pub struct Edge<Node: Clone> {
    pub from: Node,
    pub to: Node,
}
pub fn has_cycle<Node: Eq + std::hash::Hash + Clone>(
    edges: Vec<Edge<Node>>,
) -> anyhow::Result<bool> {
    let mut flag: anyhow::Result<bool> = Ok(false);
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
