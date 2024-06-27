use jito_restaking_sanitization::{
    assert_with_msg, signer::SanitizedSignerAccount, system_program::SanitizedSystemProgram,
};
use jito_vault_core::{
    config::SanitizedConfig, vault::SanitizedVault, vault_avs_list::SanitizedVaultAvsList,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

/// Processes the vault add AVS instruction: [`crate::VaultInstruction::AddAvs`]
pub fn process_vault_add_avs(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let mut accounts_iter = accounts.iter();

    let restaking_program_signer =
        SanitizedSignerAccount::sanitize(next_account_info(&mut accounts_iter)?, false)?;
    let avs = SanitizedSignerAccount::sanitize(next_account_info(&mut accounts_iter)?, false)?;
    let vault =
        SanitizedVault::sanitize(program_id, next_account_info(&mut accounts_iter)?, false)?;
    let config =
        SanitizedConfig::sanitize(program_id, next_account_info(&mut accounts_iter)?, false)?;
    let mut vault_avs_list = SanitizedVaultAvsList::sanitize(
        program_id,
        next_account_info(&mut accounts_iter)?,
        true,
        vault.account().key,
    )?;
    let payer = SanitizedSignerAccount::sanitize(next_account_info(&mut accounts_iter)?, true)?;
    let _system_program = SanitizedSystemProgram::sanitize(next_account_info(&mut accounts_iter)?)?;

    assert_with_msg(
        config.config().restaking_program_signer() == *restaking_program_signer.account().key,
        ProgramError::InvalidAccountData,
        "Restaking program signer does not match config",
    )?;

    let clock = Clock::get()?;

    assert_with_msg(
        vault_avs_list
            .vault_avs_list_mut()
            .add_avs(*avs.account().key, clock.slot),
        ProgramError::InvalidArgument,
        "AVS already added to vault",
    )?;

    msg!(
        "AVS @ {} added to vault @ {} in slot {}",
        avs.account().key,
        vault.account().key,
        clock.slot
    );

    vault_avs_list.save_with_realloc(&Rent::get()?, payer.account())?;

    Ok(())
}
