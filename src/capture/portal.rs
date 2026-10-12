use std::collections::HashMap;
use zbus::blocking::{Connection, Proxy};
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

use super::CaptureError;

pub fn capture_file() -> Result<std::path::PathBuf, CaptureError> {
    let conn = Connection::session()?;
    let token = "screencap";
    let sender = conn.unique_name().unwrap().as_str().trim_start_matches(':').replace('.', "_");
    let request_path = format!("/org/freedesktop/portal/desktop/request/{sender}/{token}");

    let request = Proxy::new(&conn, "org.freedesktop.portal.Desktop",
                             request_path, "org.freedesktop.portal.Request")?;
    let mut responses = request.receive_signal("Response")?;

    let portal = Proxy::new(&conn, "org.freedesktop.portal.Desktop",
                            "/org/freedesktop/portal/desktop",
                            "org.freedesktop.portal.Screenshot")?;
    let mut options: HashMap<&str, Value> = HashMap::new();
    options.insert("handle_token", token.into());
    options.insert("interactive", false.into());
    let _handle: OwnedObjectPath = portal.call("Screenshot", &("", options))?;

    let msg = responses.next().ok_or_else(|| CaptureError::BadMetadata("no portal response".into()))?;
    let (code, mut results): (u32, HashMap<String, OwnedValue>) = msg.body().deserialize()?;
    match code {
        0 => {}
        1 => return Err(CaptureError::Cancelled),
        2 => return Err(CaptureError::PortalDenied),
        other => return Err(CaptureError::BadMetadata(format!("portal returned {other}"))),
    }
    let uri: String = results
        .remove("uri")
        .ok_or_else(|| CaptureError::BadMetadata("portal gave no uri".into()))?
        .try_into()
        .map_err(|e: zbus::zvariant::Error| CaptureError::BadMetadata(e.to_string()))?;
    Ok(url::Url::parse(&uri).unwrap().to_file_path().unwrap())
}