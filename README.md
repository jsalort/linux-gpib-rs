# linux-gpib-rs

Low-level wrapper for Linux GPIB.

**Synchronous example**

We can use the low-level synchronous functions `ibrd` and `ibwrt`.

```rust
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
```

**Asynchronous example**

We can use slightly higher-level asynchronous functions `write` and `read` (based on `ibrda` and `ibwrta`).
This requires the `async-tokio` feature.

```rust
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
```