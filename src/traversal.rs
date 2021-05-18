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
                queue.push_back(*v);
            }
        }

        post_process_node(u);
        processed.insert(u);
    }
}

pub fn find_path<'a, G, T>(g: &'a G, u: &T, v: &T) -> Vec<&'a T>
where
    G: Graph<T>,
{
    let u = g.index(u);
    let v = g.index(v);
    let mut parents = HashMap::new();
    bfs(
        g,
        u,
        |_| {},
        |_| {},
        |u, v| {
            parents.entry(v).or_insert(u);
        },
    );
    let mut path = vec![];
    let mut v = v;
    while let Some(&p) = parents.get(&v) {
        path.push(v);
        v = p;
    }
    path.push(u);
    path.reverse();
    path.iter().map(|i| g.key(*i)).collect()
}

pub fn connected_components<G, T>(g: &G) -> usize
where
    G: Graph<T>,
{
    let mut discovered = HashSet::new();
    let mut c = 0;
    for i in 0..g.num_of_nodes() {
        if !discovered.contains(&i) {
            bfs(
                g,
                i,
                |u| {
                    discovered.insert(u);
                },
                |_| {},
                |_, _| {},
            );
            c += 1;
        }
    }
    c
}

pub fn bipartite<G, T>(g: &G) -> bool
where
    G: Graph<T>,
{
    let uncolored = 0;
    let white = 1;
    let black = 2;

    let mut color = vec![uncolored; g.num_of_nodes()];
    let mut discovered = HashSet::new();
    let mut is_bipartite = true;
    for i in 0..g.num_of_nodes() {
        if !discovered.contains(&i) {
            color[i] = white;
            bfs(
                g,
                i,
                |u| {
                    discovered.insert(u);
                },
                |_| {},
                |u, v| {
                    if color[u] == color[v] {
                        is_bipartite = false;
                    }
                    if color[u] == white {
                        color[v] = black;
                    }
                    if color[u] == black {
                        color[v] = white;
                    }
                },
            )
        }
    }
    is_bipartite
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::AdjGraph;

    #[test]
    fn find_path() {
        let mut g = AdjGraph::new(false);
        g.insert('f', 'a');
        g.insert('a', 'b');
        g.insert('a', 'e');
        g.insert('b', 'e');
        g.insert('b', 'c');
        g.insert('e', 'd');
        g.insert('c', 'd');

        let path = super::find_path(&g, &'f', &'c');

        assert_eq!(vec![&'f', &'a', &'b', &'c'], path);
    }

    #[test]
    fn connected_components() {
        let mut g = AdjGraph::new(false);
        g.insert('f', 'a');
        g.insert('a', 'b');
        g.insert('a', 'e');
        g.insert('b', 'e');
        g.insert('b', 'c');
        g.insert('e', 'd');
        g.insert('c', 'd');
        g.insert('x', 'y');
        g.insert('y', 'z');

        assert_eq!(2, super::connected_components(&g));
    }

    #[test]
    fn bipartite() {
        let mut g = AdjGraph::new(false);

        g.insert('a', 'b');
        g.insert('b', 'c');
        g.insert('c', 'd');
        g.insert('d', 'a');
        g.insert('e', 'f');
        g.insert('f', 'g');
        g.insert('g', 'h');
        g.insert('h', 'e');
        g.insert('a', 'e');
        g.insert('b', 'f');
        g.insert('d', 'h');
        g.insert('c', 'g');

        assert!(super::bipartite(&g));

        g.insert('e', 'g');
        assert_eq!(false, super::bipartite(&g));
    }
}
