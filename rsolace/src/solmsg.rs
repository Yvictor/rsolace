use super::types::{SolClientDeliveryMode, SolClientDestType, SolClientReturnCode};
use enum_primitive::FromPrimitive;
use failure::{bail, Error};
use rsolace_sys::{self, solClient_msg_alloc, solClient_msg_dump};
use std::ffi::{c_void, CStr, CString};
// use std::option::Option;
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
            let rt_code = solClient_msg_alloc(&mut msg_p);
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
                solClient_msg_dump(self.msg_p, null_mut(), 0);
                None
            }
        } else {
            None
        }
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

    #[test]
    fn new_solmsg() {
        SolMsg::new().unwrap();
    }

    #[test]
    fn solmsg_from_ptr() {
        let res = SolMsg::from_ptr(null_mut());
        assert!(res.is_err())
    }

    #[test]
    fn solmsg_delivery_mode_workable() {
        let mut solmsg = SolMsg::new().unwrap();
        assert_eq!(
            solmsg.get_delivery_mode().unwrap(),
            SolClientDeliveryMode::Direct
        );
        let mode = SolClientDeliveryMode::Direct;
        solmsg.set_delivery_mode(mode);
        assert_eq!(solmsg.get_delivery_mode().unwrap(), mode);

        let mode = SolClientDeliveryMode::Persistent;
        solmsg.set_delivery_mode(mode);
        assert_eq!(solmsg.get_delivery_mode().unwrap(), mode);

        let mode = SolClientDeliveryMode::NonPersistent;
        solmsg.set_delivery_mode(mode);
        assert_eq!(solmsg.get_delivery_mode().unwrap(), mode);
    }

    #[test]
    fn solmsg_dest_workable() {
        let mut solmsg = SolMsg::new().unwrap();
        let dest = Destination::new(SolClientDestType::Topic, "TIC/v1/test");
        solmsg.set_destination(&dest);
        assert_eq!(solmsg.get_destination().unwrap(), dest);

        let dest = Destination::new(SolClientDestType::Queue, "TIC/v1/test");
        solmsg.set_destination(&dest);
        assert_eq!(solmsg.get_destination().unwrap(), dest);
    }

    #[test]
    fn solmsg_topic_workable() {
        let mut solmsg = SolMsg::new().unwrap();
        let topic = "TIC/v1/test";
        solmsg.set_topic(topic);
        assert_eq!(solmsg.get_topic().unwrap(), topic);
    }

    #[test]
    fn solmsg_set_binary_attachment() {
        let mut solmsg = SolMsg::new().unwrap();
        let data = vec![0, 1, 2, 3, 4];
        let rt_code = solmsg.set_binary_attachment(&data);
        assert_eq!(rt_code, SolClientReturnCode::Ok)
    }

    #[test]
    fn solmsg_get_binary_attachment() {
        let mut solmsg = SolMsg::new().unwrap();
        let data = vec![0, 1, 2, 3, 4];
        solmsg.set_binary_attachment(&data);
        // assert_eq!(rt_code, SolClientReturnCode::Ok)
        let res = solmsg.get_binary_attachment().unwrap();
        assert_eq!(res, data)
    }
}
