use anchor_lang::prelude::*;

declare_id!("HttxV1yGBtm4gwouBqoG7BhPu2nj4N74KfsE2ZDnMyeo");

#[program]
pub mod puppet {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        ctx.accounts.puppet.authority = authority;
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        let puppet = &mut ctx.accounts.puppet;
        puppet.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + Data::LEN)]
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut, has_one = authority)]
    pub puppet: Account<'info, Data>,
    pub authority: Signer<'info>
}

#[account]
pub struct Data {
    pub data: u64,
    pub authority: Pubkey,
}

impl Data {
    pub const LEN: usize = 8 + 32;
}
