use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;
use graph::Graph;

type VertexRef = usize;

pub struct SymbolGraph<'a, T: 'a + Eq + Hash> { // will probably need lifetime references
    vertices: Vec<&'a T>,
    symbol_table: HashMap<&'a T, VertexRef>,
    graph: Graph // will probably need lifetime references
}

impl<'a, T: Eq + Hash> SymbolGraph<'a, T> {
    pub fn new() -> Self {
        SymbolGraph {
            vertices: Vec::new(),
            symbol_table: HashMap::new(),
            graph: Graph::new()
        }
    }

    pub fn v(&self) -> usize {
      self.graph.v()
    }

    pub fn e(&self) -> usize {
      self.graph.e()
    }

    // https://learning-rust.github.io/docs/c3.lifetimes.html
    // graph vertices len needs to stay in sync with keys len
    pub fn add_node(&mut self, data: &'a T) -> () {
        // handle data if already exists in table
        let graph_key = self.graph.add_node();
        
        let vertex_key = self.vertices.len();
        self.vertices.push(data);
        
        self.symbol_table.insert(data, graph_key);
        
        assert!(vertex_key == graph_key, "keys are out of sync!!!");
        ()
    }
    
    // fn add_nodes(&mut self, mut nodes: &Vec<T>) -> () {
    //     let start = self.vertices.len();
        
    //     self.vertices.append(&mut nodes);
        
    //     for i in start..self.vertices.len() {
    //         self.symbol_table.insert(&self.vertices[i], i);
    //         let graph_key = self.graph.add_node();
    //         assert!(i == graph_key, "keys are out of sync!!!");
    //     }
    // }

    pub fn add_edge(&mut self, v1: &'a T, v2: &'a T) -> () {
        // add handling if v1 and v2 don't exist
        let first = self.symbol_table.get(v1);
        let second = self.symbol_table.get(v2);

        if first.is_none() || second.is_none() {
          return ()
        }

        self.graph.add_edge(*first.unwrap(), *second.unwrap()); // hopefully this is safe
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_count_test() {

      let v1 = "Garrett";
      let v2 = "Joe";

      // sg has to be initialized after v1 and v2 or else v1 and v2 get dropped before sg, which causes an error
      let mut sg = SymbolGraph::new();

      assert_eq!(0, sg.v());

      sg.add_node(&v1);
      sg.add_node(&v2);

      assert_eq!(2, sg.v());
       
    }

    #[test]
    fn edge_count_test() {

      let v1 = "Garrett";
      let v2 = "Joe";
      let v3 = "Meg";
    
      let mut sg = SymbolGraph::new();

      assert_eq!(0, sg.e());

      sg.add_node(&v1);
      sg.add_node(&v2);

      sg.add_edge(&v2, &v1);

      assert_eq!(1, sg.e());

      sg.add_node(&v3);
      sg.add_edge(&v3, &v1);

      assert_eq!(2, sg.e());
    
    }
}