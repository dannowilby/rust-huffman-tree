
use std::io::BufWriter;
use std::io::BufReader;
use std::io::prelude::*;

use std::fs;
use std::fs::File;

pub use bitbit::writer::BitWriter;
pub use bitbit::reader::BitReader;
pub use bitbit::reader::MSB;

use crate::model::Huffman;

// serialize using json
pub fn write_tree_to_file(file: &str, tree: &Huffman) {

  let serialized = serde_json::to_string(&tree).unwrap();
  
  fs::write(file, serialized).ok();
}

pub fn read_tree_from_file(file: &str) -> Huffman {
  let contents = fs::read_to_string(file);
  let serialized = contents.unwrap();
  let tree = serde_json::from_str(&serialized).unwrap();

  tree
}

pub fn write_bits_to_file(file: &str, code: &str) {

  let f = File::create(file);
  let mut buf_writer = BufWriter::new(f.unwrap());
  let mut bw = BitWriter::new(&mut buf_writer);

  let vec: Vec<&str> = code.split("").collect();

  for n in vec {

    if n == "" {
      continue;
    }

    bw.write_bit(n == "1").ok();
  }

  buf_writer.flush().ok();
}

pub fn read_bits_from_file(file: &str) -> String {
  
  let mut s = String::from("");

  let r = File::open(file);
  let buff_reader = BufReader::new(r.unwrap());
  let mut br: BitReader<_, MSB> = BitReader::new(buff_reader);

  let mut b = br.read_bit();

  while (&b).is_ok() {
    
    if b.unwrap() {
      s.push_str("1");
    } else {
      s.push_str("0");
    }

    b = br.read_bit();
  }

  s
}

pub fn print_tree(tree: &Huffman, index: usize) {
  println!("{} - {} - {}", tree.nodes[index].letter, tree.nodes[index].frequency, tree.nodes[index].parent.is_some());

  if tree.nodes[index].left.is_some() {
    println!("left");
    print_tree(tree, tree.nodes[index].left.unwrap());
  }

  if tree.nodes[index].right.is_some() {
    println!("right");
    print_tree(tree, tree.nodes[index].right.unwrap());
  }
}

pub fn print_nodes(tree: &Huffman) {

  for index in 0..tree.nodes.len() {
    println!("{} - {} - {}", tree.nodes[index].letter, tree.nodes[index].frequency, tree.nodes[index].parent.is_some());
  }
}