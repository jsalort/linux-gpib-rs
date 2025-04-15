#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
linux-gpib-rs = "0.1"
---
use linux_gpib_rs::{
    OpenParam,
    open,
    ibwrt, 
    ibrd,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ud = open("GPIB0::1::INSTR", OpenParam::default())?;
    ibwrt(ud, b"*IDN?\r\n")?;
    let mut buffer: [u8; 256] = [0; 256];
    ibrd(ud, &mut buffer)?;
    let iden = String::from_utf8(buffer.to_vec())?;
    println!("{iden}");
    Ok(())
}