use {
    core::str::{from_utf8, from_utf8_unchecked},
    pinocchio::{
        entrypoint::{InstructionContext, MaybeAccount},
        lazy_program_entrypoint,
        program_error::ProgramError,
        ProgramResult,
    },
    pinocchio_log::log,
};

lazy_program_entrypoint!(process_instruction);

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    // Process signer accounts (if any).
    while context.remaining() > 0 {
        // Duplicated accounts are implicitly checked since at least one of the
        // "copies" must be a signer.
        if let MaybeAccount::Account(account) = context.next_account()? {
            if !account.is_signer() {
                return Err(ProgramError::MissingRequiredSignature);
            }
        }
    }

    Ok(())
}
