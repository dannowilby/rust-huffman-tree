
use crate::model::Huffman;
use crate::model::Node;
use crate::sort::find_min_excluding;

pub fn add_node(tree: &mut Huffman, frequency: u32, letter: String) -> usize {

  let index = tree.nodes.len();

  tree.nodes.push(Node {
    frequency: frequency,
    letter: letter,

    left: None,
    right: None,
    parent: None,
  });

  index
}

pub fn add_internal_node(tree: &mut Huffman, n1: usize, n2: usize) -> usize {

  let head = tree.nodes.len();

  let nn = Node {
    letter: "".to_string(),
    frequency: tree.nodes[n1].frequency + tree.nodes[n2].frequency,

    left: Some(n1),
    right: Some(n2),
    parent: None,
  };

  tree.nodes[n1].parent = Some(head);
  tree.nodes[n2].parent = Some(head);

  tree.nodes.push(nn);

  head
}

fn amount_unlinked(tree: &Huffman) -> u32 {

  let mut amount = 0;

  for n in 0..tree.nodes.len() {
    if !tree.nodes[n].parent.is_some() {
      amount += 1;
    }
  }

  amount
}

pub fn build_tree(tree: &mut Huffman) {

  let mut head = 0usize;

  let t = tree;

  while amount_unlinked(t) != 1 {

    let n1 = find_min_excluding(t, 0);
    let n2 = find_min_excluding(t, n1);
    
    head = add_internal_node(t, n1, n2);
  }
  t.head_node = head;
}