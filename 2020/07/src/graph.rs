use crate::LINE_ENDING;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

pub struct Graph {
  pub nodes: HashSet<Rc<Node>>,
  pub edges: HashSet<Edge>,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Node {
  pub name: String,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum EdgeKind {
  Contains(usize),
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Edge {
  pub verb: EdgeKind,
  pub from: Rc<Node>,
  pub to: Rc<Node>,
}

impl Graph {
  pub fn new() -> Self {
    Self {
      nodes: Default::default(),
      edges: Default::default(),
    }
  }
  pub fn add_node(&mut self, node: Node) -> bool {
    self.nodes.insert(Rc::new(node))
  }
  pub fn add_edge(&mut self, edge: Edge) -> bool {
    self.edges.insert(edge)
  }

  pub fn get_unique_ancestors(&self, node: Rc<Node>) -> Option<Vec<Rc<Node>>> {
    if let Some(ancestors) = self.get_all_ancestors(node) {
      Some(ancestors.iter().cloned().unique().collect_vec())
    } else {
      None
    }
  }

  fn get_all_ancestors(&self, node: Rc<Node>) -> Option<Vec<Rc<Node>>> {
    if let Some(mut parents) = self.get_parents(node.clone()) {
      let mut grandparents = parents.iter()
          .map(|p| self.get_all_ancestors(p.clone()))
          .filter(|p| p.is_some())
          .map(|p| p.unwrap())
          .flatten()
          .collect_vec();

      parents.append(&mut grandparents);
      Some(parents)
    } else {
      None
    }
  }

  pub fn get_children(&self, node: Rc<Node>) -> Option<Vec<Edge>> {
    self.edges
        .iter()
        .filter(|&edge| edge.from == node)
        .map(|e| Some(e.clone()))
        .collect::<Option<Vec<_>>>()
  }

  pub fn get_parents(&self, node: Rc<Node>) -> Option<Vec<Rc<Node>>> {
    self.edges
        .iter()
        .filter(|&edge| edge.to == node)
        .map(|e| Some(e.from.clone()))
        .collect::<Option<Vec<Rc<Node>>>>()
  }
}

pub fn build_graph() -> Result<Graph, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let mut bags = Graph::new();
  input.split(&format!("{}", LINE_ENDING)).for_each(|line| {
    let split = line.split(" contain ").collect_vec();
    let bag = split[0].replace(" ", "_");
    let source = Node {
      name: bag[0..bag.len() - 1].to_string(),
    };
    bags.add_node(source.clone());
    let edges = split[1].replace(".", "");
    if !edges.contains("no other bags") {
      for dest in edges.split(", ") {
        let dest = dest.replace("bags", "bag");
        let mut dest_iter = dest.split(" ");
        let num = dest_iter.next().unwrap().parse::<usize>().unwrap();
        let name = dest_iter.join("_");
        let dest_node = Node { name };
        bags.add_node(dest_node.clone());
        bags.add_edge(Edge {
          verb: EdgeKind::Contains(num),
          from: bags.nodes.get(&source).unwrap().clone(),
          to: bags.nodes.get(&dest_node).unwrap().clone(),
        });
      }
    }
  });

  Ok(bags)
}
