use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use super::*;
 
    pub fn create(ctx:Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
    
    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }
    
    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }
    
    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }
    
    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }

}

// we used the derive Accounts macro since we had to incorporate 3 accounts here and for all 
// these 3 accounts individually, we used the account macro.
// More on accounts -> https://docs.rs/anchor-lang/0.18.0/anchor_lang/derive.Accounts.html
#[derive(Accounts)]
pub struct Create<'info> {
    // The init macro is used to create a new account owned by the current program which is our
    //  mycalculatordapp program. Whenever init parameter is used, we must always specify the
    //  payer or the account that will be paying for creation of the account on the Solana blockchain,
    //  along with the space param which is the space with which the new account is created
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub calculator: Account<'info, Calculator>,
    // The mut parameter marks an account as mutable, which essentially means our account will 
    // be altered and Solana will need to update the data in your account. So, always use the mut 
    // parameter for persisting changes.
    #[account(mut)]
    // Signer type is used to enforce the constraint that the authority account 
    // (messengerapp in this case) signed the transaction
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}


