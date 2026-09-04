use std::fs;
use std::error::Error;
use std::env;
use std::path::Path;
use std::result::Result;
use rayon::prelude::*;
use std::io;
use tree_ds::prelude::*;
use std::fs::File;
use std::io::prelude::*;
mod data;
use data::termo::{NodeTermo, Termo};
 
const ROOT_PATH: &str = r"C:\Users\x558899\Documents\Termos de Folhas scaneadas";
 
fn build_tree() -> io::Result<(Tree<String, NodeTermo>, String)> {
    let mut tree: Tree<String, NodeTermo> = Tree::new(Some("Files Tree"));
 
    // The full path string doubles as the node's unique ID and as the value
    // we hand back to ourselves so children know who their parent node is.
    let root_id = tree
        .add_node(
            Node::new(ROOT_PATH.to_string(), Some( NodeTermo { termo: Termo::new(), path: ROOT_PATH.to_string() ,depth: 0, dir: true})),
            None,
        )
        .expect("failed to add root node");
 
    add_children(Path::new(ROOT_PATH), &root_id, 1, &mut tree)?;
 
    Ok((tree, root_id))
}
 
/// Walks dir and attaches every entry it finds as a child of parent_id.
/// When an entry is itself a directory, we recurse into it using the node
/// we just created as the new parent — this is what actually produces the
/// hierarchy instead of a flat list.
fn add_children(
    dir: &Path,
    parent_id: &String,
    prof: usize,
    tree: &mut Tree<String, NodeTermo>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.display().to_string();
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = path.is_dir();
        println!("{:?}", entry);
 
        let node_id = tree
            .add_node(
                Node::new(path_str, Some(NodeTermo { termo: Termo::new(), path: name, depth: prof, dir: is_dir })),
                Some(parent_id), // <-- attached to its real parent, not root
            )
            .expect("failed to add node");
 
        if is_dir {
            add_children(&path, &node_id, prof + 1, tree)?;
        }
    }
    Ok(())
}
 
fn print_tree(tree: &Tree<String, NodeTermo>, root_id: &String) {
    tree.traverse(root_id, TraversalStrategy::PreOrder)
        .unwrap()
        .iter()
        .for_each(|node_id| {
            let node = tree.get_node_by_id(node_id).unwrap();
            if let Some(term) = node.get_value().unwrap() {
                let name = term.path;
                let indent = "  ".repeat(term.depth);
                let marker = if term.dir { "[dir] " } else { "[file]" };
                println!("{indent}{marker} {name} -----: {:?}", term.termo);
            }
        });
}

fn save_as_json(tree: Tree<String, NodeTermo>, /*root_id: &String*/) {
    let tree_ser = serde_json::to_string_pretty(&tree);
    if tree_ser.is_ok() {
        println!("{}", tree_ser.as_ref().ok().unwrap())
    };

    let mut f = File::create("mds.json").expect("mds");

    fs::write("mds.json", tree_ser.expect("nus"));


    //tree.traverse(root_id, TraversalStrategy::PreOrder)
    //    .unwrap()
    //    .iter()
    //    .for_each(|node_id| {
    //        let node = tree.get_node_by_id(node_id).unwrap();
    //        let node_ser = serde_json::to_string(&node)
    //    })
}
 
fn main() {
    match build_tree() {
        Ok((tree, _root_id)) => save_as_json(tree)/*print_tree(&tree, &root_id)*/,
        Err(e) => eprintln!("Failed to walk directory: {e}"),
    }
}