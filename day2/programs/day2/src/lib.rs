use anchor_lang::prelude::*;

declare_id!("EiWnSyzoKH7VaqRouMVpxUm8BMMwZxq55eFmM1HqsYFX");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn subtract(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result: u64 = a - b;
        msg!("Result of subtraction: {}", result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
