use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod catch_a_feel {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_feelers = 0;
        Ok(())
    }

    pub fn add_feel(ctx: Context<AddFeel>, somethings: Vec<Something>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // for thing in somethings {
        //     things.src = Source::Address(user, "acc".to_string(), "".to_string();
        // }

        let feel = Feel { user, somethings };

        // Create user's account. (mapping?)
        // if sender has a feel on its account, do not increment.
        base_account.total_feelers += 1;
        base_account.feels.push(feel);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BaseAccount {
    pub total_feelers: u64,
    pub feels: Vec<Feel>,
}

#[derive(Accounts)]
pub struct AddFeel<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Something {
    src: Source,
    transform: Transform,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum Source {
    Uri(String),
    Address(Pubkey, String, String), // address, account, extra
    Cid(String),
    // Here(&'a [u8]),
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Transform {
    position: (f64, f64, f64),
    rotation: (f64, f64, f64),
    scale: (f64, f64, f64),
    // is it possible to have a scale... related to some unit value.
    // external tool: minimal size for something to fit within a space:
    // say, minimal factor for the distance between a pair of vertices
    // to be greater than 1 (not sure on the size)
}

impl Transform {
    pub fn new() -> Transform {
        // Determine scale independently...
        let independent_scale = (1.0, 1.0, 1.0);
        Transform {
            position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: independent_scale,
        }
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Feel {
    user: Pubkey,
    somethings: Vec<Something>,
    // gif, // Can be any 'animated' thing
}
