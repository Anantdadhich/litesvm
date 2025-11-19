use anchor_lang::prelude::*;
use state::*;
use instructions::*;
pub mod state;
pub mod instructions;

declare_id!("E6zstiTDc2K1GN3yrUJET2xrhQDKSFMmT8EcuvuWdiWg");

#[program]
pub mod litesvm {
    use super::*;
    
    pub fn make(ctx:Context<Make>,seed:u64,deposit:u64,recieve:u64)->Result<()> {
       ctx.accounts.init_escrow(seed,recieve,&ctx.bumps)?;
       ctx.accounts.deposit(deposit)
       }

    pub fn refund(ctx:Context<Refund>)->Result<()>{
     ctx.accounts.refund_and_close_vault()
       
    }
    pub fn take(ctx:Context<Take>)->Result<()>{
      ctx.accounts.deposit()?;
      ctx.accounts.withdraw_and_close_vault()
    }

  
}

