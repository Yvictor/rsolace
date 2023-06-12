use std::ffi::CString;

pub trait ConvertToCString {
    fn to_cstring(&self) -> CString;
}

impl ConvertToCString for bool {
    fn to_cstring(&self) -> CString {
        CString::new(format!("{}", *self as i32)).unwrap()
    }
}

impl ConvertToCString for u32 {
    fn to_cstring(&self) -> CString {
        CString::new(format!("{}", *self)).unwrap()
    }
}

impl ConvertToCString for i32 {
    fn to_cstring(&self) -> CString {
        CString::new(format!("{}", *self)).unwrap()
    }
}

impl ConvertToCString for &str {
    fn to_cstring(&self) -> CString {
        CString::new(*self).unwrap()
    }
}

pub fn prop2cstr<T: ConvertToCString>(prop: T) -> CString {
    prop.to_cstring()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prop2cstr_bool() {
        let res = prop2cstr(true);
        assert_eq!(res, CString::new("1").unwrap());
    }

    #[test]
    fn test_prop2cstr_u32() {
        let res = prop2cstr(3000);
        assert_eq!(res, CString::new("3000").unwrap());
    }

    #[test]
    fn test_prop2cstr_str() {
        let res = prop2cstr("192.168.0.1");
        assert_eq!(res, CString::new("192.168.0.1").unwrap());
    }
}
