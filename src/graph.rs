use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Node<T: Clone> {
    id: Uuid,
    value: T,
}

impl<T: Clone> Node<T> {
    pub fn new(value: T) -> Node<T> {
        let id = Uuid::new_v4();
        Node { id, value }
    }
}

#[derive(Clone, Debug)]
pub struct Graph<T: Clone> {
    pub nodes: Vec<Node<T>>,
    pub edges: HashSet<(Uuid, Uuid)>,
}

impl<T: Clone> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            nodes: vec![],
            edges: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, new_node: Node<T>) -> Uuid {
        let output = new_node.id;
        self.nodes.push(new_node);
        output
    }

    pub fn add_node_with_value(&mut self, value: T) -> Uuid {
        let node = Node::new(value);
        let output = node.id;
        self.nodes.push(node);
        output
    }

    pub fn get_node_ids(&self) -> Vec<Uuid> {
        let mut ids = vec![];
        for node in &self.nodes {
            ids.push(node.id);
        }
        ids
    }

    pub fn get_node(&self, id: Uuid) -> Option<&Node<T>> {
        for node in &self.nodes {
            if node.id == id {
                return Some(node);
            }
        }
        None
    }

    pub fn get_node_mut(&mut self, id: Uuid) -> Option<&mut Node<T>> {
        for node in &mut self.nodes {
            if node.id == id {
                return Some(node);
            }
        }
        None
    }

    pub fn get_node_values(&self, ids: &[Uuid]) -> Vec<Option<T>> {
        ids.iter()
            .map(|x| self.get_node(*x).map(|node| node.value.clone()))
            .collect()
    }

    pub fn get_node_edges(&self, id: Uuid) -> HashSet<(Uuid, Uuid)> {
        let mut output = HashSet::new();
        for edge in &self.edges {
            if edge.0 == id || edge.1 == id {
                output.insert(*edge);
            }
        }
        output
    }

    pub fn get_node_neighbors(&self, id: Uuid) -> HashSet<Uuid> {
        let edges = self.get_node_edges(id);
        let mut output = HashSet::new();
        for edge in edges {
            if edge.0 == id {
                output.insert(edge.1);
            } else {
                output.insert(edge.0);
            }
        }
        output
    }

    pub fn add_edge(&mut self, node1: Uuid, node2: Uuid) {
        self.edges.insert((node1, node2));
    }
}

pub trait Connected {
    type Item;
    fn get_neighbors(&self, node: &Self::Item) -> Vec<Self::Item>;
}

pub fn shortest_path<T, U>(start: &T, end: &T, graph: &U) -> Option<Vec<T>>
where T: Clone + Eq + Hash,
      U: Connected<Item = T>,
{
    let mut paths: HashMap<T, Vec<T>> = HashMap::new();
    let mut investigate: HashSet<T> = HashSet::new();
    let mut visited: HashSet<T> = HashSet::new();

    investigate.insert(end.clone());
    paths.insert(end.clone(), vec![end.clone()]);
    loop {
        let mut investigate_next: HashSet<T> = HashSet::new();

        // Iterate over nodes to investigate
        for trial in &investigate {
            let neighbors = graph.get_neighbors(&trial);

            // Iterate over neighbors of trial node
            let curr_path = paths.get(trial).unwrap().clone();
            for n in neighbors {
                if let Some(p) = paths.get_mut(&n) {
                    // If there exists a path to n already, see if this one is
                    // shorter, insert if it is.
                    if p.len() > curr_path.len() + 1 {
                        let mut tpath = curr_path.clone();
                        tpath.push(n.clone());
                        *p = tpath;
                    }
                } else {
                    // If there is no path to n already, use this one
                    let mut tpath = curr_path.clone();
                    tpath.push(n.clone());
                    paths.insert(n.clone(), tpath);
                }

                // Visit n if we haven't already
                if !visited.contains(&n) {
                    investigate_next.insert(n);
                }
            }

            // Put trial in visited if it isn't already there
            visited.insert(trial.clone());
        }

        if investigate_next.is_empty() {
            break;
        } else {
            investigate = investigate_next;
        }
    }

    paths.get(&start).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes() {
        let n1 = Node::new(12);
        assert_eq!(n1.value, 12);
    }

    #[test]
    fn graphs() {
        let mut g = Graph::<i64>::new();
        let id0 = g.add_node(Node::new(12));
        assert_eq!(g.nodes[0].value, 12);

        let id1 = g.add_node_with_value(15);
        assert_eq!(g.nodes[1].value, 15);

        let id2 = g.add_node_with_value(1);
        let id3 = g.add_node_with_value(2);
        let id4 = g.add_node_with_value(3);

        // Grab references to a couple Nodes
        let node_ids = g.get_node_ids();

        g.add_edge(id0, id1);
        g.add_edge(id0, id2);
        let neighbors = g.get_node_neighbors(id0);
        let mut testset = HashSet::new();
        testset.insert(id1);
        testset.insert(id2);
        assert_eq!(neighbors, testset);
    }

    #[test]
    fn path_finding() {
        impl Connected for HashSet::<(usize, usize)> {
            type Item = (usize, usize);
            fn get_neighbors(&self, node: &Self::Item) -> Vec<Self::Item> {
                let mut candidates = vec![];
                candidates.push((node.0, node.1 + 1));
                candidates.push((node.0 + 1, node.1));
                if node.1 > 0 {
                    candidates.push((node.0, node.1 - 1));
                }
                if node.0 > 0 {
                    candidates.push((node.0 - 1, node.1));
                }

                let mut output = vec![];
                for candidate in candidates {
                    if self.contains(&candidate) {
                        output.push(candidate);
                    }
                }
                output
            }
        }
        let mut graph: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..20 {
            for j in 0..20 {
                graph.insert((i, j));
            }
        }
        let path = shortest_path(&(12, 16), &(15, 7), &graph).unwrap();
        assert_eq!(path.len() - 1, 12);
        let path = shortest_path(&(2, 16), &(15, 17), &graph).unwrap();
        assert_eq!(path.len() - 1, 14);
    }
}
