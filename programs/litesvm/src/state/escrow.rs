use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace,Debug)]
pub struct Escrow {
    pub seed:u64,   
    pub maker:Pubkey,    // who make the token exchange
    pub mint_a:Pubkey,   //token mint a 
    pub mint_b:Pubkey,   //token mint b 
    pub recieve:u64,  //who recive the token 
    pub bump:u8
}