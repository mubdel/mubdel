use anyhow::{bail, Result};
use web_sys::{self, Storage};

fn local_storage() -> Result<Storage> {
    match web_sys::window() {
        Some(window) => match window.local_storage() {
            Ok(st) => match st {
                Some(storage) => Ok(storage),
                None => bail!("none storage"),
            },
            Err(err) => bail!("local storage error: {:?}", err),
        },
        None => bail!("none window"),
    }
}

const AUTHENTICATED: &str = "mdl_auth";

pub fn is_authenticated() -> bool {
    if let Ok(local_storage) = local_storage() {
        if let Ok(Some(value)) = local_storage.get_item(AUTHENTICATED) {
            if value == "true" {
                return true;
            }
        }
    }
    false
}
