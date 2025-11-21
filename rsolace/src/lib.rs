pub mod solcache;
pub mod solclient;
pub mod solcontainer;
pub mod solevent;
pub mod solmsg;
pub mod types;
pub mod utils;

use std::sync::Once;
use std::ptr::null_mut;
use types::SolClientLogLevel;

/// 全局初始化標記，確保 solClient_initialize 只被調用一次
static SOLACE_INIT: Once = Once::new();

/// 確保 Solace 客戶端庫已初始化
///
/// 根據 Solace C API 官方文檔：
/// "solClient_initialize() must be called before any other API interface call is made"
///
/// 這個函數使用 std::sync::Once 確保初始化只發生一次，
/// 可以在任何需要使用 Solace API 的地方安全地調用。
///
/// 自動初始化使用 Warning 級別的日誌。
/// 如果需要自定義日誌級別，請在調用任何 Solace API 之前顯式創建 SolClient。
pub fn ensure_solace_initialized() {
    SOLACE_INIT.call_once(|| {
        unsafe {
            // 使用 Warning 級別作為默認日誌級別
            rsolace_sys::solClient_initialize(
                SolClientLogLevel::Warning as rsolace_sys::solClient_log_level_t,
                null_mut(),
            );
        }
        tracing::debug!("Solace client library initialized automatically");
    });
}
