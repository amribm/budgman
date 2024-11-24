use crate::sql::DB;
use std::{fs, io, path::PathBuf};
use thiserror::Error;

pub struct BudgMan {
    db: DB,
}

impl BudgMan {
    pub fn new() -> Result<BudgMan, BudgManError> {
        let home = env!("HOME");

        let db_path = format!("{}/.config/budgman/data.db", home);
        let mut new_file = false;

        let path = PathBuf::from(db_path);

        // creats a db file if now exisita
        if !path.exists() {
            let dir_path = PathBuf::from(format!("{}/.config/budgman", home));
            if !dir_path.exists() {
                fs::DirBuilder::new().create(dir_path)?;
            }
            let _ = fs::File::create(path.clone())?;
            new_file = true;
        }

        let db = DB::new(path)?;

        if new_file {
            db.init()?;
        }

        Ok(BudgMan { db })
    }
}

#[derive(Error, Debug)]
pub enum BudgManError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    DBError(#[from] rusqlite::Error),
}
