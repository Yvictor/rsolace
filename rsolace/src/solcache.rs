use super::utils::ConvertToCString;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;

#[derive(Debug)]
pub struct CacheSessionProps {
    cache_name: CString,
    max_msgs: CString,
    max_age: CString,
    request_reply_timeout_ms: CString,
}

impl CacheSessionProps {
    pub fn to_c(&self) -> [*const c_char; 9] {
        [
            rsolace_sys::SOLCLIENT_CACHESESSION_PROP_CACHE_NAME.as_ptr() as *const c_char,
            self.cache_name.as_ptr() as *const c_char,
            rsolace_sys::SOLCLIENT_CACHESESSION_PROP_MAX_MSGS.as_ptr() as *const c_char,
            self.max_msgs.as_ptr() as *const c_char,
            rsolace_sys::SOLCLIENT_CACHESESSION_PROP_MAX_AGE.as_ptr() as *const c_char,
            self.max_age.as_ptr() as *const c_char,
            rsolace_sys::SOLCLIENT_CACHESESSION_PROP_REQUESTREPLY_TIMEOUT_MS.as_ptr()
                as *const c_char,
            self.request_reply_timeout_ms.as_ptr() as *const c_char,
            null(),
        ]
    }

    pub fn cache_name(mut self, cache_name: &str) -> Self {
        self.cache_name = cache_name.to_cstring();
        self
    }

    pub fn max_msgs(mut self, max_msgs: u32) -> Self {
        self.max_msgs = max_msgs.to_cstring();
        self
    }

    pub fn max_age(mut self, max_age: u32) -> Self {
        self.max_age = max_age.to_cstring();
        self
    }

    pub fn request_reply_timeout(mut self, request_reply_timeout_ms: u32) -> Self {
        self.request_reply_timeout_ms = request_reply_timeout_ms.to_cstring();
        self
    }
}

impl Default for CacheSessionProps {
    fn default() -> Self {
        Self {
            cache_name: "".to_cstring(),
            max_msgs: 1.to_cstring(),
            max_age: 0.to_cstring(),
            request_reply_timeout_ms: 10000.to_cstring(),
        }
    }
}
