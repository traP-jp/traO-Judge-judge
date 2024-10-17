use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::custom_file::{
    text_file::{TextFileEntity, TextFileLink},
    traits::File,
};

#[test]
fn drop_not_leaf_node() -> anyhow::Result<()> {
    let a = Arc::new(RwLock::new(TextFileEntity::new(
        PathBuf::from("a.txt"),
        "hello".to_string(),
    )?));

    let b = TextFileLink::new(PathBuf::from("b.txt"), a.clone())?;

    let c = b.create_symlink_to(PathBuf::from("c.txt"))?;

    drop(b);

    println!("b is dropped.");

    Ok(())
}
