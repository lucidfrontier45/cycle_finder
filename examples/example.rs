use cycle_finder::Graph;

fn main() {
    let edges = [
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 6),
        (6, 1),
        (3, 7),
        (7, 8),
        (8, 2),
        (9, 10),
        (10, 11),
        (11, 9),
    ];

    let graph = Graph::from(edges);
    let cycles = graph.find_cycles();
    dbg!(cycles);
}
