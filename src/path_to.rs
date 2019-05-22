use std::collections::VecDeque;
use graph::Graph;

type VertexRef = usize;


struct Path {
  edges: Vec<usize>,
}

impl Path {
  pub fn new() -> Self {
    Path { 
      edges: Vec::new(),
    }
  }
}

impl Iterator for Path {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    self.edges.pop()
  }
}

pub struct BreadthFirstPaths {
  marked: Vec<bool>,
  edge_to: Vec<Option<VertexRef>>,
  source: VertexRef
}

impl BreadthFirstPaths {
  pub fn new(graph: &Graph, source: VertexRef) -> Self {

    let bfs = |graph: &Graph, source: VertexRef| {
      let mut marked = vec![false; graph.v()];
      let mut edge_to = vec![None; graph.v()];

      let mut queue = VecDeque::new();

      queue.push_back(source);
      marked[source] = true;

      while !queue.is_empty() {

        let next = queue.pop_front().unwrap(); //shouldn't be None

        for vertex in &graph.vertices[next] {
          let v = *vertex;

          if !marked[v] {
            edge_to[v] = Some(next);
            queue.push_back(v);
            marked[v] = true;
          }
        }
      }
      (marked, edge_to)
    };

    let (marked, edge_to) = bfs(graph, source);

    BreadthFirstPaths {
      marked: marked,
      edge_to: edge_to,
      source: source
    }
  }

  pub fn has_path_to(&self, vertex: usize) -> bool {
    self.marked.get(vertex).map_or(false, |val| *val)
  }

  pub fn path_to(&self, vertex: usize) -> Option<impl Iterator<Item = usize>> {
    if !self.has_path_to(vertex) {
      return None;
    }

    let mut path = Path::new();
    let mut next = vertex;

    while next != self.source {
      path.edges.push(next);
      next = self.edge_to[next].unwrap(); //shouldn't have a None if there is a path 
    }
    path.edges.push(self.source);

    Some(path)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_has_path_to() {
    let mut graph = Graph::new();

    (0..=8).for_each(|_| {
      graph.add_node();
    });
    
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(3, 5);
    graph.add_edge(5, 8);
    graph.add_edge(3, 7);
    graph.add_edge(5, 6);
    graph.add_edge(4, 8);

    let paths_for_7 = BreadthFirstPaths::new(&graph, 7);

    assert!(!paths_for_7.has_path_to(0));
    assert!(!paths_for_7.has_path_to(2));
    assert!(paths_for_7.has_path_to(1));
    assert!(paths_for_7.has_path_to(3));
    assert!(paths_for_7.has_path_to(4));
    assert!(paths_for_7.has_path_to(5));
    assert!(paths_for_7.has_path_to(6));
    assert!(paths_for_7.has_path_to(8));
  }

  #[test]
  fn test_path_to() {

    let mut graph = Graph::new();

    (0..=8).for_each(|_| {
      graph.add_node();
    });
    
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(3, 5);
    graph.add_edge(5, 8);
    graph.add_edge(3, 7);
    graph.add_edge(5, 6);
    graph.add_edge(4, 8);

    let paths_for_7 = BreadthFirstPaths::new(&graph, 7);

    let shortest_path_from_7_to_0 = paths_for_7.path_to(0);

    assert!(shortest_path_from_7_to_0.is_none());

    let mut path = paths_for_7.path_to(8).unwrap();

    assert_eq!(Some(7 as usize), path.next());
    assert_eq!(Some(3 as usize), path.next());
    assert_eq!(Some(5 as usize), path.next());
    assert_eq!(Some(8 as usize), path.next());
  }

  // #[test]
  // fn test_symbol_path_to() {

  // }
}