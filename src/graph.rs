use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub trait Graph<T> {
    fn num_of_nodes(&self) -> usize;
    fn insert(&mut self, u: T, v: T);
    fn num_of_edges(&self) -> usize;
    fn edges(&self, n: usize) -> &Vec<usize>;
    fn index(&self, n: &T) -> usize;
    fn key(&self, i: usize) -> &T;
}

#[derive(Debug)]
pub struct AdjGraph<T>
where
    T: Eq + Hash,
{
    adj: Vec<Vec<usize>>,
    nodes: HashMap<Rc<T>, usize>,
    keys: Vec<Rc<T>>,
    is_direct: bool,
}

impl<T> AdjGraph<T>
where
    T: Eq + Hash,
{
    pub fn new(is_direct: bool) -> Self {
        AdjGraph {
            adj: Vec::new(),
            nodes: HashMap::new(),
            keys: Vec::new(),
            is_direct,
        }
    }

    fn insert_adj(&mut self, u: usize, v: usize) {
        if u >= self.adj.len() {
            self.adj.push(vec![]);
        }

        self.adj[u].push(v);
    }

    fn insert_node(&mut self, n: T) -> usize {
        let size = self.nodes.len();
        let n = Rc::new(n);
        let i = self.nodes.entry(Rc::clone(&n)).or_insert(size);
        if *i >= self.keys.len() {
            self.keys.push(Rc::clone(&n));
        } else {
            self.keys[*i] = Rc::clone(&n);
        }
        *i
    }
}

impl<T> Graph<T> for AdjGraph<T>
where
    T: Eq + Hash,
{
    fn insert(&mut self, u: T, v: T) {
        let u = self.insert_node(u);
        let v = self.insert_node(v);

        self.insert_adj(u, v);
        if !self.is_direct {
            self.insert_adj(v, u);
        }
    }

    fn num_of_nodes(&self) -> usize {
        self.adj.len()
    }

    fn num_of_edges(&self) -> usize {
        let s = self.adj.iter().map(|v| v.len()).sum();
        if self.is_direct {
            s
        } else {
            s / 2
        }
    }

    fn edges(&self, n: usize) -> &Vec<usize> {
        &self.adj[n]
    }

    fn index(&self, n: &T) -> usize {
        self.nodes[n]
    }

    fn key(&self, i: usize) -> &T {
        &self.keys[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut g = AdjGraph::new(false);
        g.insert("a", "b");
        g.insert("b", "c");
        g.insert("c", "d");
        g.insert("b", "d");

        assert_eq!(2, Rc::strong_count(&g.keys[0]));
        assert_eq!(4, g.num_of_nodes());
        assert_eq!(4, g.num_of_edges());

        assert_eq!(3, g.index(&"d"));
        assert_eq!(&vec![2, 1], g.edges(3));
    }
}
