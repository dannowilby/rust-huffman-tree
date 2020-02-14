
use crate::model::Huffman;
use crate::sort::find_node;

/** 
 * left = 0,
 * right = 1
*/
pub fn get_direction(tree: &Huffman, node: usize) -> String {

  if tree.nodes[node].parent.is_some() {

    let rl = tree.nodes[tree.nodes[node].parent.unwrap()].right.unwrap();

    return ((node == rl) as i32).to_string();
  }

  String::from("-1")
}

pub fn encode(tree: &Huffman, text: String) -> String {
 
  let mut output: String = String::from("");
  let vec: Vec<&str> = text.split("").collect();

  for i in vec {

    if i == "" {
      continue;
    }

    let mut n = find_node(tree, String::from(i.to_lowercase()));
    let mut s = get_direction(tree, n);

    let mut character_binary = String::from("");

    while s != "-1" {

      character_binary.push_str(&s);
      
      n = tree.nodes[n].parent.unwrap();
      s = get_direction(tree, n);
    }
    
    character_binary = character_binary.chars().rev().collect();

    output.push_str(&character_binary);
  }

  output
}

pub fn decode(tree: &Huffman, code: String) -> String {

  let mut output = String::from("");
  let mut current = &tree.nodes[tree.head_node];

  let vec: Vec<&str> = code.split("").collect();

  for n in vec {

    if n == "0" && current.left.is_some() {
      current = &tree.nodes[current.left.unwrap()];
    }

    if n == "1" && current.right.is_some() {
      current = &tree.nodes[current.right.unwrap()];
    }

    if current.letter != "" {

      output.push_str(&current.letter);

      current = &tree.nodes[tree.head_node];
    }
  }

  output
}