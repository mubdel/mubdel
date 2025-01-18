use anyhow::Result;

use crate::deposit::Depositor;

pub struct Solana {}

impl Depositor for Solana {
    async fn start(&self) -> Result<()> {
        // get latest block

        // fetch 10k blocks
        Ok(())
    }
}
