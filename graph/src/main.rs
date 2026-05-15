
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
    if g.nodes[from].outgoing.iter().any(|e| e.target == to) {
        println!("Edge from {} to {} already exists.", from, to);
        return;
    }
    g.nodes[from].outgoing.push(edge { target: to, weight });
    g.nodes[to].incoming.push(edge { target: from, weight });
}

fn remove_edge<T>(g: &mut graph<T>, from: usize, to: usize) {
    g.nodes[from].outgoing.retain(|e| e.target != to);
    g.nodes[to].incoming.retain(|e| e.target != from);
}

fn traverse_graph<T>(g: &graph<T>, start: usize) {
    let mut visited = vec![false; g.nodes.len()];
    let mut stack = vec![start];

    while let Some(node_index) = stack.pop() {
        if !visited[node_index] {
            visited[node_index] = true;
            print!("Node {}: ", node_index);
            print!("Outgoing edges: {:?} ", g.nodes[node_index].outgoing.iter().map(|e| e.target).collect::<Vec<_>>());
            println!("Incoming edges: {:?}", g.nodes[node_index].incoming.iter().map(|e| e.target).collect::<Vec<_>>());
            for edge in &g.nodes[node_index].outgoing {
                stack.push(edge.target);
            }
        }
    }
}

fn remove_node<T>(g: &mut graph<T>, index: usize) {
    for node in &mut g.nodes {
        node.outgoing.retain(|e| e.target != index);
        node.incoming.retain(|e| e.target != index);
    }

    g.nodes.remove(index);

    for node in &mut g.nodes {
        for edge in &mut node.outgoing {
            if edge.target > index {
                edge.target -= 1;
            }
        }
        for edge in &mut node.incoming {
            if edge.target > index {
                edge.target -= 1;
            }
        }
    }
}

fn main() {
    let mut g = graph { nodes: Vec::new() };
    for i in 0..10 {
        add_node(&mut g, format!("Node {}", i));
    }
    for i in 0..7 {
        add_edge(&mut g, i, i + 2, 1);
        add_edge(&mut g, i + 1, i + 3, 1);
    }
    for i in 0..6 {
        add_edge(&mut g, i, i + 3, 2);
    }
    add_edge(&mut g, 0, 1, 5);
    traverse_graph(&g, 0);
}
