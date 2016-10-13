extern crate lmdb_zero as lmdb;
extern crate tempdir;

use lmdb_zero::traits::AsLmdbBytes;

#[allow(dead_code)]
#[inline]
pub fn lmdb_create_env(path: &str) -> lmdb::Environment {
    unsafe {
        let mut builder = lmdb::EnvBuilder::new().unwrap();
        builder.set_maxdbs(2).unwrap();
        builder.open(
            path,
            lmdb::open::Flags::empty(),
            0o600
        ).unwrap()
    }
}

#[allow(dead_code)]
#[inline]
pub fn lmdb_open(env: &lmdb::Environment) -> lmdb::Database {
    lmdb::Database::open(env, None, &lmdb::DatabaseOptions::defaults()).unwrap()
}

#[allow(dead_code)]
pub struct Cache<'a> {
    lmdb_env: &'a lmdb::Environment,
    lmdb_db: &'a lmdb::Database<'a>
}

impl<'a> Cache<'a> {
    #[inline]
    pub fn new (lmdb_env: &'a lmdb::Environment, lmdb_db: &'a lmdb::Database) -> Cache<'a> {
        Cache {
            lmdb_env: lmdb_env,
            lmdb_db: lmdb_db
        }
    }

    pub fn put (&mut self, key: &str, data: &Vec<u8>) {
        let tx = lmdb::WriteTransaction::new(&self.lmdb_env).unwrap();

        {
            let mut access = tx.access();

            access.put(&self.lmdb_db, key, data.as_lmdb_bytes(), lmdb::put::Flags::empty()).unwrap();
        }

        tx.commit().unwrap();
    }

    pub fn get (&mut self, key: &str) -> Option<Vec<u8>> {
        let tx = lmdb::ReadTransaction::new(&self.lmdb_env).unwrap();

        {
            let access = tx.access();

            let res: Result<&[u8], _> = access.get(&self.lmdb_db, key);

            match res {
                Ok(data) => Some(data.to_vec()),
                _ => None
            }
        }
    }

    /* for strings */

    pub fn put_str (&mut self, key: &str, data: &str) {
        let tx = lmdb::WriteTransaction::new(&self.lmdb_env).unwrap();

        {
            let mut access = tx.access();

            access.put(&self.lmdb_db, key, data.as_lmdb_bytes(), lmdb::put::Flags::empty()).unwrap();
        }

        tx.commit().unwrap();
    }

    pub fn get_str (&mut self, key: &str) -> Option<String> {
        let tx = lmdb::ReadTransaction::new(&self.lmdb_env).unwrap();

        {
            let access = tx.access();

            let res: Result<&str, _> = access.get(&self.lmdb_db, key);

            match res {
                Ok(data) => Some(data.to_string()),
                _ => None
            }
        }
    }
}
