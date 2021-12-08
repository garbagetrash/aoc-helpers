use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone, Debug)]
struct Node<T: Clone> {
    id: Uuid,
    value: T,
}

impl<T: Clone> Node<T> {
    fn new(value: T) -> Node<T> {
        let id = Uuid::new_v4();
        Node { id, value }
    }
}

#[derive(Clone, Debug)]
struct Graph<T: Clone> {
    nodes: Vec<Node<T>>,
    edges: HashSet<(Uuid, Uuid)>,
}

impl<T: Clone> Graph<T> {
    fn new() -> Graph<T> {
        Graph {
            nodes: vec![],
            edges: HashSet::new(),
        }
    }

    fn add_node(&mut self, new_node: Node<T>) -> Uuid {
        let output = new_node.id;
        self.nodes.push(new_node);
        output
    }

    fn add_node_with_value(&mut self, value: T) -> Uuid {
        let node = Node::new(value);
        let output = node.id;
        self.nodes.push(node);
        output
    }

    fn get_node_ids(&self) -> Vec<Uuid> {
        let mut ids = vec![];
        for node in &self.nodes {
            ids.push(node.id);
        }
        ids
    }

    fn get_node(&self, id: Uuid) -> Option<&Node<T>> {
        for node in &self.nodes {
            if node.id == id {
                return Some(node);
            }
        }
        None
    }

    fn get_node_mut(&mut self, id: Uuid) -> Option<&mut Node<T>> {
        for node in &mut self.nodes {
            if node.id == id {
                return Some(node);
            }
        }
        None
    }

    fn get_node_values(&self, ids: &[Uuid]) -> Vec<Option<T>> {
        ids.iter()
            .map(|x| self.get_node(*x).map(|node| node.value.clone()))
            .collect()
    }

    fn get_node_edges(&self, id: Uuid) -> HashSet<(Uuid, Uuid)> {
        let mut output = HashSet::new();
        for edge in &self.edges {
            if edge.0 == id || edge.1 == id {
                output.insert(*edge);
            }
        }
        output
    }

    fn get_node_neighbors(&self, id: Uuid) -> HashSet<Uuid> {
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

    fn add_edge(&mut self, node1: Uuid, node2: Uuid) {
        self.edges.insert((node1, node2));
    }
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
}
