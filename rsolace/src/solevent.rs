use super::types::SolClientSessionEvent;
use enum_primitive::FromPrimitive;
use failure::Error;
use std::ffi::CStr;

#[derive(Debug, Clone)]
pub struct SolEvent {
    pub session_event: SolClientSessionEvent,
    pub response_code: u32,
    pub info: String,
    // correlation: String,
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
    ///
    /// This function should not be called by check  event_ptr is valid?.
    pub unsafe fn from_ptr(
        event_p: rsolace_sys::solClient_session_eventCallbackInfo_pt,
    ) -> Result<SolEvent, Error> {
        unsafe {
            let event = *event_p;
            Ok(SolEvent {
                session_event: SolClientSessionEvent::from_u32(event.sessionEvent).unwrap(),
                response_code: event.responseCode,
                info: CStr::from_ptr(event.info_p).to_str().unwrap().to_owned(),
            })
        }
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
