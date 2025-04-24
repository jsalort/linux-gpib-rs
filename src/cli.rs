use linux_gpib_rs::error::GpibError;
use linux_gpib_rs::instrument::{Board, Parameters};
use linux_gpib_rs::lowlevel::traditional::{ibdev, ibrd, ibwrt};
use linux_gpib_rs::types::IbSendEOI;
use linux_gpib_rs::types::{IbEosMode, IbTimeout, PrimaryAddress, SecondaryAddress};
use tokio::task::JoinSet;

fn lowlevel_query() -> Result<(), GpibError> {
    let ud = ibdev(
        0,
        PrimaryAddress::new(1)?,
        SecondaryAddress::new(0)?,
        IbTimeout::T1s,
        IbSendEOI::default(),
        IbEosMode::default(),
    )?;
    ibwrt(ud, b"*IDN?\n")?;
    let mut buffer: [u8; 256] = [0; 256];
    ibrd(ud, &mut buffer)?;
    let iden = String::from_utf8(buffer.to_vec())?;
    println!("{iden}");
    Ok(())
}

fn multidevice_query() -> Result<(), GpibError> {
    let board = Board::with_board_number(0);
    let instruments = board.find_listeners()?;
    board.send_list(&instruments, b"*IDN?\n", IbSendEOI::default())?;
    for instr in instruments {
        let iden = instr.receive()?;
        println!("{:>20} {}", instr.visa_string(), iden.trim());
    }
    Ok(())
}

async fn asynchronous_simple() -> Result<(), GpibError> {
    let board = Board::with_board_number(0);
    let instruments = board.find_listeners()?;
    for instr in instruments {
        let handle = instr.open(Parameters::default())?;
        let visa_string = instr.visa_string();
        let iden = handle.query("*IDN?\n").await?;
        println!("{:>20} {}", visa_string, iden.trim());
    }
    Ok(())
}

async fn asynchronous_bis() -> Result<(), GpibError> {
    let board = Board::with_board_number(0);
    let instruments = board.find_listeners()?;
    let mut set = JoinSet::<Result<(String, String), GpibError>>::new();
    for instr in instruments {
        let handle = instr.open(Parameters::default())?;
        let visa_string = instr.visa_string();
        set.spawn(async move {
            let iden = handle.query("*IDN?\n").await?;
            Ok((visa_string, iden))
        });
    }
    while let Some(Ok(val)) = set.join_next().await {
        if let Ok((visa_string, iden)) = val {
            println!("{:>20} {}", visa_string, iden.trim());
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    println!("*** lowlevel_query ***");
    lowlevel_query().unwrap();

    println!("*** Multidevice query ***");
    multidevice_query().unwrap();

    println!("*** Asynchronous simple ***");
    asynchronous_simple().await.unwrap();

    println!("**** Asynchronous bis ***");
    asynchronous_bis().await.unwrap();
}
