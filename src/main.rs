pub mod data;
pub mod saver;
use crate::saver::{save_as_json, get_from_json};
use crate::data::tree::build_tree;

 
const ROOT_PATH: &str = r"C:\Users\x558899\Documents\Termos de Folhas scaneadas";

 
fn main() {
    match build_tree(&ROOT_PATH.to_string()) {
        Ok((tree, _root_id)) => save_as_json(tree, "p1.json")/*print_tree(&tree, &root_id)*/,
        Err(e) => eprintln!("Failed to walk directory: {e}"),
    }
    get_from_json().expect("nn foi");
}