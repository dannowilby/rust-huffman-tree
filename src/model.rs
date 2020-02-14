
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Huffman {
  pub nodes: Vec<Node>,
  pub head_node: usize,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Node {

  pub frequency: u32,
  pub letter: String,

  pub left: Option<usize>,
  pub right: Option<usize>,
  pub parent: Option<usize>,
}