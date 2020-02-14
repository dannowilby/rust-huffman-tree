
use std::fs;

pub mod model;
pub mod io;
pub mod sort;
pub mod tree;
pub mod code;

fn create(file: &str) {

  let text = fs::read_to_string(&[file, ".txt"].concat()).unwrap();

  let mut tree = sort::sort_text(String::from(&text));
  
  tree::build_tree(&mut tree);

  let encoded = code::encode(&tree, String::from(&text));

  io::write_bits_to_file(&[file, ".encoded.dat"].concat(), &encoded);
  io::write_tree_to_file(&[file, ".tree.json"].concat(), &tree);
}

fn load(file: &str) {
  
  let tree = io::read_tree_from_file(&[file, ".tree.json"].concat());
  let decoded = code::decode(&tree, io::read_bits_from_file(&[file, ".encoded.dat"].concat()));
  
  fs::write(&[file, ".output.txt"].concat(), &decoded).ok();
}

fn main() {
  
  let file: &str = "data";

  create(file);
  
  load(file);
}
