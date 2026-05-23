#[derive(Debug)]
pub enum GraphError {
    NodeNotFound(usize),
    EdgeAlreadyExists(usize),
    EdgeNotFound(usize),
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(index) => write!(f, "Node {} not found.", index),
            GraphError::EdgeAlreadyExists(target,) => write!(f, "Edge target {} already exists.", target),
            GraphError::EdgeNotFound(target ) => write!(f, "Edge target {} not found.", target),
        }
    }
}

impl std::error::Error for GraphError {}

struct Edge {
    target: usize,
    weight: usize,
}
struct Node<T> {
    data: T,
    outgoing: Vec<Edge>,
    incoming: Vec<Edge>,
    deleted: bool,
}

struct Graph<T> {
    nodes: Vec<Node<T>>,
    free_nodes: Vec<usize>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            outgoing: Vec::new(),
            incoming: Vec::new(),
            deleted: false,
        }
    }

    fn add_outgoing_edge(&mut self, target: usize, weight: usize) -> Result<(), GraphError> {
        if self.outgoing.iter().any(|e| e.target == target) {
            println!("Edge to {} already exists.", target);
            return Err(GraphError::EdgeAlreadyExists(target));
        }
        self.outgoing.push(Edge { target, weight });
        Ok(())
    }

    fn add_incoming_edge(&mut self, source: usize, weight: usize) -> Result<(), GraphError> {
        if self.incoming.iter().any(|e| e.target == source) {
            println!("Edge from {} already exists.", source);
            return Err(GraphError::EdgeAlreadyExists(source));
        }
        self.incoming.push(Edge { target: source, weight });
        Ok(())
    }

    fn remove_outgoing_edge(&mut self, target: usize) {
        self.outgoing.retain(|e| e.target != target);
    }

    fn remove_incoming_edge(&mut self, source: usize) {
        self.incoming.retain(|e| e.target != source);
    }
}


impl<T> Graph<T> {
    fn get_node(&self, index: usize) -> Option<&Node<T>> {
        self.nodes.get(index).filter(|n| !n.deleted)
    }

    fn traverse(&self, start: usize) {
        let mut visited = vec![false; self.nodes.len()];
        let mut stack = vec![start];

        while let Some(node_index) = stack.pop() {
            if !visited[node_index] {
                visited[node_index] = true;
                print!("Node {}: ", node_index);
                print!("Outgoing edges: {:?} ", self.nodes[node_index].outgoing.iter().map(|e| e.target).collect::<Vec<_>>());
                println!("Incoming edges: {:?}", self.nodes[node_index].incoming.iter().map(|e| e.target).collect::<Vec<_>>());
                for edge in &self.nodes[node_index].outgoing {
                    stack.push(edge.target);
                }
            }
        }
    }

    fn add_node(&mut self, data: T) -> usize {
        let index = if let Some(free_index) = self.free_nodes.pop() {
            free_index
        } else {
            self.nodes.len()
        };
        self.nodes.insert(index, Node::new(data));
        index
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: usize) -> Result<(), GraphError> {
        let res: Result<(), GraphError>;
        if let Some(node) = self.nodes.get_mut(from) && !node.deleted {
            let res = node.add_outgoing_edge(to, weight);
            if res.is_err() {
                return res;
            }
        } else {
            println!("Source node {} not found or deleted.", from);
            return Err(GraphError::NodeNotFound(from));
        };
        if let Some(node) = self.nodes.get_mut(to) && !node.deleted {
            res = node.add_incoming_edge(from, weight);
            if !res.is_err() {
                return Ok(());
            }
        } else {
            println!("Target node {} not found or deleted.", to);
            res = Err(GraphError::NodeNotFound(to));
        }
        
        /* Undo the edge add above */
        let node = self.nodes.get_mut(from).unwrap();
        node.remove_outgoing_edge(to);
        return res;
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        if let Some(source_node) = self.nodes.get_mut(from) {
            source_node.remove_outgoing_edge(to);
        }
        if let Some(target_node) = self.nodes.get_mut(to) {
            target_node.remove_incoming_edge(from);
        }
    }

    fn remove_node(&mut self, index: usize) {
        let Some(node) = self.nodes.get_mut(index) else {
            println!("Node {} does not exist.", index);
            return;
        };
        if node.deleted {
            println!("Node {} is already deleted.", index);
            return;
        }

        let (incoming_edges, outgoing_edges) = {
            node.deleted = true;
            self.free_nodes.push(index);
            (std::mem::take(&mut node.incoming), std::mem::take(&mut node.outgoing)) 
        };

        for edge in incoming_edges {
            if let Some(target_node) = self.nodes.get_mut(edge.target) {
                target_node.remove_outgoing_edge(index);
            }
        }
        for edge in outgoing_edges {
            if let Some(target_node) = self.nodes.get_mut(edge.target) {
                target_node.remove_incoming_edge(index);
            }
        }
    }
}


fn main() {
    let mut g = Graph {
            nodes: Vec::new(),
            free_nodes: Vec::new()
    };
    let start = g.add_node(format!("Start"));
    for i in 0..10 {
        g.add_node(format!("Node {}", i));
    }
    for i in 0..7 {
        let _ = g.add_edge(i, i + 2, 1);
        let _ = g.add_edge(i + 1, i + 3, 1);
    }
    for i in 0..6 {
        let _ = g.add_edge(i, i + 3, 2);
    }
    let _ = g.add_edge(0, 1, 5);
    g.traverse(start);

    println!("\nRemoving edge from 8 to 10 and node 10...");
    g.remove_edge(8, 10);
    g.remove_node(10);
    g.traverse(start);
}
