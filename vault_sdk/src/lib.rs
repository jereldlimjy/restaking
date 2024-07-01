use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[rustfmt::skip]
#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum VaultInstruction {
    /// Initializes global configuration
    #[account(0, writable, name = "config")]
    #[account(1, writable, signer, name = "admin")]
    #[account(2, name = "restaking_program_signer")]
    #[account(3, name = "restaking_program")]
    #[account(4, name = "system_program")]
    InitializeConfig,

    /// Initializes the vault
    #[account(0, writable, name = "config")]
    #[account(1, writable, name = "vault")]
    #[account(2, writable, name = "vault_avs_list")]
    #[account(3, writable, name = "vault_operator_list")]
    #[account(4, writable, name = "vault_slasher_list")]
    #[account(5, writable, signer, name = "lrt_mint")]
    #[account(6, name = "token_mint")]
    #[account(7, writable, signer, name = "admin")]
    #[account(8, signer, name = "base")]
    #[account(9, name = "system_program")]
    #[account(10, name = "token_program")]
    InitializeVault {
        deposit_fee_bps: u16,
        withdrawal_fee_bps: u16,
    },

    /// Initializes a vault with an already-created LRT mint
    InitializeVaultWithMint,

    /// AVS can CPI into this program calling this instruction to add themselves to receive
    /// delegation from a vault
    #[account(0, signer, name = "restaking_program_signer")]
    #[account(1, signer, name = "avs")]
    #[account(2, name = "vault")]
    #[account(3, name = "config")]
    #[account(4, writable, name = "vault_avs_list")]
    #[account(5, writable, signer, name = "payer")]
    #[account(6, name = "system_program")]
    AddAvs,

    /// AVS can CPI into this program calling this instruction to add themselves to remove
    /// delegation support for a vault
    #[account(0, signer, name = "restaking_program_signer")]
    #[account(1, signer, name = "avs")]
    #[account(2, name = "vault")]
    #[account(3, name = "config")]
    #[account(4, writable, name = "vault_avs_list")]
    RemoveAvs,

    /// Node operator's can CPI into this program calling this instruction to add themselves to receive
    /// delegation from a vault
    #[account(0, signer, name = "restaking_program_signer")]
    #[account(1, signer, name = "operator")]
    #[account(2, name = "vault")]
    #[account(3, name = "config")]
    #[account(4, writable, name = "vault_operator_list")]
    #[account(5, writable, signer, name = "payer")]
    #[account(6, name = "system_program")]
    AddOperator,

    /// Node operator's can CPI into this program calling this instruction to add themselves to remove
    /// delegation from a vault
    #[account(0, signer, name = "restaking_program_signer")]
    #[account(1, signer, name = "operator")]
    #[account(2, name = "vault")]
    #[account(3, name = "config")]
    #[account(4, writable, name = "vault_operator_list")]
    #[account(5, writable, signer, name = "payer")]
    #[account(6, name = "system_program")]
    RemoveOperator,

    /// Mints LRT by depositing tokens into the vault
    #[account(0, writable, name = "vault")]
    #[account(1, writable, name = "lrt_mint")]
    #[account(2, writable, signer, name = "source_owner")]
    #[account(3, writable, name = "source_token_account")]
    #[account(4, writable, name = "dest_token_account")]
    #[account(5, writable, name = "lrt_receiver")]
    #[account(6, name = "token_program")]
    #[account(7, signer, optional, name = "mint_signer", description = "Signer for minting")]
    MintTo {
        amount: u64
    },

    /// Burns LRT by withdrawing tokens from the vault
    Burn {
        amount: u64
    },

    /// Enqueues a withdrawal of LRT tokens
    /// Used when there aren't enough idle assets in the vault to cover a withdrawal
    EnqueueWithdrawal {
        amount: u64
    },

    /// Sets the max tokens that can be deposited into the LRT
    #[account(0, writable, name = "vault")]
    #[account(1, signer, name = "admin")]
    SetDepositCapacity {
        amount: u64
    },

    /// Withdraws any non-backing tokens from the vault
    WithdrawalAsset {
        amount: u64
    },

    /// Changes the signer for vault delegation
    #[account(0, writable, name = "vault")]
    #[account(1, signer, name = "admin", description = "Admin or delegation admin of the LRT")]
    #[account(2, signer, name = "new_admin", description = "Admin or delegation admin of the LRT")]
    SetDelegationAdmin,

    /// Changes the signer for vault admin
    #[account(0, writable, name = "vault")]
    #[account(1, signer, name = "old_admin")]
    #[account(2, signer, name = "old_admin")]
    SetAdmin,

    /// Delegates a token amount to a specific node operator
    #[account(0, name = "config")]
    #[account(1, name = "vault")]
    #[account(2, writable, name = "vault_operator_list")]
    #[account(3, name = "operator")]
    #[account(4, signer, name = "delegation_admin")]
    #[account(5, writable, signer, name = "payer")]
    AddDelegation {
        amount: u64,
    },

    /// Delegates a token amount to a specific node operator
    #[account(0, name = "config")]
    #[account(1, name = "vault")]
    #[account(2, writable, name = "vault_operator_list")]
    #[account(3, name = "operator")]
    #[account(4, signer, name = "admin")]
    #[account(5, writable, signer, name = "payer")]
    RemoveDelegation {
        amount: u64,
    },

    /// Updates delegations at epoch boundaries
    #[account(0, name = "config")]
    #[account(1, name = "vault")]
    #[account(2, writable, name = "vault_operator_list")]
    #[account(3, writable, signer, name = "payer")]
    UpdateDelegations,

    /// Registers a slasher with the vault
    #[account(0, name = "config")]
    #[account(1, name = "vault")]
    #[account(2, writable, name = "vault_slasher_list")]
    #[account(3, name = "slasher")]
    #[account(4, signer, name = "admin")]
    #[account(5, signer, writable, name = "payer")]
    #[account(6, name = "avs")]
    #[account(7, name = "avs_slasher_list")]
    AddSlasher,

    /// Slashes an amount of tokens from the vault
    #[account(0, name = "config")]
    #[account(1, writable, name = "vault")]
    #[account(2, name = "vault_slasher_list")]
    #[account(3, writable, name = "vault_operator_list")]
    #[account(4, writable, name = "vault_token_account")]
    #[account(5, name = "avs")]
    #[account(6, name = "avs_operator_list")]
    #[account(7, name = "operator")]
    #[account(8, signer, name = "slasher")]
    #[account(9, name = "slasher_token_account")]
    #[account(10, name = "token_program")]
    Slash {
        amount: u64
    },

    /// Creates token metadata for the vault LRT
    CreateTokenMetadata {
        name: String,
        symbol: String,
        uri: String,
    },

    /// Updates token metadata for the vault LRT
    UpdateTokenMetadata {
        name: String,
        symbol: String,
        uri: String,
    },
}

pub fn initialize_config(
    program_id: &Pubkey,
    config: &Pubkey,
    admin: &Pubkey,
    restaking_program_signer: &Pubkey,
    restaking_program: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*admin, true),
            AccountMeta::new(*restaking_program_signer, false),
            AccountMeta::new_readonly(*restaking_program, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: VaultInstruction::InitializeConfig.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn initialize_vault(
    program_id: &Pubkey,
    config: &Pubkey,
    vault: &Pubkey,
    vault_avs_list: &Pubkey,
    vault_operator_list: &Pubkey,
    vault_slasher_list: &Pubkey,
    lrt_mint: &Pubkey,
    token_mint: &Pubkey,
    admin: &Pubkey,
    base: &Pubkey,
    deposit_fee_bps: u16,
    withdrawal_fee_bps: u16,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*vault, false),
            AccountMeta::new(*vault_avs_list, false),
            AccountMeta::new(*vault_operator_list, false),
            AccountMeta::new(*vault_slasher_list, false),
            AccountMeta::new(*lrt_mint, true),
            AccountMeta::new(*token_mint, false),
            AccountMeta::new(*admin, true),
            AccountMeta::new(*base, true),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: VaultInstruction::InitializeVault {
            deposit_fee_bps,
            withdrawal_fee_bps,
        }
        .try_to_vec()
        .unwrap(),
    }
}

pub fn add_avs(
    program_id: &Pubkey,
    restaking_program_signer: &Pubkey,
    avs: &Pubkey,
    vault: &Pubkey,
    config: &Pubkey,
    vault_avs_list: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*restaking_program_signer, true),
            AccountMeta::new_readonly(*avs, true),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new(*vault_avs_list, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: VaultInstruction::AddAvs.try_to_vec().unwrap(),
    }
}

pub fn remove_avs(
    program_id: &Pubkey,
    restaking_program_signer: &Pubkey,
    avs: &Pubkey,
    vault: &Pubkey,
    config: &Pubkey,
    vault_avs_list: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*restaking_program_signer, true),
            AccountMeta::new_readonly(*avs, true),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new(*vault_avs_list, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: VaultInstruction::RemoveAvs.try_to_vec().unwrap(),
    }
}

pub fn add_operator(
    program_id: &Pubkey,
    restaking_program_signer: &Pubkey,
    operator: &Pubkey,
    vault: &Pubkey,
    config: &Pubkey,
    vault_operator_list: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*restaking_program_signer, true),
            AccountMeta::new_readonly(*operator, true),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new(*vault_operator_list, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: VaultInstruction::AddOperator.try_to_vec().unwrap(),
    }
}

pub fn remove_operator(
    program_id: &Pubkey,
    restaking_program_signer: &Pubkey,
    operator: &Pubkey,
    vault: &Pubkey,
    config: &Pubkey,
    vault_operator_list: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*restaking_program_signer, true),
            AccountMeta::new_readonly(*operator, true),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new(*vault_operator_list, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: VaultInstruction::RemoveOperator.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn mint_to(
    program_id: &Pubkey,
    vault: &Pubkey,
    lrt_mint: &Pubkey,
    source_owner: &Pubkey,
    source_token_account: &Pubkey,
    dest_token_account: &Pubkey,
    lrt_receiver: &Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*vault, false),
            AccountMeta::new(*lrt_mint, false),
            AccountMeta::new(*source_owner, true),
            AccountMeta::new(*source_token_account, false),
            AccountMeta::new(*dest_token_account, false),
            AccountMeta::new(*lrt_receiver, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: VaultInstruction::MintTo { amount }.try_to_vec().unwrap(),
    }
}

pub fn set_capacity(
    program_id: &Pubkey,
    vault: &Pubkey,
    admin: &Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*vault, false),
            AccountMeta::new(*admin, true),
        ],
        data: VaultInstruction::SetDepositCapacity { amount }
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn add_delegation(
    program_id: &Pubkey,
    config: &Pubkey,
    vault: &Pubkey,
    vault_operator_list: &Pubkey,
    operator: &Pubkey,
    delegation_admin: &Pubkey,
    payer: &Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new(*vault_operator_list, false),
            AccountMeta::new_readonly(*operator, false),
            AccountMeta::new_readonly(*delegation_admin, true),
            AccountMeta::new(*payer, true),
        ],
        data: VaultInstruction::AddDelegation { amount }
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn remove_delegation(
    program_id: &Pubkey,
    config: &Pubkey,
    vault: &Pubkey,
    vault_operator_list: &Pubkey,
    operator: &Pubkey,
    admin: &Pubkey,
    payer: &Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new(*vault_operator_list, false),
            AccountMeta::new_readonly(*operator, false),
            AccountMeta::new_readonly(*admin, true),
            AccountMeta::new(*payer, true),
        ],
        data: VaultInstruction::RemoveDelegation { amount }
            .try_to_vec()
            .unwrap(),
    }
}
