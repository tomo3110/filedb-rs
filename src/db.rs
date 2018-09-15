use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use dirs::home_dir;

use c::C;
use result::{Result, Error};

/// データベースファイルの拡張子
const FILEDB_EXT: &str = "filedb";


/// FileDB構造体
#[derive(Debug)]
pub struct FileDB {
    path: PathBuf,
    cmap: HashMap<PathBuf, Mutex<C>>
}

impl FileDB {
    /// filedbとのコネクションを作成
    /// 実際は接続などは行わず、アプリケーションへの組み込みデータストアなので、
    /// コンストラクタと同様の振る舞いとなります。
    pub fn connect<P: AsRef<OsStr>>(path: P) -> Result<Self> {
        let p = Path::new(&path);
        // 指定したディレクトリが存在しない時は、新規作成を行う。
        if !p.exists() {
            fs::create_dir_all(p)?;
        }
        if !p.is_dir() {
            return Err(
                Error::DBFile {
                    path: PathBuf::from(&path).to_string_lossy().to_string()
                }
            );
        }
        Ok(FileDB {
            path: PathBuf::from(&path), 
            cmap: HashMap::new()
        })
    }

    /// カラム一覧を取得する。
    pub fn col_names(&self) -> Result<Vec<String>> {
        let dir: fs::ReadDir = self.path.read_dir()?;
        let mut res: Vec<String> = Vec::new();
        for ent in dir {
            let ent: DirEntry = ent?;
            let ext = PathBuf::from(&ent.file_name())
                .extension()
                .map(|s| s.to_os_string())
                .unwrap_or_default();
            if ext == OsStr::new(FILEDB_EXT) {
                if let Some(file_name) = ent.file_name().to_str().map(|name| name.to_string()) {
                    res.push(file_name);
                }
            }
        }
        Ok(res)
    }

    /// カラムのハンドルクラスを取得する。
    pub fn c<'inc>(&'inc mut self, name: &'inc str) -> Result<&'inc Mutex<C>> {
        let path = self.get_path(name);

        if !self.cmap.contains_key(&path) {
            let file = if path.exists() {
                OpenOptions::new()
                    .append(true)
                    .read(true)
                    .open(&path)?
            } else {
                let parent = &path.parent().ok_or(Error::DBFile { path: name.to_string() })?;
                fs::create_dir_all(parent)?;
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .read(true)
                    .open(&path)?
            };
            self.cmap.insert(path.to_path_buf(), Mutex::new(C::new(&path, file)));
        }

        let path = self.get_path(name).to_str().unwrap_or_default().to_string();

        self.cmap.get(&self.get_path(name)).ok_or(Error::DBFile { path })
    }

    fn get_path(&self, name: &str) -> PathBuf {
        self.path.join(name).with_extension(FILEDB_EXT)
    }

}

impl Default for FileDB {
    fn default() -> Self {
        let path = home_dir().unwrap_or_default().join(".filedb");
        FileDB::connect(path).unwrap()
    }
}

