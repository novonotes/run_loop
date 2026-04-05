//! プラットフォーム固有のネイティブ run loop（CFRunLoop / ALooper / GMainContext / Win32
//! メッセージループ）を共通の API で扱うクレートです。
//! [irondash_run_loop](https://github.com/irondash/irondash) をベースに、
//! DLL・オーディオプラグイン環境での安全性を強化したフォークです。
//!
//! 使い方・サンプルコードは [README](https://github.com/novonotes/run_loop) を参照してください。
//! 設計の背景は [docs/maintainers.md](../docs/maintainers.md) を参照してください。
//!
//! ## 注意点
//!
//! - [`RunLoop::current()`] は run loop スレッドからのみ呼び出せます。他スレッドからは [`RunLoop::sender()`] を使ってください。
//! - `init()` と `deinit()` は必ず対にしてください（内部で参照カウントを管理しています）。
//! - テストは singleton 制約があるため `#[serial_test::serial]` で直列化が必要です（[`test_harness`] 参照）。

#![allow(clippy::new_without_default)]

mod handle;
mod main_thread;
mod run_loop;
mod run_loop_sender;
mod task;
pub mod test_harness;
pub mod test_helper;
mod thread_id;

pub use handle::*;
pub use run_loop::*;
pub use run_loop_sender::*;
pub use task::*;
pub use thread_id::*;

// Note: These modules are public but there are no API stability guarantees
pub mod platform;
pub mod util;
