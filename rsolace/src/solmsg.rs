use super::types::{SolClientDeliveryMode, SolClientDestType, SolClientReturnCode};
use enum_primitive::FromPrimitive;
use failure::{bail, Error};
use std::ffi::{c_void, CStr, CString};
// use std::option::Option;
use chrono::DateTime;
use std::ptr::null_mut;

pub struct SolMsg {
    msg_p: rsolace_sys::solClient_opaqueMsg_pt,
    // container_p: Option<rsolace_sys::solClient_opaqueContainer_pt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Destination {
    pub dest_type: SolClientDestType,
    pub dest: String,
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
    pub fn new() -> Result<SolMsg, Error> {
        let mut msg_p: rsolace_sys::solClient_opaqueMsg_pt = null_mut();
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_alloc(&mut msg_p);
            if rt_code != (SolClientReturnCode::Ok as i32) {
                bail!("solmsg msg alloc faile");
            }
        }
        Ok(SolMsg {
            msg_p: msg_p,
            // container_p: None,
        })
    }

    pub fn from_ptr(msg_p: rsolace_sys::solClient_opaqueMsg_pt) -> Result<SolMsg, Error> {
        // TODO how to check the ptr is valid
        let mut mode = 0;
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_getDeliveryMode(msg_p, &mut mode);
            if rt_code != (SolClientReturnCode::Ok as i32) {
                bail!("not valid msg ptr");
            }
        }
        Ok(SolMsg { msg_p: msg_p })
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

    pub fn get_delivery_mode(&self) -> Result<SolClientDeliveryMode, Error> {
        let mut mode = 0;
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_getDeliveryMode(self.msg_p, &mut mode);
            if rt_code != (SolClientReturnCode::Ok as i32) {
                bail!("get delivery mode error");
            }
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

    pub fn get_destination(&self) -> Result<Destination, Error> {
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
            if rt_code != (SolClientReturnCode::Ok as i32) {
                bail!("get delivery mode error");
            }
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

    pub fn get_reply_to(&self) -> Result<Destination, Error> {
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
            if rt_code != (SolClientReturnCode::Ok as i32) {
                bail!("get delivery mode error");
            }
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

    pub fn get_correlation_id(&self) -> Result<String, Error> {
        unsafe {
            let mut corr_id: *const std::os::raw::c_char = null_mut();
            let rt_code = rsolace_sys::solClient_msg_getCorrelationId(self.msg_p, &mut corr_id);
            if rt_code != (SolClientReturnCode::Ok as i32) {
                bail!("get corr error");
            }
            match CStr::from_ptr(corr_id).to_str() {
                Ok(corr_id) => Ok(corr_id.to_string()),
                Err(_) => {
                    bail!("Utf8Error");
                }
            }
        }
    }

    pub fn set_class_of_service(&mut self, cos: u32) -> SolClientReturnCode {
        SolClientReturnCode::from_i32(unsafe {
            rsolace_sys::solClient_msg_setClassOfService(self.msg_p, cos - 1)
        })
        .unwrap()
    }

    pub fn get_class_of_service(&self) -> Result<u32, Error> {
        let mut cos = 0;
        let rt_code = unsafe { rsolace_sys::solClient_msg_getClassOfService(self.msg_p, &mut cos) };
        if rt_code != SolClientReturnCode::Ok as i32 {
            bail!("get msg cos faile");
        }
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

    pub fn get_topic(&self) -> Result<String, Error> {
        let dest = self.get_destination();
        match dest {
            Ok(dest) => Ok(dest.dest),
            Err(e) => {
                bail!(e)
            }
        }
    }

    pub fn set_reply_topic(&mut self, topic: &str) -> SolClientReturnCode {
        let dest = Destination::new(SolClientDestType::Topic, topic);
        self.set_reply_to(&dest)
    }

    pub fn get_reply_topic(&self) -> Result<String, Error> {
        let dest = self.get_reply_to();
        match dest {
            Ok(dest) => Ok(dest.dest),
            Err(e) => {
                bail!(e)
            }
        }
    }

    pub fn get_sender_time(&self) -> Result<DateTime<chrono::Utc>, Error> {
        let mut ts = 0;
        let rt_code = unsafe { rsolace_sys::solClient_msg_getSenderTimestamp(self.msg_p, &mut ts) };
        if rt_code != SolClientReturnCode::Ok as i32 {
            bail!("get msg sender time faile");
        }
        match chrono::naive::NaiveDateTime::from_timestamp_millis(ts) {
            Some(naive_datetime) => Ok(DateTime::from_utc(naive_datetime, chrono::Utc)),
            None => {
                bail!("get msg sender time faile");
            }
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

    pub fn get_binary_attachment(&self) -> Result<Vec<u8>, Error> {
        let mut data_ptr = null_mut();
        let mut data_len = 0;
        unsafe {
            let rt_code = rsolace_sys::solClient_msg_getBinaryAttachmentPtr(
                self.msg_p,
                &mut data_ptr,
                &mut data_len,
            );
            if rt_code != (SolClientReturnCode::Ok as i32) {
                bail!("solmsg get binary attachment faile");
            }

            // assert!(!data_ptr.is_null());
            if data_len <= 0 {
                bail!("solmsg get binary attachment empty");
            }
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
        unsafe {
            rsolace_sys::solClient_msg_free(&mut self.msg_p);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;

    use crate::types::{SolClientDeliveryMode, SolClientDestType, SolClientReturnCode};

    use super::{Destination, SolMsg};

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
    fn solmsg_set_binary_attachment(mut solmsg: SolMsg) {
        let data = vec![0, 1, 2, 3, 4];
        let rt_code = solmsg.set_binary_attachment(&data);
        assert_eq!(rt_code, SolClientReturnCode::Ok)
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
