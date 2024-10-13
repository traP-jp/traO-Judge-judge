#[cfg(test)]
mod tests;

use std::{
    io::Write,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use super::traits::File;

struct FileAccess;

struct TextFileInner {
    path: PathBuf,
    _parent: Option<TextFile>,
    lock: Arc<RwLock<FileAccess>>,
}

impl Drop for TextFileInner {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        // ./text_file/tests.rs でのテスト用
        eprintln!("{:?}", self.path);
    }
}

// TextFileInner を Arc でラップして使う．
//
// シンボリックリンク自体を子，リンク先を親と表現するとき，子は Arc<親> を持つ．
// （これにより子から drop することが保証されるはず？）
//
// オリジナルファイルには Arc<RwLock<FileAccess>> を新たに作成して持たせる．
// シンボリックリンク（子や孫すべて）にはその clone を持たせる．
// （元ファイルを根とする根付き木全体で 1 つの RwLock<FileAccess> を共有しているはず？）
#[derive(Clone)]
struct TextFile(Arc<TextFileInner>);

impl File for TextFile {
    type InitArgs = String;
    fn new(path: PathBuf, args: Self::InitArgs) -> anyhow::Result<Self> {
        // ファイルの作成・初期化
        let mut file = std::fs::File::create(&path)?;
        file.write_all(args.as_bytes())?;

        // オリジナルファイルなので，親は存在しない．
        // Arc<RwLock<FileAccess>> を新たに作成して持たせる．
        let inner = TextFileInner {
            path,
            _parent: None,
            lock: Arc::new(RwLock::new(FileAccess)),
        };

        Ok(TextFile(Arc::new(inner)))
    }
    fn create_symlink_to(&self, path: PathBuf) -> anyhow::Result<Self> {
        // シンボリックリンクの作成
        std::os::unix::fs::symlink(&self.0.path, &path)?;

        // 自身の clone を子に持たせる．
        // Arc<RwLock<FileAccess>> の clone を持たせる．
        let inner = TextFileInner {
            path,
            _parent: Some(self.clone()),
            lock: self.0.lock.clone(),
        };

        Ok(TextFile(Arc::new(inner)))
    }
}

impl Drop for TextFile {
    fn drop(&mut self) {
        // Arc<TextFileInner> が drop したときに呼び出すと，複数回呼び出されてしまう．
        // TextFileInner が drop したときに呼び出されるようにしたい．
    }
}
