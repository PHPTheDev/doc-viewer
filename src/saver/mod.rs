use std::io;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tree_ds::prelude::*;
use crate::data::termo::NodeTermo;

pub fn save_as_json(tree: Tree<String, NodeTermo>, name: &str) {
    let tree_ser = serde_json::to_string_pretty(&tree);
    if tree_ser.is_ok() {
        let _f = File::create(name).expect("mds");
        let _ = fs::write(name, tree_ser.expect("nus"));
    };
}

pub fn get_from_json() -> io::Result<Tree<String, NodeTermo>>{
    let pai = Path::new(r"C:\Users\x558899\Documents\code\rust\rust-learn\p1.json");
    let file = File::open(pai);

    let mut contents = String::new();
    file?.read_to_string(&mut contents)?;

    let p: Tree<String, NodeTermo> = serde_json::from_str(&contents)?;

    save_as_json(p.clone(), "p2.json");
    Ok(p)
}