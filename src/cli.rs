use linux_gpib_rs::instrument::{Board, Parameters};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let board = Board::with_board_number(0);
    let instruments = board.find_listeners()?;
    for instr in instruments {
        let h = instr.open(Parameters::default())?;
        let iden = h.blocking_query("*IDN?\n")?;
        println!("{:>20} {}", instr.visa_string(), iden.trim());
    }
    Ok(())
}
