use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;
use crate::state::UserAccount;
#[vrf]
#[derive(Accounts)]
    pub struct ReqRandom<'info>{
        #[account(mut)]
        pub payer:Signer <'info>,
        #[account(
           seeds:[b"user",payer.key().as_ref()]
           bump
        )]
         pub user_account: Account<'info,UserAccount>,
         #[account(mut)]
         pub oracle_queue:AccountInfo<'info>,
    }
impl<'info> ReqRandom<'info>{
    pub fn request_randomness(&self,client_seed:u8)->Result<()>{
         msg!("Requesting randomness...");
        let ix=create_request_randomness_ix(RequestRandomnessParams {
             payer: self.payer.key(),
              oracle_queue: self.oracle_queue.key(), 
              callback_program_id: crate::ID, 
              callback_discriminator: crate::instructions::callback::DISCRIMINATOR.to_vec(),
              caller_seed: [client_seed;32],
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: self.user_account.key(),
                is_signer: false,
                is_writable: true, // mut in callback
            }]),
            ..Default::default()
        });
        self.invoke_signed_vrf(&self.payer.to_account_info(), &ix)?;
             Ok(())
    }
}