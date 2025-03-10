use pinocchio::{
    entrypoint::{InstructionContext, MaybeAccount},
    lazy_program_entrypoint,
    program_error::ProgramError,
    syscalls::sol_log_,
    ProgramResult,
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
    // SAFETY: All accounts have been processed and the syscall validates if
    // the data is valid UTF-8.
    unsafe {
        let instruction_data = context.instruction_data_unchecked();
        sol_log_(instruction_data.as_ptr(), instruction_data.len() as u64);
    }

    Ok(())
}
