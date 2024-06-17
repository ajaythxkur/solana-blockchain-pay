use anchor_lang::prelude::*;

declare_id!("9AyteXMSrFPjeNabzaXv2YHyXzSoCaz6K6WNWYuMsyxq");

#[program]
pub mod solana_blockchain_pay {
    use super::*;

    pub fn init_with_signer(ctx: Context<InitWithSigner>) -> Result<()> {
        ctx.accounts.init_account.payer = ctx.accounts.signer.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWithSigner<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"signer", signer.key().as_ref()],
        bump,
        space = 8 + 32 + 8
    )]
    pub init_account: Account<'info, InitAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct InitAccount{
    payer: Pubkey,
    worker_count: u64
}

#[derive(Accounts)]
pub struct AddWorker<'info> {
    #[account(
        mut,
        seeds = [b"signer", init_account.payer.key().as_ref()],
        bump,
        has_one = payer
    )]
    pub init_account: Account<'info, InitAccount>,
    #[account(
        init,
        payer = payer,
        seeds = [b"worker", init_account.key().as_ref(), &(init_account.worker_count + 1).to_le_bytes()],
        bump,
        space = 8 + 32
    )]
    pub worker_account: Account<'info, Worker>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Worker{
    address: Pubkey,
    pay_token: Pubkey,
    pay: u64,
    name: String,
}