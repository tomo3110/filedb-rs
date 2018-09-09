use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::path::PathBuf;

use tempfile::Builder;
use tempfile::NamedTempFile;

use result::Result;

/// C構造体  
/// カラムの管理を行う
#[derive(Debug)]
pub struct C {
    path: PathBuf,
    file: File,
}

/// カラム操作関数(select_each, for_each, remove_each)の返却値  
/// filedbでは返却値により、ファイルを削除や修正、処理の中断などをおこなうため必要となる。
pub mod callback {

    /// SelectResultValue構造体  
    /// select_each関数の返却値  
    #[derive(Clone, Debug, PartialEq)]
    pub struct SelectResultValue (pub bool, pub Vec<u8>, pub bool);
    impl SelectResultValue {
        /// SelectResultValue構造体のコンストラクタ
        pub fn new(inclode: bool, data: Vec<u8>, stop: bool) -> Self {
            SelectResultValue( inclode, data, stop )
        }
        /// SelectResultValue構造体をタプル構造体へ変換する.
        pub fn to_tuple(&self) -> (bool, Vec<u8>, bool) {
            (self.0, self.1.clone(), self.2)
        }
    }

    /// RemoveResultValue構造体   
    /// remove_each関数の返却値  
    #[derive(Clone, Debug, PartialEq)]
    pub struct RemoveResultValue (pub bool, pub bool);
    impl RemoveResultValue {
        /// RemoveResultValue構造体のコンストラクタ
        pub fn new(remove: bool, stop: bool) -> Self {
            RemoveResultValue( remove, stop )
        }
        /// RemoveResultValue構造体をタプル構造体へ変換する.
        pub fn to_tuple(&self) -> (bool, bool) {
            (self.0, self.1)
        }
    }

    /// ForEachResultValue構造体   
    /// for_each関数の返却値  
    #[derive(Clone, Debug, PartialEq)]
    pub struct ForEachResultValue(pub bool);
    impl ForEachResultValue {
        /// ForEachResultValue構造体のコンストラクタ
        pub fn new(stop: bool) -> Self {
            ForEachResultValue(stop)
        }
        /// ForEachResultValue構造体のコンストラクタ
        pub fn is_stop(&self) -> bool {
            self.0
        }
    }
}

impl C {
    /// C構造体のコンストラクタ
    pub fn new<P: AsRef<OsStr>>(path: P, file: File) -> Self {
        // let file = Arc::from(file);
        let path = PathBuf::from(&path);
        C { path, file }
    }
    
    /// C構造体のパス名(カラム名)取得
    pub fn get_path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }

    /// C構造体が管理しているカラムデータごと削除
    pub fn delete(&mut self) -> Result<()> {
        Ok(fs::remove_file(&self.path)?)
    }

    /// データを追加する
    pub fn insert(&mut self, buf: &[u8]) -> Result<()> {
        let mut file = BufWriter::new(&mut self.file);

        file.write_all(buf)?;
        writeln!(file, "")?;

        file.flush()?;

        Ok(())
    }

    /// データ追加,更新,削除が行える。
    /// しかし、扱いが複雑すぎる場合は専用の関するを利用する方が良い。
    pub fn select_each<F>(&mut self, mut f: F) -> Result<usize>
        where F: FnMut(usize, Vec<u8>) -> callback::SelectResultValue
    {
        use callback::SelectResultValue as ResultValue;

        let mut tmp: NamedTempFile = Builder::new().prefix("filedb").tempfile()?;
        let mut file = BufReader::new(&self.file);
        let mut index = 0;

        file.seek(SeekFrom::Start(0))?;

        for line in file.lines() {
            let line = line?;
            let ResultValue(remove, data, stop) = f(index, line.into_bytes());
            if !remove {
                tmp.write_all(&data)?;
                writeln!(tmp, "")?;
                tmp.flush()?;
            }
            if stop {
                break;
            }
            index += 1;
        }

        fs::remove_file(&self.path)?;

        fs::rename(tmp.path(), &self.path)?;

        Ok(index)
    }

    /// データの読み出しのみに利用できる。
    pub fn for_each<F>(&self, mut f: F) -> Result<usize>
        where F: FnMut(usize, Vec<u8>) -> callback::ForEachResultValue
    {

        let mut index = 0;
        let mut file = BufReader::new(&self.file);

        file.seek(SeekFrom::Start(0))?;

        for line in file.lines() {
            let line = line?;
            if f(index, line.into_bytes()).is_stop() {
                break;
            }
            index += 1;
        }

        Ok(index)
    }

    /// 削除処理用の関数
    /// クロージャにて受け取ったデータから削除を決めることが可能。
    pub fn remove_each<F>(&mut self, mut f: F) -> Result<usize>
        where F: FnMut(usize, Vec<u8>) -> callback::RemoveResultValue
    {
        use callback::SelectResultValue;

        let index = self.select_each(move |index, line| {
            let (remove, stop) = f(index, line.clone()).to_tuple();
            SelectResultValue::new(remove, line, stop)
        })?;

        Ok(index)

    }

}
