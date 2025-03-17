use pinocchio::{
    entrypoint::{InstructionContext, MaybeAccount},
    lazy_program_entrypoint,
    program_error::ProgramError,
    ProgramResult,
};
#[cfg(feature = "xl")]
use pinocchio_log::log;

lazy_program_entrypoint!(process_instruction);

#[cfg(not(feature = "xl"))]
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

    #[cfg(not(feature = "xs"))]
    // SAFETY: All accounts have been processed and the syscall validates if
    // the data is valid UTF-8.
    unsafe {
        let instruction_data = context.instruction_data_unchecked();
        pinocchio::syscalls::sol_log_(instruction_data.as_ptr(), instruction_data.len() as u64);
    }

    Ok(())
}

#[cfg(feature = "xl")]
pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    let mut output = [0u8; 44];
    let mut missing_required_signature = false;

    // Process signer accounts (if any).
    while context.remaining() > 0 {
        // Duplicated accounts are implicitly checked since at least one of the
        // "copies" must be a signer.
        if let MaybeAccount::Account(account) = context.next_account()? {
            if account.is_signer() {
                let len = five8::encode_32(account.key(), &mut output);
                let as_str = core::str::from_utf8(unsafe { output.get_unchecked(..len as usize) })
                    .map_err(|_error| ProgramError::InvalidAccountData)?;
                log!("Signed by {}", as_str);
            } else {
                missing_required_signature = true;
            }
        }
    }

    if missing_required_signature {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // SAFETY: All accounts have been processed.
    let instruction_data = unsafe { context.instruction_data_unchecked() };

    log!(
        "Memo (len {}): \"{}\"",
        instruction_data.len(),
        core::str::from_utf8(instruction_data).map_err(|error| {
            log!(1000, "Invalid UTF-8, from byte {}", error.valid_up_to());
            ProgramError::InvalidInstructionData
        })?
    );

    Ok(())
}
