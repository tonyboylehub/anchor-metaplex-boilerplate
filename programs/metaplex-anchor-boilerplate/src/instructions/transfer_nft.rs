


use mpl_token_metadata::{accounts::Metadata, instructions::TransferV1CpiBuilder};
use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct TransferNft<'info> {

        #[account(mut)]
        pub user: Signer<'info>,
        /// CHECK: Checking in program
        pub update_authority: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub destination_owner: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub destination_token_account: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub return_to_destination: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub mint: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub metadata: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub token_account: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub token_metadata_program: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub authorization_rules_program: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub authorization_rules: AccountInfo<'info>,
        /// CHECK: Checking in program
		pub spl_token_program: AccountInfo<'info>,
        /// CHECK: Checking in program
		pub spl_ata_program: AccountInfo<'info>,
        /// CHECK: Checking in program
        pub system_program: Program<'info, System>,
        /// CHECK: Checking in program
        pub sysvar_instructions: AccountInfo<'info,>

}


pub fn handle_transfer_nft<'info>(
        ctx: Context<TransferNft>,
    ) -> Result<()> {

		// decode accounts
		let metadata_account = Metadata::try_from(&ctx.accounts.metadata)?;
 

        TransferV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
            .token(&ctx.accounts.token_account)
            .token_owner(&ctx.accounts.user)
            .destination_token(&ctx.accounts.destination_token_account)
            .destination_owner(&ctx.accounts.destination_owner)
            .mint(&ctx.accounts.mint)
            .metadata(&ctx.accounts.metadata)
            .authority(&ctx.accounts.update_authority)
            .payer(&ctx.accounts.user)
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&ctx.accounts.sysvar_instructions)
            .spl_token_program(&ctx.accounts.spl_token_program)
            .spl_ata_program(&ctx.accounts.spl_ata_program)
            .amount(1)
            .build()
            .invoke();      
        
    
    Ok(())
        
    }