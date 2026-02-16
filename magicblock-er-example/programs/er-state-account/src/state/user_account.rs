use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub user: Pubkey,
    pub data: u64,
    pub bump: u8,
}

impl UserAccount {
    // Anchor prepends an 8-byte account discriminator. UserAccount contains:
    // Pubkey (32) + u64 (8) + u8 (1) = 41 bytes. Total space = discriminator (8) + 41 = 49.
    // We also keep a small safety buffer if needed.
    pub const INIT_SPACE: usize = 8 + 32 + 8 + 1;
}

