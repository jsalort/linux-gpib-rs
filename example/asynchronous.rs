#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
linux-gpib-rs = "0.1"
tokio = { version = "1", features = ["full"] }
---
use linux_gpib_rs::{open, write, read, OpenParam};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ud = open("GPIB0::1::INSTR", OpenParam::default())?;
    write(ud, "*IDN?\r\n").await?;
    let iden = read(ud).await?;
    println!("{iden}");
    Ok(())
}