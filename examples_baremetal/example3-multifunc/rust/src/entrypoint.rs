use crate::processor::Processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Declare and export the program's entrypoint
// TODO set the correct entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("[entrypoint] multifunc example entrypoint");
    // use processor function
    return Processor::process_program_call(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    )
    // TODO call the correct function in Processor, passing the arguments from this function
   //  return Processor::
}
