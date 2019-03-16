type VertexRef = usize;

pub struct Graph {
    vertices: Vec<Vec<VertexRef>>,
    edge_count: usize
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            edge_count: 0
        }
    }

    pub fn v(&self) -> usize {
        self.vertices.len() // is this O(1)?
    }

    pub fn e(&self) -> usize {
        self.edge_count
    }

    pub fn add_node(&mut self) -> VertexRef {
        let vertex_ref = self.vertices.len();
        self.vertices.push(Vec::new());
        vertex_ref
    }

    pub fn add_edge(&mut self, v1: VertexRef, v2: VertexRef) -> () {
        if self.vertices.get(v1).is_none() || self.vertices.get(v2).is_none() {
            ()
        }
        self.vertices[v1].push(v2);
        self.vertices[v2].push(v1);

        self.edge_count += 1;
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_count_test() {
      let mut g = Graph::new();

      g.add_node();
      g.add_node();

      assert_eq!(2, g.v());

      g.add_node();

      assert_eq!(3, g.v());
    }

    #[test]
    fn edge_count_test() {
      let mut g = Graph::new();

      g.add_node();
      g.add_node();

      assert_eq!(0, g.e());

      g.add_edge(0, 1);

      assert_eq!(1, g.e());

      g.add_node();
      g.add_edge(2, 1);

      assert_eq!(2, g.e());
    }
}