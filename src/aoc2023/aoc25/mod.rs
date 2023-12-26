use std::collections::HashMap;

use petgraph::{
    algo::{bellman_ford, connected_components},
    dot::Dot,
    graph::NodeIndex,
    Graph, Undirected,
};

static INPUT: &'static str = include_str!("./input");

fn parse() -> Graph<&'static str, f32, Undirected> {
    let mut graph = Graph::<&str, f32, Undirected>::new_undirected();

    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();

    for line in INPUT.lines() {
        let mut split = line.split(":");

        let s = split.next().unwrap();
        let start = if let Some(index) = nodes.get(s) {
            *index
        } else {
            let res = graph.add_node(s);
            nodes.insert(s, res);
            res
        };

        let connected: Vec<&str> = split
            .next()
            .unwrap()
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();

        for other in connected {
            if vec!["xkf", "rcn"].contains(&s) && vec!["xkf", "rcn"].contains(&other) {
                continue;
            }
            if vec!["thk", "cms"].contains(&s) && vec!["thk", "cms"].contains(&other) {
                continue;
            }
            if vec!["dht", "xmv"].contains(&s) && vec!["dht", "xmv"].contains(&other) {
                continue;
            }
            // if vec!["cmg", "bvb"].contains(&s) && vec!["cmg", "bvb"].contains(&other) {
            //     continue;
            // }
            // if vec!["nvd", "jqt"].contains(&s) && vec!["nvd", "jqt"].contains(&other) {
            //     continue;
            // }
            // if vec!["pzl", "hfx"].contains(&s) && vec!["pzl", "hfx"].contains(&other) {
            //     continue;
            // }

            let end = if let Some(index) = nodes.get(other) {
                *index
            } else {
                let res = graph.add_node(other);
                nodes.insert(other, res);
                res
            };

            graph.add_edge(start, end, 1.0);
        }
    }

    graph
}

pub fn one() -> Option<String> {
    let graph = parse();

    println!("{}", Dot::new(&graph));
    // After looking at graph in viz-js with neato layout engine, remove edges between nodes that
    // split the graph
    // for example input, this is cmg-bvb, nvd-jqt, pzl-hfx
    assert_eq!(2, connected_components(&graph));
    if let Ok(res) = bellman_ford(&graph, graph.node_indices().next().unwrap()) {
        let g1 = res.distances.iter().filter(|d| d.is_infinite()).count();
        let g2 = res.distances.iter().filter(|d| d.is_finite()).count();

        return Some((g1 * g2).to_string());
    }

    None
}

pub fn two() -> Option<String> {
    None
}

mod tests {}
