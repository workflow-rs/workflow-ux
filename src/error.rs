use downcast::DowncastError;
use wasm_bindgen::JsValue;
//, convert::{WasmAbi, IntoWasmAbi, FromWasmAbi}};
use core::num::{ParseFloatError, ParseIntError};
use hex::FromHexError;
use qrcodegen::DataTooLong;
use serde_wasm_bindgen::Error as SerdeError;
use std::io::Error as IoError;
use std::sync::PoisonError;
use thiserror::Error;
use workflow_core::channel::{RecvError, SendError, TrySendError};
use workflow_i18n::Error as i18nError;
use workflow_wasm::callback::CallbackError;

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => ( workflow_ux::error::Error::String(format_args!($($t)*).to_string()) )
}
pub use error;

#[allow(non_camel_case_types)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    String(String),

    #[error("JsValue: {0:?}")]
    JsValue(JsValue),

    #[error("Module {0} registration failure: {1}")]
    ModuleRegistrationFailure(String, String),

    #[error("WebElement: {0:?}")]
    WebElement(web_sys::Element),

    #[error("PoisonError: {0:?}")]
    PoisonError(String),

    #[error("Channel send error: {0}")]
    ChannelSendError(String),

    #[error("Channel try_send error: {0}")]
    ChannelTrySendError(String),

    #[error("Channel receive error: {0}")]
    ChannelReceiveError(String),

    #[error("Parent element not found {0:?}")]
    ParentNotFound(web_sys::Element),

    #[error("Layout<{0}>: the supplied HTML contains bindings that are not used: {1}")]
    MissingLayoutBindings(String, String),

    #[error("[{0}]: Unable to locate element with id {1}")]
    MissingElement(String, String),

    #[error("[{0}]: Unable to locate parent element with id {1}")]
    MissingParent(String, String),

    #[error("Menu: icon-box element is missing")]
    MissingIconBox,

    #[error("Application global is not initialized")]
    ApplicationGlobalNotInitialized,

    #[error("{0}")]
    i18nError(#[from] i18nError),

    #[error("data_types_to_modules map is missing; ensure modules::seal() is invoked after module registration")]
    DataTypesToModuleMapMissing,

    #[error("Unable to obtain document body")]
    UnableToGetBody,

    // #[error("Timer error: {0}")]
    // TimerError(#[from] workflow_wasm::timers::Error),

    #[error("Dialog error: {0}")]
    DialogError(String),

    #[error("Downcast error: {0}")]
    Downcast(String),

    #[error("IoError error: {0}")]
    IoError(IoError),

    #[error("ParseFloatError error: {0}")]
    ParseFloatError(ParseFloatError),

    #[error("ParseIntError error: {0}")]
    ParseIntError(ParseIntError),

    #[error("FromHexError error: {0}")]
    FromHexError(FromHexError),

    #[error("DataTooLong error: {0}")]
    DataTooLong(#[from] DataTooLong),

    #[error("CallbackError error: {0}")]
    CallbackError(#[from] CallbackError),

    #[error("DOM error: {0}")]
    DomError(#[from] workflow_dom::error::Error),
}

unsafe impl Send for Error {}

impl From<FromHexError> for Error {
    fn from(error: FromHexError) -> Error {
        Self::FromHexError(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Error {
        Self::ParseIntError(error)
    }
}

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Error {
        Self::ParseFloatError(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Self::IoError(error)
    }
}

impl From<JsValue> for Error {
    fn from(val: JsValue) -> Self {
        Self::JsValue(val)
    }
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Self {
        Self::JsValue(e.into())
    }
}

impl From<Error> for JsValue {
    fn from(error: Error) -> JsValue {
        JsValue::from(format!("{error:?}"))
    }
}

// impl Into<JsValue> for Error {
//     fn into(self) -> Self {
//         JsValue::from(format!("{}",self).to_string())
//     }
// }

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Self::PoisonError(format!("{err:?}"))
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::String(val.to_string())
    }
}

impl From<String> for Error {
    fn from(val: String) -> Self {
        Self::String(val)
    }
}

impl From<web_sys::Element> for Error {
    fn from(el: web_sys::Element) -> Self {
        Self::WebElement(el)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(error: SendError<T>) -> Error {
        Error::ChannelSendError(format!("{error:?}"))
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(e: TrySendError<T>) -> Self {
        Self::ChannelTrySendError(e.to_string())
    }
}

impl From<RecvError> for Error {
    fn from(error: RecvError) -> Error {
        Error::ChannelReceiveError(format!("{error:?}"))
    }
}

impl<T> From<DowncastError<T>> for Error {
    fn from(error: DowncastError<T>) -> Error {
        Error::Downcast(format!("{error:?}"))
    }
}

// impl WasmAbi for Error {}

// impl IntoWasmAbi for u128 {
//     type Abi = Wasm128;

//     #[inline]
//     fn into_abi(self) -> Wasm128 {
//         Wasm128 {
//             low: (self as u64).into_abi(),
//             high: ((self >> 64) as u64).into_abi(),
//         }
//     }
// }

// impl FromWasmAbi for u128 {
//     type Abi = Wasm128;

//     #[inline]
//     unsafe fn from_abi(js: Wasm128) -> u128 {
//         u128::from(u64::from_abi(js.low)) | (u128::from(u64::from_abi(js.high)) << 64)
//     }
// }
