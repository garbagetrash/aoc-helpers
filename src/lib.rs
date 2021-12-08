use uuid::Uuid;

struct Node<T> {
    id: Uuid,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        let id = Uuid::new_v4();
        Node { id, value }
    }
}

struct Edge<'a, T> {
    node1: &'a Node<T>,
    node2: &'a Node<T>,
}

struct Graph<'a, T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge<'a, T>>,
}

impl<'a, T> Graph<'a, T> {
    fn new() -> Graph<'a, T> {
        Graph { nodes: vec![], edges: vec![] }
    }

    fn add_node(&mut self, new_node: Node<T>) {
        self.nodes.push(new_node);
    }

    fn add_node_with_value(&mut self, value: T) {
        let node = Node::new(value);
        self.nodes.push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes_and_edges() {
        let n1 = Node::new(12);
        let n2 = Node::new(10);

        let e = Edge::<i64>{ node1: &n1, node2: &n2 };

        assert_eq!(e.node1.value, 12);
    }

    #[test]
    fn graphs() {
        let mut g = Graph::<i64>::new();
        g.add_node(Node::new(12));
        assert_eq!(g.nodes[0].value, 12);

        g.add_node_with_value(15);
        assert_eq!(g.nodes[1].value, 15);
    }
}
