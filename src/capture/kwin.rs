use std::collections::HashMap;
use std::io::Read;
use std::os::fd::AsFd;
use zbus::blocking::Connection;
use zbus::zvariant::{Fd, OwnedValue, Value};
use crate::capture::errors::CaptureError;
use crate::capture::pixels::RawCapture;

pub fn capture_workspace() -> Result<RawCapture, CaptureError> {
    let conn = Connection::session()?;
    let (mut reader, writer) = os_pipe::pipe()?;

    let thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).map(|_| buf)
    });

    let mut options: HashMap<&str, Value> = HashMap::new();
    options.insert("native-resolution", true.into());

    let reply = conn.call_method(
        Some("org.kde.KWin.ScreenShot2"),
        "/org/kde/KWin/ScreenShot2",
        Some("org.kde.KWin.ScreenShot2"),
        "CaptureWorkspace",
        &(options, Fd::from(writer.as_fd())),
    )?;

    drop(writer);

    let meta: HashMap<String, OwnedValue> = reply.body().deserialize()?;
    let data = thread.join().unwrap()?;

    Ok(RawCapture {
        data,
        width: get_u32(&meta, "width")?,
        height: get_u32(&meta, "height")?,
        stride: get_u32(&meta, "stride")?,
        format: get_u32(&meta, "format")?,
        scale: get_f64(&meta, "scale")?,
    })
}

fn get_u32(meta: &HashMap<String, OwnedValue>, key: &str) -> Result<u32, CaptureError> {
    let v = meta
        .get(key)
        .ok_or_else(|| CaptureError::BadMetadata(format!("missing field `{key}`")))?;
    u32::try_from(v).map_err(|_| CaptureError::BadMetadata(format!("field `{key}` is not a u32")))
}

fn get_f64(meta: &HashMap<String, OwnedValue>, key: &str) -> Result<f64, CaptureError> {
    let v = meta
        .get(key)
        .ok_or_else(|| CaptureError::BadMetadata(format!("missing field `{key}`")))?;
    f64::try_from(v).map_err(|_| CaptureError::BadMetadata(format!("field `{key}` is not a f64")))
}
