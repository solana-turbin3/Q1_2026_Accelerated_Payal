use anchor_lang::prelude::*;

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Whitelist::INIT_SPACE,
        // Use a distinct root seed so per-address whitelist PDAs can use the
        // `b"whitelist"` seed without colliding with this root account.
        seeds = [b"whitelist-root", admin.key().as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWhitelist<'info> {
    pub fn initialize_whitelist(&mut self, bumps: InitializeWhitelistBumps) -> Result<()> {
        // Initialize the whitelist with an empty address vector
        self.whitelist.set_inner(Whitelist { 
            address: self.admin.key(), // Set the admin's address as the initial entry in the whitelist
            bump: bumps.whitelist,
        });

        Ok(())
    }
}