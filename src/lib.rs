use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

pub struct Node {
    id: usize,
    child_ids: HashSet<usize>,
    parent_ids: HashSet<usize>,
}

impl Node {
    fn new(id: usize, child_ids: HashSet<usize>, parent_ids: HashSet<usize>) -> Self {
        Self {
            id,
            child_ids,
            parent_ids,
        }
    }
}

type Edge = (usize, usize);

pub struct Graph {
    nodes: HashMap<usize, Node>,
}

impl Graph {
    pub fn from_edges(edges: &[Edge]) -> Graph {
        let mut nodes = HashMap::new();
        for &(from_id, to_id) in edges {
            if !nodes.contains_key(&from_id) {
                let node = Node::new(from_id, HashSet::new(), HashSet::new());
                nodes.insert(from_id, node);
            }
            if !nodes.contains_key(&to_id) {
                let node = Node::new(to_id, HashSet::new(), HashSet::new());
                nodes.insert(to_id, node);
            }
            nodes.get_mut(&from_id).unwrap().child_ids.insert(to_id);
            nodes.get_mut(&to_id).unwrap().parent_ids.insert(from_id);
        }
        Graph { nodes }
    }

    fn get_parent_ids(&self, node_id: usize) -> &HashSet<usize> {
        &self.nodes.get(&node_id).unwrap().parent_ids
    }
}

pub fn find_cycles(graph: &Graph) -> Vec<Vec<usize>> {
    let mut searched_nodes = HashSet::new();
    let mut cycles: Vec<Vec<usize>> = vec![];

    for &root_node_id in graph.nodes.keys() {
        dfs_search(graph, root_node_id, &mut searched_nodes, &mut cycles);
    }

    for cycle in cycles.iter_mut() {
        cycle.push(*cycle.first().unwrap());
    }

    cycles
}

fn rewind_route(mut route: Vec<usize>, current_node_id: usize) -> Vec<usize> {
    // go back the route untill the current_node_id
    loop {
        if *route.last().unwrap() == current_node_id {
            break;
        }
        route.pop();
    }
    route
}

fn dfs_search(
    graph: &Graph,
    root_node_id: usize,
    searched_nodes: &mut HashSet<usize>,
    cycles: &mut Vec<Vec<usize>>,
) {
    if searched_nodes.contains(&root_node_id) {
        return;
    }
    searched_nodes.insert(root_node_id);

    let mut route = vec![];
    route.push(root_node_id);

    let mut dfs_stack = vec![];
    graph.get_parent_ids(root_node_id).iter().for_each(|&id| {
        dfs_stack.push((root_node_id, id));
    });

    while !dfs_stack.is_empty() {
        let (current_node_id, next_node_id) = dfs_stack.pop().unwrap();

        route = rewind_route(route, current_node_id);

        if searched_nodes.contains(&next_node_id) {
            if let Some(idx) = route.iter().position(|&i| i == next_node_id) {
                let new_cycle = route[idx..].to_vec();
                cycles.push(new_cycle);
            } else {
                for cycle in cycles.iter() {
                    let root_place = cycle.iter().position(|&i| i == root_node_id);
                    let next_place = cycle.iter().position(|&i| i == next_node_id);
                    if let (Some(root_idx), Some(next_idx)) = (root_place, next_place) {
                        let small_idx = min(root_idx, next_idx);
                        let large_idx = max(root_idx, next_idx);
                        let mut new_cycle = route.clone();
                        new_cycle.extend_from_slice(&cycle[large_idx..]);
                        new_cycle.extend_from_slice(&cycle[..small_idx]);
                        cycles.push(new_cycle);
                        break;
                    }
                }
            }
        } else {
            route.push(next_node_id);
            searched_nodes.insert(next_node_id);
            graph
                .get_parent_ids(next_node_id)
                .iter()
                .for_each(|&next_next_id| {
                    dfs_stack.push((next_node_id, next_next_id));
                });
        }
    }
}
