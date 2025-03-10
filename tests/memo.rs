use mollusk_svm::{result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{error::InstructionError, AccountMeta, Instruction};
use solana_pubkey::Pubkey;

/// Program ID for the p-memo program.
const PROGRAM_ID: Pubkey = Pubkey::from_str_const("PMemo11111111111111111111111111111111111111");

/// The memo to be printed.
const MEMO: &str = "why does spl memo use 36000 cus to print len 60 msg of ascii";

/// Creates an instruction for the p-memo program.
fn instruction(message: &[u8], signers: Option<&[Pubkey]>) -> Instruction {
    let accounts = if let Some(signers) = signers {
        let mut accounts = Vec::with_capacity(signers.len());
        for signer in signers {
            accounts.push(AccountMeta::new_readonly(*signer, true));
        }
        accounts
    } else {
        Vec::new()
    };

    Instruction {
        program_id: PROGRAM_ID,
        accounts,
        data: message.to_vec(),
    }
}

#[test]
fn test_valid_ascii_no_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let instruction = instruction(MEMO.as_bytes(), None);

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}

#[test]
fn fail_test_invalid_ascii_no_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let instruction = instruction(&[255, 255], None);

    mollusk.process_and_validate_instruction(
        &instruction,
        &[],
        &[Check::instruction_err(
            InstructionError::ProgramFailedToComplete,
        )],
    );
}

#[test]
fn test_valid_ascii_one_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let signer = Pubkey::new_unique();
    let instruction = instruction(MEMO.as_bytes(), Some(&[signer]));

    mollusk.process_instruction(&instruction, &[(signer, Account::default())]);
}

#[test]
fn test_valid_ascii_two_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let signers = [Pubkey::new_unique(), Pubkey::new_unique()];
    let instruction = instruction(MEMO.as_bytes(), Some(&signers));

    mollusk.process_instruction(
        &instruction,
        &signers
            .iter()
            .map(|signer| (*signer, Account::default()))
            .collect::<Vec<(Pubkey, Account)>>(),
    );
}

#[test]
fn test_valid_ascii_duplicated_accounts() {
    let mollusk = Mollusk::new(&PROGRAM_ID, "p_memo");

    let unique = Pubkey::new_unique();
    let duplicated = Pubkey::new_unique();
    let instruction = instruction(MEMO.as_bytes(), Some(&[duplicated, unique, duplicated]));

    mollusk.process_instruction(
        &instruction,
        &[
            (duplicated, Account::default()),
            (unique, Account::default()),
        ],
    );
}
