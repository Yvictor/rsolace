use super::types::{
    SolClientCacheStatus, SolClientDeliveryMode, SolClientDestType, SolClientReturnCode,
};
use enum_primitive::FromPrimitive;
use std::borrow::Cow;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
// use std::marker::PhantomData;
// use std::option::Option;
use chrono::DateTime;
use snafu::prelude::{ensure, Snafu};
use snafu::{OptionExt, ResultExt};
use std::ptr::null_mut;

pub struct SolMsg {
    msg_p: rsolace_sys::solClient_opaqueMsg_pt,
    user_prop_p: Option<rsolace_sys::solClient_opaqueContainer_pt>,
    // _ph: PhantomData<&'a ()>,
    // container_p: Option<rsolace_sys::solClient_opaqueContainer_pt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Destination {
    pub dest_type: SolClientDestType,
    pub dest: String,
}

#[derive(Debug, Snafu, PartialEq)]
pub enum SolMsgError {
    #[snafu(display("SolMsg alloc {msg_p:?} Error"))]
    Alloc {
        msg_p: rsolace_sys::solClient_opaqueMsg_pt,
    },
    #[snafu(display("SolMsg from invalid ptr {msg_p:?} Error"))]
    FromInvalidPtr {
        msg_p: rsolace_sys::solClient_opaqueMsg_pt,
    },
    #[snafu(display("SolMsg get {attr} Error"))]
    GetAttr { attr: String },
    #[snafu(display("SolMsg get {attr} empty Error"))]
    GetAttrEmpty { attr: String },
    #[snafu(display("SolMsg get {attr} {}", source))]
    GetAttrUtf8Error {
        source: std::str::Utf8Error,
        attr: String,
    },
    #[snafu(display("SolMsg without user prop"))]
    UserPropNotExist,
}

// pub trait FromCptr {
//     pub fn from_ptr
// }

impl Destination {
    pub fn new(dest_type: SolClientDestType, dest: &str) -> Destination {
        Destination {
            dest_type,
            dest: dest.to_string(),
        }
    }
    pub fn from_ptr(dest_p: rsolace_sys::solClient_destination_t) -> Destination {
        Destination {
            dest: unsafe { CStr::from_ptr(dest_p.dest).to_str().unwrap().to_string() },
            dest_type: SolClientDestType::from_i32(dest_p.destType).unwrap(),
        }
    }
}

impl SolMsg {
    pub fn new() -> Result<SolMsg, SolMsgError> {
        let mut msg_p: rsolace_sys::solClient_opaqueMsg_pt = null_mut();
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_alloc(&mut msg_p);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                AllocSnafu { msg_p }
            );
            // if rt_code != (SolClientReturnCode::Ok as i32) {
            //     return Err(SolMsgError::Alloc(msg_p));
            // }
        }
        Ok(SolMsg {
            msg_p,
            user_prop_p: None,
            // _ph: PhantomData,
            // container_p: None,
        })
    }

    /// # Safety
    ///
    /// This function should not be called by check msg_ptr is valid?.
    pub unsafe fn from_ptr(
        msg_p: rsolace_sys::solClient_opaqueMsg_pt,
    ) -> Result<SolMsg, SolMsgError> {
        // TODO how to check the ptr is valid
        let mut mode = 0;
        let mut user_prop_p: rsolace_sys::solClient_opaqueContainer_pt = null_mut();
        let rt_code = unsafe { rsolace_sys::solClient_msg_getDeliveryMode(msg_p, &mut mode) };
        ensure!(
            rt_code == (SolClientReturnCode::Ok as i32),
            FromInvalidPtrSnafu { msg_p }
        );
        let rt_code =
            unsafe { rsolace_sys::solClient_msg_getUserPropertyMap(msg_p, &mut user_prop_p) };
        match SolClientReturnCode::from_i32(rt_code).unwrap() {
            SolClientReturnCode::Ok => Ok(SolMsg {
                msg_p,
                user_prop_p: Some(user_prop_p),
                // _ph: PhantomData,
            }),
            _ => {
                Ok(SolMsg {
                    msg_p,
                    user_prop_p: None, //Some(user_prop_p)
                                       // _ph: PhantomData,
                })
            }
        }
    }

    pub fn get_ptr(&self) -> rsolace_sys::solClient_opaqueMsg_pt {
        self.msg_p
    }

    pub fn set_delivery_mode(&mut self, mode: SolClientDeliveryMode) -> SolClientReturnCode {
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_setDeliveryMode(self.msg_p, mode as u32);
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn get_delivery_mode(&self) -> Result<SolClientDeliveryMode, SolMsgError> {
        let mut mode = 0;
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_getDeliveryMode(self.msg_p, &mut mode);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetAttrSnafu {
                    attr: "deliver_mode".to_string(),
                }
            );
            Ok(SolClientDeliveryMode::from_u32(mode).unwrap())
        }
    }

    pub fn set_destination(&mut self, dest: &Destination) -> SolClientReturnCode {
        let dest_dest_cstr = CString::new(dest.dest.clone()).unwrap();
        let mut dest_c = rsolace_sys::solClient_destination {
            destType: dest.dest_type as i32,
            dest: dest_dest_cstr.as_ptr(),
        };
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_setDestination(
                self.msg_p,
                (&mut dest_c) as *mut rsolace_sys::solClient_destination,
                std::mem::size_of::<rsolace_sys::solClient_destination>(),
            );
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn get_destination(&self) -> Result<Destination, SolMsgError> {
        let mut dest_c = rsolace_sys::solClient_destination {
            destType: SolClientDestType::Null as i32,
            dest: null_mut(),
        };
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_getDestination(
                self.msg_p,
                (&mut dest_c) as *mut rsolace_sys::solClient_destination,
                std::mem::size_of::<rsolace_sys::solClient_destination>(),
            );
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetAttrSnafu {
                    attr: "destination".to_string(),
                }
            );
            Ok(Destination::from_ptr(dest_c))
            // SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn set_reply_to(&mut self, dest: &Destination) -> SolClientReturnCode {
        let dest_dest_cstr = CString::new(dest.dest.clone()).unwrap();
        let mut dest_c = rsolace_sys::solClient_destination {
            destType: dest.dest_type as i32,
            dest: dest_dest_cstr.as_ptr(),
        };
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_setReplyTo(
                self.msg_p,
                (&mut dest_c) as *mut rsolace_sys::solClient_destination,
                std::mem::size_of::<rsolace_sys::solClient_destination>(),
            );
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn get_reply_to(&self) -> Result<Destination, SolMsgError> {
        let mut dest_c = rsolace_sys::solClient_destination {
            destType: SolClientDestType::Null as i32,
            dest: null_mut(),
        };
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_getReplyTo(
                self.msg_p,
                (&mut dest_c) as *mut rsolace_sys::solClient_destination,
                std::mem::size_of::<rsolace_sys::solClient_destination>(),
            );
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetAttrSnafu {
                    attr: "reply_to".to_string(),
                }
            );
            Ok(Destination::from_ptr(dest_c))
        }
    }

    pub fn del_reply_to(&mut self) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_deleteReplyTo(self.msg_p)
        })
        .unwrap()
    }

    pub fn set_as_reply(&mut self, is_reply: bool) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_setAsReplyMsg(self.msg_p, is_reply as u8)
        })
        .unwrap()
    }

    pub fn is_reply(&self) -> bool {
        unsafe { rsolace_sys::solClient_msg_isReplyMsg(self.msg_p) == 1 }
    }

    pub fn set_eliding_eligible(&mut self, elide: bool) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_setElidingEligible(self.msg_p, elide as u8)
        })
        .unwrap()
    }

    pub fn is_eliding_eligible(&self) -> bool {
        unsafe { rsolace_sys::solClient_msg_isElidingEligible(self.msg_p) == 1 }
    }

    pub fn is_p2p(&self) -> bool {
        match self.get_topic() {
            Ok(topic) => matches!(&topic[..4], "#P2P"),
            Err(_) => false,
        }
    }

    pub fn set_correlation_id(&mut self, corr_id: &str) -> SolClientReturnCode {
        let corr_id_c = CString::new(corr_id).unwrap();
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_setCorrelationId(self.msg_p, corr_id_c.as_ptr())
        })
        .unwrap()
    }

    pub fn del_correlation_id(&mut self) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_deleteCorrelationId(self.msg_p)
        })
        .unwrap()
    }

    pub fn get_correlation_id(&self) -> Result<String, SolMsgError> {
        unsafe {
            let mut corr_id: *const std::os::raw::c_char = null_mut();
            let rt_code = rsolace_sys::solClient_msg_getCorrelationId(self.msg_p, &mut corr_id);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetAttrSnafu {
                    attr: "corrid".to_string()
                }
            );
            let corr_id = CStr::from_ptr(corr_id)
                .to_str()
                .context(GetAttrUtf8Snafu { attr: "corrid" })?;
            // Ok(Cow::Borrowed(corr_id))
            Ok(corr_id.to_string())
        }
    }

    pub fn set_class_of_service(&mut self, cos: u32) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_setClassOfService(self.msg_p, cos - 1)
        })
        .unwrap()
    }

    pub fn get_class_of_service(&self) -> Result<u32, SolMsgError> {
        let mut cos = 0;
        let rt_code = unsafe { rsolace_sys::solClient_msg_getClassOfService(self.msg_p, &mut cos) };

        ensure!(
            rt_code == (SolClientReturnCode::Ok as i32),
            GetAttrSnafu {
                attr: "class_of_service".to_string()
            }
        );

        Ok(cos + 1)
    }

    pub fn set_delivery_to_one(&mut self, dto: bool) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_setDeliverToOne(self.msg_p, dto as u8)
        })
        .unwrap()
    }

    pub fn is_delivery_to_one(&self) -> bool {
        unsafe { rsolace_sys::solClient_msg_isDeliverToOne(self.msg_p) == 1 }
    }

    pub fn set_topic(&mut self, topic: &str) -> SolClientReturnCode {
        let dest = Destination::new(SolClientDestType::Topic, topic);
        self.set_destination(&dest)
    }

    pub fn get_topic(&self) -> Result<String, SolMsgError> {
        let dest = self.get_destination()?;
        Ok(dest.dest)
    }

    pub fn set_reply_topic(&mut self, topic: &str) -> SolClientReturnCode {
        let dest = Destination::new(SolClientDestType::Topic, topic);
        self.set_reply_to(&dest)
    }

    pub fn get_reply_topic(&self) -> Result<String, SolMsgError> {
        let dest = self.get_reply_to()?;
        Ok(dest.dest)
    }

    pub fn get_sender_id(&self) -> Result<String, SolMsgError> {
        let mut sender_id: *const std::os::raw::c_char = null_mut();
        let rt_code = unsafe { rsolace_sys::solClient_msg_getSenderId(self.msg_p, &mut sender_id) };
        ensure!(
            rt_code == SolClientReturnCode::Ok as i32,
            GetAttrSnafu { attr: "sender_id" }
        );
        Ok(unsafe { CStr::from_ptr(sender_id) }
            .to_string_lossy()
            .to_string())
    }

    pub fn del_sender_id(&mut self) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_deleteSenderId(self.msg_p)
        })
        .unwrap()
    }

    pub fn set_sender_id(&mut self, sender_id: &str) -> SolClientReturnCode {
        let sender_id_c = CString::new(sender_id).unwrap();
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_setSenderId(self.msg_p, sender_id_c.as_ptr())
        })
        .unwrap()
    }

    pub fn set_sender_ts(&mut self, ts: i64) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_setSenderTimestamp(self.msg_p, ts)
        })
        .unwrap()
    }

    pub fn del_sender_ts(&mut self) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_deleteSenderTimestamp(self.msg_p)
        })
        .unwrap()
    }

    pub fn get_sender_dt(&self) -> Result<DateTime<chrono::Utc>, SolMsgError> {
        let mut ts = 0;
        let rt_code = unsafe { rsolace_sys::solClient_msg_getSenderTimestamp(self.msg_p, &mut ts) };
        ensure!(
            rt_code == SolClientReturnCode::Ok as i32,
            GetAttrSnafu {
                attr: "sender_time"
            }
        );
        let datetime = DateTime::from_timestamp_millis(ts).context(GetAttrEmptySnafu {
            attr: "sender_time",
        })?;
        Ok(datetime)
    }

    pub fn get_sender_ts(&self) -> Result<i64, SolMsgError> {
        let mut ts = 0;
        let rt_code = unsafe { rsolace_sys::solClient_msg_getSenderTimestamp(self.msg_p, &mut ts) };
        ensure!(
            rt_code == SolClientReturnCode::Ok as i32,
            GetAttrSnafu {
                attr: "sender_time"
            }
        );
        Ok(ts)
    }

    pub fn get_recv_ts(&self) -> Result<i64, SolMsgError> {
        let mut ts = 0;
        let rt_code = unsafe { rsolace_sys::solClient_msg_getRcvTimestamp(self.msg_p, &mut ts) };
        ensure!(
            rt_code == SolClientReturnCode::Ok as i32,
            GetAttrSnafu { attr: "recv_time" }
        );
        Ok(ts)
    }

    pub fn get_seq(&self) -> Result<i64, SolMsgError> {
        let mut seq_num = 0;
        let rt_code =
            unsafe { rsolace_sys::solClient_msg_getSequenceNumber(self.msg_p, &mut seq_num) };
        ensure!(
            rt_code == SolClientReturnCode::Ok as i32,
            GetAttrSnafu { attr: "seq_num" }
        );
        Ok(seq_num)
    }

    pub fn set_seq(&mut self, seq: u64) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_setSequenceNumber(self.msg_p, seq)
        })
        .unwrap()
    }

    pub fn del_seq(&mut self) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_deleteSequenceNumber(self.msg_p)
        })
        .unwrap()
    }

    pub fn get_msg_type(&self) -> Result<Cow<str>, SolMsgError> {
        let mut msg_type: *const std::os::raw::c_char = null_mut();
        let rt_code =
            unsafe { rsolace_sys::solClient_msg_getApplicationMsgType(self.msg_p, &mut msg_type) };
        ensure!(
            rt_code == SolClientReturnCode::Ok as i32,
            GetAttrSnafu { attr: "msg_type" }
        );
        Ok(unsafe { CStr::from_ptr(msg_type) }.to_string_lossy())
    }

    pub fn set_msg_type(&self, msg_type: &str) -> SolClientReturnCode {
        let msg_type_c = CString::new(msg_type).unwrap();
        let rt_code = unsafe {
            rsolace_sys::solClient_msg_setApplicationMsgType(self.msg_p, msg_type_c.as_ptr())
        };
        SolClientReturnCode::from_i32(rt_code).unwrap()
    }

    pub fn del_msg_type(&mut self) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_deleteApplicationMsgType(self.msg_p)
        })
        .unwrap()
    }

    pub fn get_cache_request_id(&self) -> Result<u64, SolMsgError> {
        let mut cache_req_id: u64 = 0;
        let rt_code =
            unsafe { rsolace_sys::solClient_msg_getCacheRequestId(self.msg_p, &mut cache_req_id) };
        ensure!(
            rt_code == SolClientReturnCode::Ok as i32,
            GetAttrSnafu {
                attr: "cache_req_id"
            }
        );
        Ok(cache_req_id)
    }

    pub fn get_cache_status(&self) -> SolClientCacheStatus {
        let cache_status = unsafe { rsolace_sys::solClient_msg_isCacheMsg(self.msg_p) };
        SolClientCacheStatus::from_i32(cache_status).unwrap()
    }

    pub fn is_cache(&self) -> bool {
        let cache_status = self.get_cache_status();
        cache_status == SolClientCacheStatus::Cache
    }

    pub fn is_discard_indication(&self) -> bool {
        let is_discard_indication =
            unsafe { rsolace_sys::solClient_msg_isDiscardIndication(self.msg_p) };
        is_discard_indication == 1
    }

    // TODO add reset setter getter delete

    pub fn get_user_prop(&self, key: &str) -> Result<String, SolMsgError> {
        match self.user_prop_p {
            Some(user_prop_p) => {
                let key_c = CString::new(key).unwrap();
                let mut value_c: *const std::os::raw::c_char = null_mut();
                let rt_code = unsafe {
                    rsolace_sys::solClient_container_getStringPtr(
                        user_prop_p,
                        &mut value_c,
                        key_c.as_ptr(),
                    )
                };
                ensure!(
                    rt_code == SolClientReturnCode::Ok as i32,
                    GetAttrSnafu { attr: key }
                );
                let value = unsafe { CStr::from_ptr(value_c) }
                    .to_str()
                    .context(GetAttrUtf8Snafu { attr: key })?;
                Ok(value.to_string())
                // rsolace_sys::solClient_container_getString(container_p, string, size, name)
            }
            None => Err(SolMsgError::UserPropNotExist),
        }
    }

    pub fn set_user_prop(&mut self, key: &str, value: &str, map_size: u32) -> SolClientReturnCode {
        let key_c = CString::new(key).unwrap();
        let value_c = CString::new(value).unwrap();
        match self.user_prop_p {
            Some(use_prop_p) => unsafe {
                let rt_code = rsolace_sys::solClient_container_addString(
                    use_prop_p,
                    value_c.as_ptr(),
                    key_c.as_ptr(),
                );
                SolClientReturnCode::from_i32(rt_code).unwrap()
            },
            None => unsafe {
                let mut user_prop_p: rsolace_sys::solClient_opaqueContainer_pt = null_mut();
                let rt_code = rsolace_sys::solClient_msg_createUserPropertyMap(
                    self.msg_p,
                    &mut user_prop_p,
                    map_size,
                );
                if rt_code == (SolClientReturnCode::Ok as i32) {
                    self.user_prop_p = Some(user_prop_p);
                    let rt_code = rsolace_sys::solClient_container_addString(
                        self.user_prop_p.unwrap(),
                        value_c.as_ptr(),
                        key_c.as_ptr(),
                    );
                    SolClientReturnCode::from_i32(rt_code).unwrap()
                } else {
                    SolClientReturnCode::from_i32(rt_code).unwrap()
                }
            },
        }
    }

    pub fn set_binary_attachment(&mut self, data: &[u8]) -> SolClientReturnCode {
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_setBinaryAttachment(
                self.msg_p,
                data.as_ptr() as *const c_void,
                data.len() as u32,
            );
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn get_binary_attachment(&self) -> Result<Cow<[u8]>, SolMsgError> {
        let mut data_ptr = null_mut();
        let mut data_len = 0;
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_getBinaryAttachmentPtr(
                self.msg_p,
                &mut data_ptr,
                &mut data_len,
            );
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetAttrEmptySnafu {
                    attr: "binary_attachment"
                }
            );
            ensure!(
                data_len > 0,
                GetAttrEmptySnafu {
                    attr: "binary_attachment"
                }
            );
            // assert!(!data_ptr.is_null());
            let s = std::slice::from_raw_parts(data_ptr as *const u8, data_len as usize);
            Ok(Cow::Borrowed(s))
        }
    }

    pub fn dump(&self, display_only: bool) -> Option<Cow<str>> {
        if display_only {
            unsafe {
                rsolace_sys::solClient_msg_dump(self.msg_p, null_mut(), 0);
                None
            }
        } else {
            let mut buffer_p: [std::os::raw::c_char; 4096] = [0; 4096];
            // let buffer_p: *mut std::os::raw::c_char = null_mut();
            unsafe {
                rsolace_sys::solClient_msg_dump(
                    self.msg_p,
                    &mut buffer_p as *mut std::os::raw::c_char,
                    4096,
                );
                // println!("buffer_p: {:?}", buffer_p);
                Some(CStr::from_ptr(&buffer_p as *const c_char).to_string_lossy())
                // match CStr::from_ptr(&buffer_p as *const i8).to_string_lossy() {
                //     Ok(dump) => Some(dump.to_string()),
                //     Err(e) => {
                //         tracing::error!("Error converting buffer to string: {}", e);
                //         None
                //     }
                // }
            }
        }
    }
}

impl std::fmt::Debug for SolMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("SolMsg").field("ptr", &self.msg_p).finish();
        write!(
            f,
            "SolMsg {:?} \n{}",
            &self.msg_p,
            &self.dump(false).unwrap_or("None".into())
        )
    }
}

impl Drop for SolMsg {
    fn drop(&mut self) {
        tracing::debug!("solmsg: {:?} drop call", self.msg_p);
        unsafe {
            rsolace_sys::solClient_msg_free(&mut self.msg_p);
        }
    }
}

unsafe impl Send for SolMsg {}

#[derive(Debug, Clone)]
pub struct SolMsgBuilder {
    delivery_mode: SolClientDeliveryMode,
    destination: Option<Destination>,
    reply_to: Option<Destination>,
    is_reply: Option<bool>,
    eliding_eligible: Option<bool>,
    correlation_id: Option<String>,
    cos: Option<u32>,
    is_delivery_to_one: Option<bool>,
    user_props: Vec<(String, String)>,
    binary_attachment: Option<Vec<u8>>,
}

impl Default for SolMsgBuilder {
    fn default() -> Self {
        SolMsgBuilder {
            delivery_mode: SolClientDeliveryMode::Direct,
            destination: None,
            reply_to: None,
            is_reply: None,
            eliding_eligible: None,
            correlation_id: None,
            cos: None,
            is_delivery_to_one: None,
            user_props: Vec::new(),
            binary_attachment: None,
        }
    }
}

impl SolMsgBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_delivery_mode(mut self, delivery_mode: SolClientDeliveryMode) -> Self {
        self.delivery_mode = delivery_mode;
        self
    }

    pub fn with_destination(mut self, destination: Destination) -> Self {
        self.destination = Some(destination);
        self
    }

    pub fn with_topic(mut self, topic: &str) -> Self {
        self.destination = Some(Destination::new(SolClientDestType::Topic, topic));
        self
    }

    pub fn with_reply_to(mut self, destination: Destination) -> Self {
        self.reply_to = Some(destination);
        self
    }

    pub fn with_reply_to_topic(mut self, topic: &str) -> Self {
        self.reply_to = Some(Destination::new(SolClientDestType::Topic, topic));
        self
    }

    pub fn as_reply(mut self, is_reply: bool) -> Self {
        self.is_reply = Some(is_reply);
        self
    }
    pub fn as_eliding_eligible(mut self, is_eliding_eligible: bool) -> Self {
        self.eliding_eligible = Some(is_eliding_eligible);
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: &str) -> Self {
        self.correlation_id = Some(correlation_id.to_string());
        self
    }

    pub fn with_class_of_service(mut self, cos: u32) -> Self {
        self.cos = Some(cos);
        self
    }

    pub fn as_delivery_to_one(mut self, is_delivery_to_one: bool) -> Self {
        self.is_delivery_to_one = Some(is_delivery_to_one);
        self
    }

    pub fn with_user_prop(mut self, key: &str, value: &str) -> Self {
        self.user_props.push((key.to_string(), value.to_string()));
        self
    }

    pub fn with_binary_attachment(mut self, binary_attachment: Vec<u8>) -> Self {
        self.binary_attachment = Some(binary_attachment);
        self
    }

    pub fn build(self) -> SolMsg {
        let mut m = SolMsg::new().unwrap();
        m.set_delivery_mode(self.delivery_mode);
        if let Some(dest) = self.destination {
            m.set_destination(&dest);
        }
        if let Some(reply_to) = self.reply_to {
            m.set_reply_to(&reply_to);
        }
        if let Some(is_reply) = self.is_reply {
            m.set_as_reply(is_reply);
        }
        if let Some(is_eliding_eligible) = self.eliding_eligible {
            m.set_eliding_eligible(is_eliding_eligible);
        }
        if let Some(correlation_id) = self.correlation_id {
            m.set_correlation_id(&correlation_id);
        }
        if let Some(cos) = self.cos {
            m.set_class_of_service(cos);
        }
        if let Some(is_delivery_to_one) = self.is_delivery_to_one {
            m.set_delivery_to_one(is_delivery_to_one);
        }
        for (key, value) in self.user_props {
            m.set_user_prop(&key, &value, 24);
        }
        if let Some(binary_attachment) = self.binary_attachment {
            m.set_binary_attachment(&binary_attachment);
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;

    use crate::types::{
        SolClientCacheStatus, SolClientDeliveryMode, SolClientDestType, SolClientReturnCode,
    };

    use super::{Destination, SolMsg, SolMsgBuilder, SolMsgError};

    use rstest::{fixture, rstest};

    #[fixture]
    pub fn solmsg() -> SolMsg {
        // SolMsg::new().unwrap()
        SolMsgBuilder::new().build()
    }

    #[test]
    fn solmsg_from_ptr() {
        let res = unsafe { SolMsg::from_ptr(null_mut()) };
        assert!(res.is_err())
    }

    #[rstest]
    #[case(SolClientDeliveryMode::Direct)]
    #[case(SolClientDeliveryMode::Persistent)]
    #[case(SolClientDeliveryMode::NonPersistent)]
    fn solmsg_delivery_mode_workable(mut solmsg: SolMsg, #[case] mode: SolClientDeliveryMode) {
        assert_eq!(
            solmsg.get_delivery_mode().unwrap(),
            SolClientDeliveryMode::Direct
        );
        solmsg.set_delivery_mode(mode);
        assert_eq!(solmsg.get_delivery_mode().unwrap(), mode);
    }

    #[rstest]
    fn solmsg_dest_workable() {
        let mut solmsg = SolMsg::new().unwrap();
        let dest = Destination::new(SolClientDestType::Topic, "TIC/v1/test");
        solmsg.set_destination(&dest);
        assert_eq!(solmsg.get_destination().unwrap(), dest);

        let dest = Destination::new(SolClientDestType::Queue, "TIC/v1/test");
        solmsg.set_destination(&dest);
        assert_eq!(solmsg.get_destination().unwrap(), dest);
    }

    #[rstest]
    fn solmsg_topic_workable(mut solmsg: SolMsg) {
        let topic = "TIC/v1/test";
        solmsg.set_topic(topic);
        assert_eq!(solmsg.get_topic().unwrap(), topic);
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn solmsg_reply_workable(mut solmsg: SolMsg, #[case] is_reply: bool) {
        solmsg.set_as_reply(is_reply);
        assert_eq!(solmsg.is_reply(), is_reply);
    }

    #[rstest]
    fn solmsg_reply_to_workable(mut solmsg: SolMsg) {
        let dest = Destination::new(SolClientDestType::Topic, "TIC/v1/test");
        solmsg.set_reply_to(&dest);
        assert_eq!(solmsg.get_reply_to().unwrap(), dest);
        solmsg.del_reply_to();
        assert!(solmsg.get_reply_to().is_err());
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn solmsg_elide_workable(mut solmsg: SolMsg, #[case] elide: bool) {
        solmsg.set_eliding_eligible(elide);
        assert_eq!(solmsg.is_eliding_eligible(), elide);
    }

    #[rstest]
    #[case("#P2P/abc", true)]
    #[case("TIC/v1/test1", false)]
    fn solmsg_is_p2p_workable(mut solmsg: SolMsg, #[case] topic: &str, #[case] is_p2p: bool) {
        solmsg.set_topic(topic);
        assert_eq!(solmsg.is_p2p(), is_p2p);
    }

    #[rstest]
    fn solmsg_corr_id_workable(mut solmsg: SolMsg) {
        let corr_id = "R1";
        solmsg.set_correlation_id(corr_id);
        assert_eq!(solmsg.get_correlation_id().unwrap(), corr_id);
        solmsg.del_correlation_id();
        assert!(solmsg.get_correlation_id().is_err());
    }

    #[rstest]
    fn solmsg_sender_id_workable(mut solmsg: SolMsg) {
        let sender = "R1";
        solmsg.set_sender_id(sender);
        assert_eq!(solmsg.get_sender_id().unwrap(), sender);
        solmsg.del_sender_id();
        assert!(solmsg.get_sender_id().is_err());
    }

    #[rstest]
    fn solmsg_sender_ts_workable(mut solmsg: SolMsg) {
        let sender_ts = 1234567890;
        solmsg.set_sender_ts(sender_ts);
        assert_eq!(solmsg.get_sender_ts().unwrap(), sender_ts);
        solmsg.del_sender_ts();
        assert!(solmsg.get_sender_ts().is_err());
    }

    #[rstest]
    fn solmsg_seq_workable(mut solmsg: SolMsg) {
        let seq = 1234567890;
        solmsg.set_seq(seq);
        assert_eq!(solmsg.get_seq().unwrap(), seq.try_into().unwrap());
        solmsg.del_seq();
        assert!(solmsg.get_seq().is_err());
    }

    #[rstest]
    fn solmsg_msg_type_workable(mut solmsg: SolMsg) {
        let msg_type = "msgpack";
        solmsg.set_msg_type(msg_type);
        assert_eq!(solmsg.get_msg_type().unwrap(), msg_type);
        solmsg.del_msg_type();
        assert!(solmsg.get_msg_type().is_err());
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn solmsg_delivery_to_one_workable(#[case] dto: bool) {
        let mut solmsg = SolMsg::new().unwrap();
        solmsg.set_delivery_to_one(dto);
        assert_eq!(solmsg.is_delivery_to_one(), dto);
    }

    #[rstest]
    fn solmsg_cache_status_workable(solmsg: SolMsg) {
        let cache_status = solmsg.get_cache_status();
        assert_eq!(cache_status, SolClientCacheStatus::Invalid);
        let is_cache = solmsg.is_cache();
        assert_eq!(is_cache, false);
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    fn solmsg_cos_workable(mut solmsg: SolMsg, #[case] cos: u32) {
        solmsg.set_class_of_service(cos);
        assert_eq!(solmsg.get_class_of_service().unwrap(), cos);
    }

    #[rstest]
    #[rstest]
    fn solmsg_user_prop_workable(mut solmsg: SolMsg) {
        let key = "ct";
        let value = "bytes/msgpack";
        let rt_code = solmsg.set_user_prop(key, value, 24);
        assert_eq!(rt_code, SolClientReturnCode::Ok);
        assert_eq!(value, solmsg.get_user_prop(key).unwrap());
        let key2 = "c2";
        let rt_code = solmsg.set_user_prop(key2, value, 24);
        assert_eq!(rt_code, SolClientReturnCode::Ok);
        assert_eq!(value, solmsg.get_user_prop(key2).unwrap());
    }

    #[rstest]
    fn solmsg_set_binary_attachment(mut solmsg: SolMsg) {
        let data = vec![0, 1, 2, 3, 4];
        let rt_code = solmsg.set_binary_attachment(&data);
        assert_eq!(rt_code, SolClientReturnCode::Ok)
    }

    #[rstest]
    fn solmsg_get_binary_attachment_err(solmsg: SolMsg) {
        let res = solmsg.get_binary_attachment();
        assert!(res.is_err());
        let err = res.unwrap_err();
        println!("{}", err);
        assert_eq!(
            err,
            SolMsgError::GetAttrEmpty {
                attr: "binary_attachment".to_string()
            }
        );
        // ensure!();
        // assert_eq!(res, Err(SolMsgError::GetAttr("binary_attachment")));
    }

    #[rstest]
    fn solmsg_get_binary_attachment(mut solmsg: SolMsg) {
        let data = vec![0, 1, 2, 3, 4];
        solmsg.set_binary_attachment(&data);
        // assert_eq!(rt_code, SolClientReturnCode::Ok)
        let res = solmsg.get_binary_attachment().unwrap();
        assert_eq!(res, data)
    }

    #[rstest]
    fn solmsg_builder_workable() {
        let solmsg = SolMsgBuilder::new()
            .with_delivery_mode(SolClientDeliveryMode::Direct)
            .with_destination(Destination::new(SolClientDestType::Topic, "TIC/v1/test"))
            .as_delivery_to_one(true)
            .with_correlation_id("R1")
            .with_class_of_service(1)
            .with_user_prop("ct", "bytes/msgpack")
            .with_binary_attachment(vec![0, 1, 2, 3, 4])
            .build();

        assert!(solmsg.is_delivery_to_one());
        assert_eq!(solmsg.get_topic().unwrap(), "TIC/v1/test");
        assert_eq!(solmsg.get_correlation_id().unwrap(), "R1");
        assert_eq!(solmsg.get_class_of_service().unwrap(), 1);
        assert_eq!(solmsg.get_user_prop("ct").unwrap(), "bytes/msgpack");
        assert_eq!(solmsg.get_binary_attachment().unwrap(), vec![0, 1, 2, 3, 4]);
    }
}
