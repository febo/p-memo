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
    let mut output = [0u8; 44];
    let mut missing_required_signature = false;

    // Process signer accounts (if any).
    while context.remaining() > 0 {
        // Duplicated accounts are implicitly checked since at least one of the
        // "copies" must be a signer.
        if let MaybeAccount::Account(account) = context.next_account()? {
            if account.is_signer() {
                let len = five8::encode_32(account.key(), &mut output);
                // SAFETY: b58 encoding produces a valid UTF-8 string.
                let as_str = unsafe { from_utf8_unchecked(output.get_unchecked(..len as usize)) };
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
        1300,
        "Memo (len {}): \"{}\"",
        instruction_data.len(),
        from_utf8(instruction_data).map_err(|error| {
            log!(1300, "Invalid UTF-8, from byte {}", error.valid_up_to());
            ProgramError::InvalidInstructionData
        })?
    );

    Ok(())
}
