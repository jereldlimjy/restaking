use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};
use spl_token::state::Account;

use crate::assert_with_msg;

pub struct SanitizedTokenAccount<'a, 'info> {
    inner: &'a AccountInfo<'info>,
    token_account: Account,
}

impl<'a, 'info> SanitizedTokenAccount<'a, 'info> {
    pub fn sanitize(
        account: &'a AccountInfo<'info>,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> Result<SanitizedTokenAccount<'a, 'info>, ProgramError> {
        assert_with_msg(
            account.data_len() == Account::LEN,
            ProgramError::InvalidAccountData,
            &format!(
                "Invalid token account data length: {} expected: {}",
                account.data_len(),
                Account::LEN
            ),
        )?;

        assert_with_msg(
            *account.owner == spl_token::id(),
            ProgramError::IllegalOwner,
            &format!(
                "Invalid token account owner: {:?} pubkey: {:?}",
                account.owner, account.key
            ),
        )?;

        let token_account = Account::unpack(&account.data.borrow())?;

        assert_with_msg(
            token_account.mint == *mint,
            ProgramError::InvalidAccountData,
            "Invalid token account mint",
        )?;
        assert_with_msg(
            token_account.owner == *owner,
            ProgramError::InvalidAccountData,
            "Invalid token account owner",
        )?;

        Ok(SanitizedTokenAccount {
            inner: account,
            token_account,
        })
    }

    pub const fn token_account(&self) -> &Account {
        &self.token_account
    }

    pub const fn account(&self) -> &AccountInfo<'info> {
        self.inner
    }

    /// Reload needs to be called after CPIs to ensure the data is up-to-date
    pub fn reload(&mut self) -> Result<(), ProgramError> {
        self.token_account = Account::unpack(&self.inner.data.borrow())?;
        Ok(())
    }
}
