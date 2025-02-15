use crate::conn::DB;
use crate::crypto::ty::CryptoDatabase;
use crate::payment::ty::PaymentDatabase;
use crate::user::ty::UserDatabase;

impl DB {
    pub fn user_db(&self) -> UserDatabase {
        UserDatabase { db: &self.db }
    }

    pub fn payment_db(&self) -> PaymentDatabase {
        PaymentDatabase { db: &self.db }
    }

    pub fn crypto_db(&self) -> CryptoDatabase {
        CryptoDatabase { db: &self.db }
    }
}
