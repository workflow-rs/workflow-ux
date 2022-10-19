#[cfg(target_arch = "wasm32")]
pub use ::async_trait::async_trait_without_send as async_trait;

//pub use ::async_trait::async_trait as async_trait;

#[cfg(not(target_arch = "wasm32"))]
pub use ::async_trait::async_trait_with_send as async_trait;
