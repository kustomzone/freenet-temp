use crate::app::TryNodeAction;

pub(crate) fn log(msg: impl AsRef<str>) {
    let msg = msg.as_ref();
    #[cfg(target_family = "wasm")]
    {
        web_sys::console::info_1(&serde_wasm_bindgen::to_value(&msg).unwrap());
    }
    let _ = msg;
}

pub(crate) fn error(msg: impl AsRef<str>, action: Option<TryNodeAction>) {
    let error = msg.as_ref();
    if let Some(action) = action {
        tracing::error!(%error, %action);
        #[cfg(target_family = "wasm")]
        {
            let error = format!("error while `{action}`: {error}");
            web_sys::console::error_1(&serde_wasm_bindgen::to_value(&error).unwrap());
        }
    } else {
        tracing::error!(%error);
        #[cfg(target_family = "wasm")]
        {
            web_sys::console::error_1(&serde_wasm_bindgen::to_value(&error).unwrap());
        }
    }
}
