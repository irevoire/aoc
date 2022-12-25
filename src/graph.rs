use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    hash::Hash,
    marker::PhantomData,
};

pub type Id = usize;

#[derive(Debug)]
pub enum Directed {}

#[derive(Debug)]
pub enum Undirected {}

#[derive(Debug, Clone)]
pub struct Graph<Value, Edge = (), Kind = Undirected> {
    nodes_ids: HashMap<Value, Id>,
    nodes: Vec<Option<Value>>,
    edges: Vec<Vec<(Id, Edge)>>,
    kind: PhantomData<Kind>,
}

impl<Value, Edge, Kind> Graph<Value, Edge, Kind>
where
    Value: Clone + PartialEq + Eq + Hash + std::fmt::Debug,
{
    /// O(1)
    pub fn insert_value(&mut self, value: Value) -> Id {
        if let Some(pos) = self.nodes_ids.get(&value) {
            *pos
        } else {
            let pos = self.nodes.len();
            self.nodes_ids.insert(value.clone(), pos);
            self.nodes.push(Some(value));
            self.edges.push(Vec::new());
            pos
        }
    }

    /// O(1)
    pub fn get_value(&self, id: Id) -> Option<&Value> {
        self.nodes[id].as_ref()
    }

    /// O(1)
    pub fn get_value_mut(&mut self, id: Id) -> Option<&mut Value> {
        self.nodes[id].as_mut()
    }

    /// O(1)
    pub fn get_id(&self, value: &Value) -> Option<Id> {
        self.nodes_ids.get(value).copied()
    }

    /// O(edges)
    pub fn delete_value(&mut self, id: Id) -> bool {
        debug_assert!(id < self.nodes.len());

        let value = std::mem::take(&mut self.nodes[id]);
        if value.is_none() {
            return false;
        }
        let value = value.unwrap();

        self.nodes_ids.remove(&value);
        self.edges[id].clear();
        for edge in &mut self.edges {
            edge.retain(|(i, _)| *i != id);
        }

        true
    }

    /// O(edges)
    pub fn delete_by_value(&mut self, value: &Value) -> bool {
        self.delete_value(self.get_id(&value).unwrap())
    }

    /// Dijkstra
    pub fn distance_between(&self, start: Id, end: Id) -> Option<usize> {
        let mut explored = HashSet::new();
        let mut to_explore = vec![(start, 0)];

        loop {
            to_explore.sort_unstable_by(|(_, left), (_, right)| Reverse(left).cmp(&Reverse(right)));
            if let Some((current, distance)) = to_explore.pop() {
                if current == end {
                    return Some(distance);
                }
                explored.insert(current);
                to_explore.extend(
                    self.edges[current]
                        .iter()
                        .filter(|(id, _)| !explored.contains(&id))
                        .map(|(id, _)| (*id, distance + 1)),
                );
            } else {
                return None;
            }
        }
    }

    pub fn generate_cache(&self) -> HashMap<(Id, Id), usize> {
        let mut map = HashMap::new();
        for left in 0..self.nodes.len() {
            for right in 0..self.nodes.len() {
                if let Some(distance) = self.distance_between(left, right) {
                    map.insert((left, right), distance);
                }
            }
        }
        map
    }

    /// Iterate over all the node in the graph with no defined order
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.nodes.iter().filter_map(|v| v.as_ref())
    }
}

impl<Value, Edge> Graph<Value, Edge, Undirected>
where
    Edge: Clone,
{
    pub fn new_undirected() -> Self {
        Self {
            nodes_ids: HashMap::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            kind: PhantomData,
        }
    }

    pub fn create_edge_with_data(&mut self, a: Id, b: Id, metadata: Edge) {
        if self.edges[a].iter().find(|(id, _)| b == *id).is_none() {
            self.edges[a].push((b, metadata.clone()));
        }
        if self.edges[b].iter().find(|(id, _)| a == *id).is_none() {
            self.edges[b].push((a, metadata));
        }
    }
}

impl<Value> Graph<Value, (), Undirected> {
    pub fn create_edge(&mut self, a: Id, b: Id) {
        self.create_edge_with_data(a, b, ());
    }
}

impl<Value, Edge> Graph<Value, Edge, Undirected>
where
    Value: Clone + PartialEq + Eq + Hash,
    Edge: PartialEq + Eq,
{
    /// O(edge)
    pub fn delete_edge(&mut self, from: Id, to: Id) -> Option<Edge> {
        debug_assert!(from < self.edges.len());
        debug_assert!(to < self.edges.len());

        if let Some(position) = self.edges[from].iter().position(|(i, _)| *i == to) {
            let left = self.edges[from].remove(position).1;
            let pos = self.edges[to]
                .iter()
                .position(|(i, _)| *i == from)
                .expect("Corrupted graph");
            let right = self.edges[to].remove(pos).1;
            debug_assert!(left == right, "left should be equal to right");
            Some(left)
        } else {
            None
        }
    }
}

impl<Value, Edge> Graph<Value, Edge, Directed> {
    pub fn new_directed() -> Self {
        Self {
            nodes_ids: HashMap::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            kind: PhantomData,
        }
    }

    pub fn create_edge_with_data(&mut self, a: Id, b: Id, metadata: Edge) {
        if self.edges[a].iter().find(|(id, _)| b == *id).is_none() {
            self.edges[a].push((b, metadata));
        }
    }
}

impl<Value, Edge> Graph<Value, Edge, Directed>
where
    Value: Clone + PartialEq + Eq + Hash,
{
    /// O(edge)
    pub fn delete_edge(&mut self, from: Id, to: Id) -> Option<Edge> {
        debug_assert!(from < self.edges.len());
        debug_assert!(to < self.edges.len());

        if let Some(position) = self.edges[from].iter().position(|(i, _)| *i == to) {
            Some(self.edges[from].remove(position).1)
        } else {
            None
        }
    }
}

impl<Value> Graph<Value, (), Directed> {
    pub fn create_edge(&mut self, a: Id, b: Id) {
        self.create_edge_with_data(a, b, ());
    }
}

impl<Value, Edge> std::fmt::Display for Graph<Value, Edge, Directed>
where
    Value: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "digraph a {{")?;
        for (idx, node) in self.nodes.iter().enumerate() {
            if let Some(node) = node {
                writeln!(f, "\t \"{}\" [label = \"{}\"]", idx, node)?;
            }
        }
        for (idx, edges) in self.edges.iter().enumerate() {
            for edge in edges {
                writeln!(f, "\t \"{}\" -> \"{}\"", idx, edge.0)?;
            }
        }
        writeln!(f, "}}")
    }
}

impl<Value, Edge> std::fmt::Display for Graph<Value, Edge, Undirected>
where
    Value: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "graph a {{")?;
        for (idx, node) in self.nodes.iter().enumerate() {
            if let Some(node) = node {
                writeln!(f, "\t \"{}\" [label = \"{}\"]", idx, node)?;
            }
        }
        let mut already_inserted = HashSet::new();

        for (left, edges) in self.edges.iter().enumerate() {
            for (right, _) in edges {
                if !already_inserted.contains(&(left, *right))
                    && !already_inserted.contains(&(*right, left))
                {
                    already_inserted.insert((left, *right));
                    writeln!(f, "\t \"{}\" -- \"{}\"", left, right)?;
                }
            }
        }
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;

    impl<Value, Edge> Graph<Value, Edge, Directed>
    where
        Value: PartialEq + Eq + Hash + Debug,
    {
        pub fn ensure_correctness(&self) {
            assert_eq!(self.nodes.len(), self.edges.len());
            let nb_nodes = self.nodes.iter().filter(|node| node.is_some()).count();
            assert_eq!(nb_nodes, self.nodes_ids.len());

            for (value, id) in &self.nodes_ids {
                assert_eq!(self.nodes[*id].as_ref().unwrap(), value);
                for (id, _) in &self.edges[*id] {
                    assert!(
                        self.nodes.get(*id).is_some(),
                        "An edge points to an unexisting node"
                    );
                    let node = self.nodes[*id].as_ref().unwrap();
                    assert_eq!(
                        self.nodes_ids.get(&node).unwrap(),
                        id,
                        "An edge points to an unexisting node"
                    );
                }
            }
        }
    }

    impl<Value, Edge> Graph<Value, Edge, Undirected>
    where
        Value: PartialEq + Eq + Hash + Debug,
        Edge: PartialEq + Eq + Debug,
    {
        pub fn ensure_correctness(&self) {
            assert_eq!(self.nodes.len(), self.edges.len());
            let nb_nodes = self.nodes.iter().filter(|node| node.is_some()).count();
            assert_eq!(nb_nodes, self.nodes_ids.len());

            for (value, id) in &self.nodes_ids {
                assert_eq!(self.nodes[*id].as_ref().unwrap(), value);
                for (eid, left) in &self.edges[*id] {
                    assert_eq!(
                        &self.edges[*eid].iter().find(|(i, _)| i == id).unwrap().1,
                        left
                    );
                    assert!(
                        self.nodes.get(*eid).is_some(),
                        "An edge points to an unexisting node"
                    );
                    let node = self.nodes[*eid].as_ref().unwrap();
                    assert_eq!(
                        self.nodes_ids.get(&node).unwrap(),
                        eid,
                        "An edge points to an unexisting node"
                    );
                }
            }
        }
    }

    #[test]
    fn basic_undirected() {
        let mut graph = Graph::new_undirected();
        let a = graph.insert_value("a");
        let b = graph.insert_value("b");
        let c = graph.insert_value("c");
        let d = graph.insert_value("d");
        graph.ensure_correctness();

        graph.create_edge(a, b);
        graph.create_edge(a, c);
        graph.create_edge(b, d);
        graph.create_edge(c, d);

        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        graph a {
        	 "0" [label = "a"]
        	 "1" [label = "b"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "0" -- "1"
        	 "0" -- "2"
        	 "1" -- "3"
        	 "2" -- "3"
        }
        "###);

        graph.delete_value(b);
        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        graph a {
        	 "0" [label = "a"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "0" -- "2"
        	 "2" -- "3"
        }
        "###);

        let edge = graph.delete_edge(a, d);
        assert!(edge.is_none());

        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        graph a {
        	 "0" [label = "a"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "0" -- "2"
        	 "2" -- "3"
        }
        "###);

        let edge = graph.delete_edge(a, c);
        assert!(edge.is_some());

        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        graph a {
        	 "0" [label = "a"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "2" -- "3"
        }
        "###);
    }

    #[test]
    fn basic_directed() {
        let mut graph = Graph::new_directed();
        let a = graph.insert_value("a");
        let b = graph.insert_value("b");
        let c = graph.insert_value("c");
        let d = graph.insert_value("d");
        graph.ensure_correctness();

        graph.create_edge(a, b);
        graph.create_edge(a, c);
        graph.create_edge(b, d);
        graph.create_edge(c, d);

        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        digraph a {
        	 "0" [label = "a"]
        	 "1" [label = "b"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "0" -> "1"
        	 "0" -> "2"
        	 "1" -> "3"
        	 "2" -> "3"
        }
        "###);

        graph.delete_value(b);
        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        digraph a {
        	 "0" [label = "a"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "0" -> "2"
        	 "2" -> "3"
        }
        "###);

        let edge = graph.delete_edge(a, d);
        assert!(edge.is_none());

        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        digraph a {
        	 "0" [label = "a"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "0" -> "2"
        	 "2" -> "3"
        }
        "###);

        let edge = graph.delete_edge(a, c);
        assert!(edge.is_some());

        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        digraph a {
        	 "0" [label = "a"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "2" -> "3"
        }
        "###);
    }

    #[test]
    fn distance_between() {
        let mut graph = Graph::new_directed();
        let a = graph.insert_value("a");
        let b = graph.insert_value("b");
        let c = graph.insert_value("c");
        let d = graph.insert_value("d");

        assert_eq!(graph.distance_between(a, b), None);

        graph.create_edge(a, b);
        graph.create_edge(a, c);
        graph.create_edge(b, d);
        graph.create_edge(c, d);

        assert_eq!(graph.distance_between(a, a), Some(0));
        assert_eq!(graph.distance_between(a, b), Some(1));
        assert_eq!(graph.distance_between(a, c), Some(1));
        assert_eq!(graph.distance_between(a, d), Some(2));

        assert_eq!(graph.distance_between(b, a), None);
        graph.create_edge(b, a);
        graph.create_edge(d, b);
        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        digraph a {
        	 "0" [label = "a"]
        	 "1" [label = "b"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "0" -> "1"
        	 "0" -> "2"
        	 "1" -> "3"
        	 "1" -> "0"
        	 "2" -> "3"
        	 "3" -> "1"
        }
        "###);
        assert_eq!(graph.distance_between(d, b), Some(1));
        assert_eq!(graph.distance_between(d, a), Some(2));
        assert_eq!(graph.distance_between(d, c), Some(3));

        graph.create_edge(d, c);
        graph.ensure_correctness();
        insta::assert_display_snapshot!(graph, @r###"
        digraph a {
        	 "0" [label = "a"]
        	 "1" [label = "b"]
        	 "2" [label = "c"]
        	 "3" [label = "d"]
        	 "0" -> "1"
        	 "0" -> "2"
        	 "1" -> "3"
        	 "1" -> "0"
        	 "2" -> "3"
        	 "3" -> "1"
        	 "3" -> "2"
        }
        "###);
        assert_eq!(graph.distance_between(d, c), Some(1));
    }
}
