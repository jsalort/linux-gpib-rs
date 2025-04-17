//!
//! Low-level wrapper for Linux GPIB.
//!
//! Documentation for the functions comes from [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference.html).
//! At the moment, only the 'Traditional' API Functions are wrapped.
//!
//! ## Requirements
//!
//! This crate needs to link to an installed linux-gpib user library. It will look for `gpib/ib.h` in either `/usr/include` or `/usr/local/include`,
//! and for `libgpib.so` in either `/usr/lib` or `/usr/local/lib`.
//!
//!
//! ## Example
//!
//! Add dependencies below to `Cargo.toml`
//!
//! ```toml
//! linux-gpib-rs = { version = "0.1", features = ["async-tokio"] }
//! ```
//!
//! Codes below will connect to the instrument on `GPIB0::1::INSTR` and print out its `*IDN?` response.
//!
//! **Synchronous example with Multidevice API**
//!
//! ```rust
//! use linux_gpib_rs::instrument::Board;
//! use linux_gpib_rs::types::IbSendEOI;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let board = Board::with_board_number(0);
//!     let instruments = board.find_listeners()?;
//!     board.send_list(&instruments, b"*IDN?\n", IbSendEOI::default())?;
//!     for instr in instruments {
//!         let iden = instr.receive()?;
//!         println!("{:>20} {}", instr.visa_string(), iden.trim());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! **Asynchronous example**
//!
//! Same thing with asynchronous API.
//!
//! ```rust
//! use linux_gpib_rs::instrument::{Parameters, Board};
//! use linux_gpib_rs::error::GpibError;
//! use tokio::task::JoinSet;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let board = Board::with_board_number(0);
//!     let instruments = board.find_listeners()?;
//!     let mut set = JoinSet::<Result<(String, String), GpibError>>::new();
//!     for instr in instruments {
//!         let handle = instr.open(Parameters::default())?;
//!         let visa_string = instr.visa_string();
//!         set.spawn(async move {
//!             let iden = handle.query("*IDN?\n").await?;
//!             Ok((visa_string, iden))
//!         });
//!     }
//!     while let Some(Ok(val)) = set.join_next().await {
//!         if let Ok((visa_string, iden)) = val {
//!             println!("{:>20} {}", visa_string, iden.trim());
//!         }
//!     }
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod instrument;
pub mod lowlevel;
pub mod status;
pub mod types;

const DEBUG: bool = false;
