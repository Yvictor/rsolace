use super::types::SolClientSessionEvent;
use snafu::prelude::{ensure, Snafu};
use snafu::ResultExt;
use std::ffi::CStr;

#[derive(Debug, Clone)]
pub struct SolEvent {
    pub session_event: SolClientSessionEvent,
    pub response_code: u32,
    pub info: String,
    // correlation: String,
}

#[derive(Debug, Snafu)]
pub enum SolEventError {
    #[snafu(display("Null event pointer"))]
    NullEventPtr,
    #[snafu(display("Null info pointer"))]
    NullInfoPtr,
    #[snafu(display("Info UTF-8 error"))]
    InfoUtf8 { source: std::str::Utf8Error },
}

impl SolEvent {
    pub fn new(session_event: SolClientSessionEvent, response_code: u32, info: &str) -> SolEvent {
        SolEvent {
            session_event,
            response_code,
            info: info.to_string(),
        }
    }

    /// # Safety
    /// 呼叫端需確保 `event_p` 來源生命週期有效。
    pub unsafe fn from_ptr(
        event_p: rsolace_sys::solClient_session_eventCallbackInfo_pt,
    ) -> Result<SolEvent, SolEventError> {
        ensure!(!event_p.is_null(), NullEventPtrSnafu);
        let event = *event_p;

        ensure!(!event.info_p.is_null(), NullInfoPtrSnafu);
        let info = CStr::from_ptr(event.info_p)
            .to_str()
            .context(InfoUtf8Snafu)?
            .to_owned();

        Ok(SolEvent {
            session_event: event.sessionEvent.into(),
            response_code: event.responseCode,
            info,
        })
    }

    pub fn get_session_event_string(&self) -> String {
        unsafe {
            let res = rsolace_sys::solClient_session_eventToString(
                self.session_event as rsolace_sys::solClient_session_event_t,
            );
            CStr::from_ptr(res).to_str().unwrap().to_owned()
        }
    }
}
