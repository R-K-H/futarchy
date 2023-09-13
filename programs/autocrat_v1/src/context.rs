// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token::{Burn, Mint, MintTo, Token, TokenAccount, Transfer},
// };
use anchor_spl::token::{Mint, Token};

use super::*;

#[derive(Accounts)]
pub struct InitializeMetaDAO<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + (100 * 32), // 100 member max
        seeds = [b"WWCACOTMICMIBMHAFTTWYGHMB"], // abbreviation of the last two sentences of the Declaration of Independence of Cyberspace
        bump
    )]
    pub meta_dao: Account<'info, MetaDAO>,
    pub seed_member: Account<'info, Member>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeMember<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + 20 + 1 + 32,
        seeds = [b"member", name.as_bytes()], // 256^20 possible names, so practically impossible for all names to be exhausted
        bump
    )]
    pub member: Account<'info, Member>,
    #[account(
        init,
        payer = initializer,
        mint::authority = member,
        mint::freeze_authority = member,
        mint::decimals = 9
    )]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(
        signer @ ErrorCode::UnauthorizedFunctionCall,
        mut,
        seeds = [b"WWCACOTMICMIBMHAFTTWYGHMB"], 
        bump
    )]
    pub meta_dao: Account<'info, MetaDAO>,
}

#[derive(Accounts)]
pub struct InitializeProposal<'info> {
    #[account(zero)]
    pub proposal: Account<'info, Proposal>,
    #[account(seeds = [b"WWCACOTMICMIBMHAFTTWYGHMB"], bump)]
    pub meta_dao: Account<'info, MetaDAO>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct FailProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}