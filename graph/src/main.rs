
struct edge {
    target: usize,
    weight: usize,
}
struct node<T> {
    data: T,
    outgoing: Vec<edge>,
    incoming: Vec<edge>,
}

struct graph<T> {
    nodes: Vec<node<T>>,
}

fn add_node<T>(g: &mut graph<T>, data: T) -> usize {
    let index = g.nodes.len();
    g.nodes.push(node {
        data,
        outgoing: Vec::new(),
        incoming: Vec::new(),
    });
    index
}

fn add_edge<T>(g: &mut graph<T>, from: usize, to: usize, weight: usize) {
    g.nodes[from].outgoing.push(edge { target: to, weight });
    g.nodes[to].incoming.push(edge { target: from, weight });
}

fn remove_edge<T>(g: &mut graph<T>, from: usize, to: usize) {
    g.nodes[from].outgoing.retain(|e| e.target != to);
    g.nodes[to].incoming.retain(|e| e.target != from);
}


fn main() {
    let mut g = graph { nodes: Vec::new() };
    let node1 = add_node(&mut g, "Node 1");
    let node2 = add_node(&mut g, "Node 2");
    add_edge(&mut g, node1, node2, 5);
}
