use crate::error::{GpibError, IbError};
use crate::status::IbStatus;
use crate::traditional::{ibrda, ibwrta};
use std::os::raw::c_int;

pub async fn wait(ud: c_int, status_mask: IbStatus) -> Result<IbStatus, GpibError> {
    let status_mask = status_mask.as_ibsta();
    let status = IbStatus::from_ibsta(
        tokio::task::spawn_blocking(move || unsafe { linux_gpib_sys::ibwait(ud, status_mask) })
            .await?,
    );
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(status)
    }
}

pub async fn read(ud: c_int) -> Result<String, GpibError> {
    const BUFFER_SIZE: usize = 1024;
    let mut result: Vec<u8> = Vec::new();
    loop {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let status = IbStatus::from_ibsta(unsafe {
            linux_gpib_sys::ibrda(
                ud,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len().try_into()?,
            )
        });
        if status.err {
            return Err(GpibError::DriverError(status, IbError::current_error()?));
        }
        let status = wait(
            ud,
            IbStatus::default()
                .with_timo(true)
                .with_cmpl(true)
                .with_end(true),
        )
        .await?;
        if status.timo {
            return Err(GpibError::Timeout);
        }
        let n_read: usize = unsafe { linux_gpib_sys::ibcntl }.try_into()?;
        if n_read > 0 {
            result.extend(buffer[0..n_read].to_vec());
        }
        if status.end || n_read < BUFFER_SIZE || n_read == 0 {
            break;
        }
    }
    let answer = String::from_utf8(result)?;
    Ok(answer)
}

pub async fn write(ud: c_int, data: &str) -> Result<(), GpibError> {
    let data = data.as_bytes();
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibwrta(ud, data.as_ptr() as *const c_void, data.len().try_into()?)
    });
    if status.err {
        return Err(GpibError::DriverError(status, IbError::current_error()?));
    }
    let status = wait(
        ud,
        IbStatus::default()
            .with_timo(true)
            .with_cmpl(true)
            .with_end(true)
            .with_rqs(true),
    )
    .await?;
    if status.timo {
        Err(GpibError::Timeout)
    } else if status.cmpl || status.end {
        Ok(())
    } else {
        Err(GpibError::ValueError(format!(
            "Unexpected status after waiting: {:?}",
            status
        )))
    }
}
