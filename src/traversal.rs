use crate::graph::Graph;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

fn bfs<G, T, Pre, Post, Edge>(
    g: &G,
    n: usize,
    mut pre_process_node: Pre,
    mut post_process_node: Post,
    mut process_edge: Edge,
) where
    G: Graph<T>,
    Pre: FnMut(usize),
    Post: FnMut(usize),
    Edge: FnMut(usize, usize),
{
    let mut queue = VecDeque::new();
    queue.push_back(n);

    let mut marked = HashSet::new();
    let mut processed = HashSet::new();

    while let Some(u) = queue.pop_front() {
        pre_process_node(u);

        for v in g.edges(u) {
            if !processed.contains(v) {
                process_edge(u, *v);
            }
            if !marked.contains(v) {
                marked.insert(*v);
                queue.push_back(*v)
            }
        }

        post_process_node(u);
        processed.insert(u);
    }
}

pub fn shortest_path<G, T>(g: &G, u: usize, v: usize) -> Vec<usize>
where
    G: Graph<T>,
{
    let mut parents = HashMap::new();
    bfs(
        g,
        u,
        |_| {},
        |_| {},
        |u, v| {
            parents.insert(v, u);
        },
    );
    println!("{:?}", parents);
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::AdjGraph;

    #[test]
    fn test() {
        let mut g = AdjGraph::new(false);
        g.insert("a", "b");
        g.insert("a", "c");
        g.insert("b", "c");

        shortest_path(&g, 0, 2);
    }
}
