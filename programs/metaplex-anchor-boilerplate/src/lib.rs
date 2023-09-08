pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("4WJfQtgA6YeZzYapY1Y9gEMFR5enk32xd7ALDMjbPncg");

#[program]
pub mod metaplex_anchor_boilerplate {
    use super::*;

    pub fn transfer_nft(ctx: Context<TransferNft>) -> Result<()> {
        handle_transfer_nft(ctx);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
