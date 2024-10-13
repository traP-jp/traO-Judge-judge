use std::path::PathBuf;

use crate::custom_file::traits::File;

use super::TextFile;

#[test]
fn drop_from_leaves() -> anyhow::Result<()> {
    let v = {
        let a = TextFile::new(PathBuf::from("a.txt"), "hello!".to_string())?;

        let b1 = a.create_symlink_to(PathBuf::from("b1.txt"))?;
        let b2 = a.create_symlink_to(PathBuf::from("b2.txt"))?;
        let b3 = a.create_symlink_to(PathBuf::from("b3.txt"))?;

        let c1 = b1.create_symlink_to(PathBuf::from("c1.txt"))?;
        let c2 = b1.create_symlink_to(PathBuf::from("c2.txt"))?;
        let c3 = b1.create_symlink_to(PathBuf::from("c3.txt"))?;

        vec![a, b1, b2, b3, c1, c2, c3];
    };

    // たとえば
    // "b2.txt"→ "b3.txt"→ "c1.txt"→ "c2.txt"→ "c3.txt"→ "b1.txt"→ "a.txt"
    // のように，葉から drop することが確認できる．
    drop(v);

    Ok(())
}
