use crate::graph::{build_graph, EdgeKind, Node};
use std::error::Error;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

// Very overkill implementation for day 7. Should just use
// an adjacency list of (node, vec![(size, node)].
// Wanting something like Neo4j. I couldn't recall how they
// implemented their dict (set or table? hindsight table would
// have worked better). 
mod graph;

fn main() -> Result<(), Box<dyn Error>> {
  let bag_graph = build_graph()?;

  let gold_bag = bag_graph.nodes
      .get(&Node {
        name: "shiny_gold_bag".to_string(),
      })
      .unwrap();

  let part_one = bag_graph
      .get_unique_ancestors(gold_bag.clone())
      .unwrap()
      .len();

  let mut part_two = 0;
  let mut children = vec![gold_bag.clone()];

  while let Some(child) = children.pop() {
    if let Some(sub_bags) = bag_graph.get_children(child) {
      for edge in sub_bags {
        let EdgeKind::Contains(num) = edge.verb;
        part_two += num;
        for _ in 0..num {
          children.push(edge.to.clone())
        }
      }
    }
  }

  println!("{:?}", part_one);
  println!("{:?}", part_two);

  Ok(())
}
