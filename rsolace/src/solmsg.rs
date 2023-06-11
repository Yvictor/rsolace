use super::types::SolClientReturnCode;
use enum_primitive::FromPrimitive;
use failure::{bail, Error};
use rsolace_sys::{self, solClient_msg_alloc};
use std::ffi::c_void;
// use std::option::Option;
use std::ptr::null_mut;

pub struct SolMsg {
    msg_p: rsolace_sys::solClient_opaqueMsg_pt,
    // container_p: Option<rsolace_sys::solClient_opaqueContainer_pt>,
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
        Ok(SolMsg { msg_p: msg_p })
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
    use crate::types::SolClientReturnCode;

    use super::SolMsg;

    #[test]
    fn new_solmsg() {
        SolMsg::new().unwrap();
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
