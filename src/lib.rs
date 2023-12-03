// Ensures this builds on other arches too, so clippy works
cfg_if::cfg_if! {
    if #[cfg(target_arch="wasm32")] {
        mod input;
        pub use input::*;

        pub mod log;

        pub mod state;
    }
}
