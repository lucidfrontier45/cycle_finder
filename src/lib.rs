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
    fn get_parent_ids(&self, node_id: usize) -> &HashSet<usize> {
        &self.nodes.get(&node_id).unwrap().parent_ids
    }

    pub fn find_cycles(&self) -> Vec<Vec<usize>> {
        let mut searched_nodes = HashSet::new();
        let mut cycles: Vec<Vec<usize>> = vec![];

        for &root_node_id in self.nodes.keys() {
            let (found_cycles, _searched_nodes) = self.dfs_search(root_node_id, searched_nodes);
            searched_nodes = _searched_nodes;
            cycles.extend(found_cycles);
        }

        for cycle in cycles.iter_mut() {
            cycle.push(*cycle.first().unwrap());
        }

        cycles
    }

    fn dfs_search(
        &self,
        root_node_id: usize,
        mut searched_nodes: HashSet<usize>,
    ) -> (Vec<Vec<usize>>, HashSet<usize>) {
        let mut cycles = vec![];
        if searched_nodes.contains(&root_node_id) {
            return (cycles, searched_nodes);
        }
        searched_nodes.insert(root_node_id);

        let mut route = vec![];
        route.push(root_node_id);

        let mut dfs_stack = vec![];
        self.get_parent_ids(root_node_id).iter().for_each(|&id| {
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
                self.get_parent_ids(next_node_id)
                    .iter()
                    .for_each(|&next_next_id| {
                        dfs_stack.push((next_node_id, next_next_id));
                    });
            }
        }

        (cycles, searched_nodes)
    }
}

impl<T> From<T> for Graph
where
    T: IntoIterator<Item = Edge>,
{
    fn from(edges: T) -> Self {
        let mut nodes = HashMap::new();
        edges.into_iter().for_each(|(from_id, to_id)| {
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
        });
        Graph { nodes }
    }
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
