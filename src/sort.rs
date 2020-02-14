
use crate::model::{ Huffman, Node };
use crate::tree::add_node;

pub fn bubble_sort_tree(tree: &mut Huffman) {

  while !is_sorted(&tree) {
    bubble_sort_pass(tree);
  }

}

pub fn is_sorted(tree: &Huffman) -> bool {

  for i in 0..(tree.nodes.len() - 1) {
    
    if tree.nodes[i].frequency > tree.nodes[i + 1].frequency {
      return false;
    }
  }

  true
}

pub fn bubble_sort_pass(tree: &mut Huffman) {

  let length = tree.nodes.len();

  for i in 0..(length - 1) {
    for j in 0..(length - i - 1) {

      if tree.nodes[j].frequency > tree.nodes[j + 1].frequency {

        let a1 = tree.nodes[j].frequency;
        let a2 = tree.nodes[j].letter.clone();
        let a3 = tree.nodes[j + 1].frequency;
        let a4 = tree.nodes[j + 1].letter.clone();
          
        tree.nodes[j].frequency = a3;
        tree.nodes[j].letter = a4;
        tree.nodes[j + 1].frequency = a1;
        tree.nodes[j + 1].letter = a2;
      }

    }
  }

}

/**
 * ignore is the index you want to skip over checking
 * for a huffman tree, you need the two minimum values
 */ 
pub fn find_min_excluding(tree: &Huffman, ignore: usize) -> usize {
  let mut min = tree.nodes.len() + 1;

  for n in 0..tree.nodes.len() {
    
    if n != ignore && !tree.nodes[n].parent.is_some() { 

      if min == (tree.nodes.len() + 1) {
        min = n;
      }

      if tree.nodes[n].frequency <= tree.nodes[min].frequency {
        min = n;
      }

    }
  }

  min
}

pub fn node_exists(nodes: &Vec<Node>, car: &str) -> bool {
  
  for n in nodes {
    if n.letter.to_lowercase() == car.to_lowercase() {
      return true;
    }
  }

  false
}

pub fn find_node(tree: &Huffman, letter: String) -> usize {

  for n in 0..tree.nodes.len() {
    
    if tree.nodes[n].letter.to_lowercase() == letter.to_lowercase() {

      return n;
    }
  }

  return 0;
}

pub fn sort_text(text: String) -> Huffman {

  let mut tree = Huffman { 
    nodes: Vec::new(),
    head_node: 0, 
  };

  let vec: Vec<&str> = text.split("").collect();

  for i in vec {

    if i == "" {
      continue;
    }
    
    if node_exists(&tree.nodes, i) {

      let n = find_node(&tree, String::from(i));
      
      tree.nodes[n].frequency += 1;
    } else {
      add_node(&mut tree, 1, i.to_string().to_lowercase());
    }
  }

  tree
}