use super::types::{SolClientDeliveryMode, SolClientDestType, SolClientReturnCode};
use enum_primitive::FromPrimitive;
use std::ffi::{c_void, CStr, CString};
// use std::option::Option;
use chrono::DateTime;
use snafu::prelude::{ensure, Snafu};
use snafu::{OptionExt, ResultExt};
use std::ptr::null_mut;

pub struct SolMsg {
    msg_p: rsolace_sys::solClient_opaqueMsg_pt,
    user_prop_p: Option<rsolace_sys::solClient_opaqueContainer_pt>,
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
            dest_type: dest_type,
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
            msg_p: msg_p,
            user_prop_p: None,
            // container_p: None,
        })
    }

    pub fn from_ptr(msg_p: rsolace_sys::solClient_opaqueMsg_pt) -> Result<SolMsg, SolMsgError> {
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
                msg_p: msg_p,
                user_prop_p: Some(user_prop_p),
            }),
            _ => {
                Ok(SolMsg {
                    msg_p: msg_p,
                    user_prop_p: None, //Some(user_prop_p)
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
            Ok(topic) => match &topic[..4] {
                "#P2P" => true,
                _ => false,
            },
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

    pub fn is_delivery_to_one(&mut self) -> bool {
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

    pub fn get_sender_time(&self) -> Result<DateTime<chrono::Utc>, SolMsgError> {
        let mut ts = 0;
        let rt_code = unsafe { rsolace_sys::solClient_msg_getSenderTimestamp(self.msg_p, &mut ts) };
        ensure!(
            rt_code == SolClientReturnCode::Ok as i32,
            GetAttrSnafu {
                attr: "sender_time"
            }
        );
        let naive_datetime =
            chrono::naive::NaiveDateTime::from_timestamp_millis(ts).context(GetAttrEmptySnafu {
                attr: "sender_time",
            })?;
        Ok(DateTime::from_utc(naive_datetime, chrono::Utc))
    }

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

    pub fn get_binary_attachment(&self) -> Result<Vec<u8>, SolMsgError> {
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
            let v: Vec<u8> =
                std::slice::from_raw_parts(data_ptr as *const u8, data_len as usize).to_vec();
            Ok(v)
        }
    }

    pub fn dump(&self, display_only: bool) -> Option<String> {
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
                let dump = CStr::from_ptr(&buffer_p as *const i8).to_str().unwrap();
                Some(dump.to_string())
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
            &self.dump(false).unwrap_or("".to_string())
        )
    }
}

impl Drop for SolMsg {
    fn drop(&mut self) {
        // tracing::debug!("solmsg: {:?} drop call", self.msg_p);
        unsafe {
            rsolace_sys::solClient_msg_free(&mut self.msg_p);
        }
    }
}

unsafe impl Send for SolMsg {}

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;

    use crate::types::{SolClientDeliveryMode, SolClientDestType, SolClientReturnCode};

    use super::{Destination, SolMsg, SolMsgError};

    use rstest::{fixture, rstest};

    #[fixture]
    pub fn solmsg() -> SolMsg {
        SolMsg::new().unwrap()
    }

    #[test]
    fn solmsg_from_ptr() {
        let res = SolMsg::from_ptr(null_mut());
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
    #[case(1)]
    #[case(2)]
    #[case(3)]
    fn solmsg_cos_workable(mut solmsg: SolMsg, #[case] cos: u32) {
        solmsg.set_class_of_service(cos);
        assert_eq!(solmsg.get_class_of_service().unwrap(), cos);
    }

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
}
