struct Node<T> {
    value: T,
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
        let node = Node::<T>{ value: value };
        self.nodes.push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes_and_edges() {
        let n1 = Node::<i64>{ value: 12 };
        let n2 = Node::<i64>{ value: 10 };

        let e = Edge::<i64>{ node1: &n1, node2: &n2 };

        assert_eq!(e.node1.value, 12);
    }

    #[test]
    fn graphs() {
        let mut g = Graph::<i64>::new();
        g.add_node(Node::<i64>{ value: 12 });
        assert_eq!(g.nodes[0].value, 12);

        g.add_node_with_value(15);
        assert_eq!(g.nodes[1].value, 15);
    }
}
