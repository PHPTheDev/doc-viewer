use std::fs;
use std::io;
use std::path::Path;
use tree_ds::prelude::*;
use crate::data::termo::{NodeTermo, Termo};

pub fn build_tree(p: &String) -> io::Result<(Tree<String, NodeTermo>, String)> {
    let mut tree: Tree<String, NodeTermo> = Tree::new(Some("Files Tree"));

    let root_id = tree
        .add_node(
            Node::new(p.to_string(), Some( NodeTermo { termo: Termo::new(), path: p.clone() ,depth: 0, dir: true})),
            None,
        )
        .expect("failed to add root node");
 
    add_children(Path::new(p), &root_id, 1, &mut tree)?;
 
    Ok((tree, root_id))
}
 
fn add_children<T: AsRef<Path>>(
    dir: T,
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
        println!("{}", entry.path().as_path().to_string_lossy());
 
        let node_id = tree
            .add_node(
                Node::new(path_str, Some(NodeTermo { termo: Termo::new(), path: name, depth: prof, dir: is_dir })),
                Some(parent_id), 
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


// fn cmp_trees(l: Tree<String, NodeTermo>, r: Tree<String, NodeTermo>){
//     let lsize = l.get_nodes().len();
//     let rsize = r.get_nodes().len();
//
//     if lsize > rsize{
//         save_as_json(l, "update.json")
//     } else {
//         unimplemented!();
//     }
//
// }