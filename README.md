# linux-gpib-rs

Low-level wrapper for Linux GPIB.

Documentation for the functions comes from [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference.html).
At the moment, only the 'Traditional' API Functions are wrapped.

## Requirements

This crate needs to link to an installed linux-gpib user library. It will look for `gpib/ib.h` in either `/usr/include` or `/usr/local/include`,
and for `libgpib.so` in either `/usr/lib` or `/usr/local/lib`.


## Example

Add dependencies below to `Cargo.toml`

```toml
linux-gpib-rs = { version = "0.1", features = ["async-tokio"] }
```

Codes below will connect to the instrument on `GPIB0::1::INSTR` and print out its `*IDN?` response.

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