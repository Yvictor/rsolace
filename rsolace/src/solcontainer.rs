use super::types::SolClientReturnCode;
use enum_primitive::FromPrimitive;
use snafu::prelude::{ensure, Snafu};
use snafu::ResultExt;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null_mut;

#[derive(Debug, Clone, PartialEq)]
pub enum ContainerType {
    Map,
    Stream,
}

#[derive(Debug)]
pub enum ContainerFieldType {
    Null,
    Boolean(bool),
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Uint64(u64),
    Int64(i64),
    Char(char),
    Wchar(char),
    Float(f32),
    Double(f64),
    String(String),
    ByteArray(Vec<u8>),
    Container(SolContainer),
    Smf(Vec<u8>),
    Destination(String), // Simplified destination representation
}

#[derive(Debug)]
pub struct ContainerField {
    pub name: Option<String>,
    pub field_type: ContainerFieldType,
}

pub struct SolContainer {
    container_p: rsolace_sys::solClient_opaqueContainer_pt,
    container_type: ContainerType,
    buffer: Vec<u8>,
}

#[derive(Debug, Snafu, PartialEq)]
pub enum SolContainerError {
    #[snafu(display("SolContainer create {container_type:?} Error: {code}"))]
    Create {
        container_type: ContainerType,
        code: SolClientReturnCode,
    },
    #[snafu(display("SolContainer operation '{operation}' Error: {code}"))]
    Operation {
        operation: String,
        code: SolClientReturnCode,
    },
    #[snafu(display("SolContainer get field '{field}' Error: {code}"))]
    GetField {
        field: String,
        code: SolClientReturnCode,
    },
    #[snafu(display("SolContainer get field '{field}' UTF8 Error"))]
    GetFieldUtf8Error {
        source: std::str::Utf8Error,
        field: String,
    },
    #[snafu(display("SolContainer buffer size {size} too small"))]
    BufferTooSmall { size: usize },
    #[snafu(display("SolContainer field '{field}' not found"))]
    FieldNotFound { field: String },
    #[snafu(display("SolContainer conversion error: {message}"))]
    ConversionError { message: String },
}

impl SolContainer {
    /// Create a new Map container with specified buffer size
    pub fn create_map(size: usize) -> Result<SolContainer, SolContainerError> {
        // 確保 Solace 庫已初始化
        crate::ensure_solace_initialized();

        if size == 0 {
            return Err(SolContainerError::BufferTooSmall { size });
        }

        let mut buffer = vec![0u8; size];
        let mut container_p: rsolace_sys::solClient_opaqueContainer_pt = null_mut();

        unsafe {
            let rt_code = rsolace_sys::solClient_container_createMap(
                &mut container_p,
                buffer.as_mut_ptr() as *mut c_char,
                size,
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                CreateSnafu {
                    container_type: ContainerType::Map,
                    code
                }
            );
        }

        Ok(SolContainer {
            container_p,
            container_type: ContainerType::Map,
            buffer,
        })
    }

    /// Create a new Stream container with specified buffer size
    pub fn create_stream(size: usize) -> Result<SolContainer, SolContainerError> {
        // 確保 Solace 庫已初始化
        crate::ensure_solace_initialized();

        if size == 0 {
            return Err(SolContainerError::BufferTooSmall { size });
        }

        let mut buffer = vec![0u8; size];
        let mut container_p: rsolace_sys::solClient_opaqueContainer_pt = null_mut();

        unsafe {
            let rt_code = rsolace_sys::solClient_container_createStream(
                &mut container_p,
                buffer.as_mut_ptr() as *mut c_char,
                size,
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                CreateSnafu {
                    container_type: ContainerType::Stream,
                    code
                }
            );
        }

        Ok(SolContainer {
            container_p,
            container_type: ContainerType::Stream,
            buffer,
        })
    }

    /// Get the raw container pointer (for internal use)
    pub fn get_ptr(&self) -> rsolace_sys::solClient_opaqueContainer_pt {
        self.container_p
    }

    /// Get the container type
    pub fn get_type(&self) -> &ContainerType {
        &self.container_type
    }

    /// Get the buffer reference for serialization
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Create a SolContainer from raw parts (for internal use by SolMsg)
    pub fn from_raw_parts(
        container_p: rsolace_sys::solClient_opaqueContainer_pt,
        container_type: ContainerType,
        buffer: Vec<u8>
    ) -> SolContainer {
        SolContainer {
            container_p,
            container_type,
            buffer,
        }
    }

    /// Create a Map container from existing SDT buffer data
    pub fn from_map_buffer(data: &[u8]) -> Result<SolContainer, SolContainerError> {
        let buffer_size = std::cmp::max(data.len(), 1024);
        let mut buffer = vec![0u8; buffer_size];
        
        if data.len() > buffer_size {
            return Err(SolContainerError::BufferTooSmall { size: data.len() });
        }
        
        // Copy the SDT data into the buffer
        buffer[..data.len()].copy_from_slice(data);
        
        let mut container_p: rsolace_sys::solClient_opaqueContainer_pt = null_mut();
        
        unsafe {
            let rt_code = rsolace_sys::solClient_container_createMap(
                &mut container_p,
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                CreateSnafu {
                    container_type: ContainerType::Map,
                    code
                }
            );

            Ok(SolContainer {
                container_p,
                container_type: ContainerType::Map,
                buffer,
            })
        }
    }

    /// Create a Stream container from existing SDT buffer data
    pub fn from_stream_buffer(data: &[u8]) -> Result<SolContainer, SolContainerError> {
        let buffer_size = std::cmp::max(data.len(), 1024);
        let mut buffer = vec![0u8; buffer_size];
        
        if data.len() > buffer_size {
            return Err(SolContainerError::BufferTooSmall { size: data.len() });
        }
        
        // Copy the SDT data into the buffer
        buffer[..data.len()].copy_from_slice(data);
        
        let mut container_p: rsolace_sys::solClient_opaqueContainer_pt = null_mut();
        
        unsafe {
            let rt_code = rsolace_sys::solClient_container_createStream(
                &mut container_p,
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                CreateSnafu {
                    container_type: ContainerType::Stream,
                    code
                }
            );

            Ok(SolContainer {
                container_p,
                container_type: ContainerType::Stream,
                buffer,
            })
        }
    }

    /// Serialize this container to bytes using SolMsg
    pub fn to_bytes(&self) -> Result<Vec<u8>, SolContainerError> {
        use crate::solmsg::SolMsg;
        
        // Create a temporary SolMsg
        let mut msg = SolMsg::new()
            .map_err(|_| SolContainerError::ConversionError { 
                message: "Failed to create temporary SolMsg".to_string() 
            })?;
        
        // Set this container as the binary attachment
        let result = msg.set_binary_attachment_container(self);
        if result != SolClientReturnCode::Ok {
            return Err(SolContainerError::ConversionError { 
                message: format!("Failed to set container as binary attachment: {:?}", result)
            });
        }
        
        // Get the binary attachment data
        let data = msg.get_binary_attachment()
            .map_err(|e| SolContainerError::ConversionError { 
                message: format!("Failed to get binary attachment: {}", e)
            })?;
        
        Ok(data.into_owned())
    }

    /// Deserialize bytes to container using SolMsg
    pub fn from_bytes(data: &[u8]) -> Result<SolContainer, SolContainerError> {
        use crate::solmsg::SolMsg;
        
        // Create a temporary SolMsg  
        let mut msg = SolMsg::new()
            .map_err(|_| SolContainerError::ConversionError { 
                message: "Failed to create temporary SolMsg".to_string() 
            })?;
        
        // Set the binary attachment data
        let result = msg.set_binary_attachment(data);
        if result != SolClientReturnCode::Ok {
            return Err(SolContainerError::ConversionError { 
                message: format!("Failed to set binary attachment: {:?}", result)
            });
        }
        
        // Try to extract as Map first, then Stream if that fails
        if let Ok(container) = msg.get_binary_attachment_map() {
            return Ok(container);
        }
        
        if let Ok(container) = msg.get_binary_attachment_stream() {
            return Ok(container);
        }
        
        Err(SolContainerError::ConversionError { 
            message: "Failed to extract container from message - data may not be valid SDT format".to_string()
        })
    }

    /// Add a null field to the container
    pub fn add_null(&mut self, name: Option<&str>) -> SolClientReturnCode {
        let name_c = name.map(|n| CString::new(n).unwrap());
        let name_ptr = match &name_c {
            Some(c) => c.as_ptr(),
            None => null_mut(),
        };

        unsafe {
            let rt_code = rsolace_sys::solClient_container_addNull(self.container_p, name_ptr);
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Add a boolean field to the container
    pub fn add_boolean(&mut self, value: bool, name: Option<&str>) -> SolClientReturnCode {
        let name_c = name.map(|n| CString::new(n).unwrap());
        let name_ptr = match &name_c {
            Some(c) => c.as_ptr(),
            None => null_mut(),
        };

        unsafe {
            let rt_code = rsolace_sys::solClient_container_addBoolean(
                self.container_p,
                value as u8,
                name_ptr,
            );
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Add a string field to the container
    pub fn add_string(&mut self, value: &str, name: Option<&str>) -> SolClientReturnCode {
        let value_c = CString::new(value).unwrap();
        let name_c = name.map(|n| CString::new(n).unwrap());
        let name_ptr = match &name_c {
            Some(c) => c.as_ptr(),
            None => null_mut(),
        };

        unsafe {
            let rt_code = rsolace_sys::solClient_container_addString(
                self.container_p,
                value_c.as_ptr(),
                name_ptr,
            );
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Add an int32 field to the container
    pub fn add_int32(&mut self, value: i32, name: Option<&str>) -> SolClientReturnCode {
        let name_c = name.map(|n| CString::new(n).unwrap());
        let name_ptr = match &name_c {
            Some(c) => c.as_ptr(),
            None => null_mut(),
        };

        unsafe {
            let rt_code =
                rsolace_sys::solClient_container_addInt32(self.container_p, value, name_ptr);
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Add an int64 field to the container
    pub fn add_int64(&mut self, value: i64, name: Option<&str>) -> SolClientReturnCode {
        let name_c = name.map(|n| CString::new(n).unwrap());
        let name_ptr = match &name_c {
            Some(c) => c.as_ptr(),
            None => null_mut(),
        };

        unsafe {
            let rt_code =
                rsolace_sys::solClient_container_addInt64(self.container_p, value, name_ptr);
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Add a double field to the container
    pub fn add_double(&mut self, value: f64, name: Option<&str>) -> SolClientReturnCode {
        let name_c = name.map(|n| CString::new(n).unwrap());
        let name_ptr = match &name_c {
            Some(c) => c.as_ptr(),
            None => null_mut(),
        };

        unsafe {
            let rt_code =
                rsolace_sys::solClient_container_addDouble(self.container_p, value, name_ptr);
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Add a byte array field to the container
    pub fn add_byte_array(&mut self, data: &[u8], name: Option<&str>) -> SolClientReturnCode {
        let name_c = name.map(|n| CString::new(n).unwrap());
        let name_ptr = match &name_c {
            Some(c) => c.as_ptr(),
            None => null_mut(),
        };

        unsafe {
            let rt_code = rsolace_sys::solClient_container_addByteArray(
                self.container_p,
                data.as_ptr(),
                data.len() as u32,
                name_ptr,
            );
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Add another container as a sub-container
    pub fn add_container(
        &mut self,
        container: &SolContainer,
        name: Option<&str>,
    ) -> SolClientReturnCode {
        let name_c = name.map(|n| CString::new(n).unwrap());
        let name_ptr = match &name_c {
            Some(c) => c.as_ptr(),
            None => null_mut(),
        };

        unsafe {
            let rt_code = rsolace_sys::solClient_container_addContainer(
                self.container_p,
                container.container_p,
                name_ptr,
            );
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Get a string field from the container
    pub fn get_string(&self, name: &str) -> Result<String, SolContainerError> {
        let name_c = CString::new(name).unwrap();
        let mut value_ptr: *const c_char = null_mut();

        unsafe {
            let rt_code = rsolace_sys::solClient_container_getStringPtr(
                self.container_p,
                &mut value_ptr,
                name_c.as_ptr(),
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetFieldSnafu {
                    field: name.to_string(),
                    code
                }
            );

            let value = CStr::from_ptr(value_ptr)
                .to_str()
                .context(GetFieldUtf8Snafu {
                    field: name.to_string(),
                })?;
            Ok(value.to_string())
        }
    }

    /// Get an int32 field from the container
    pub fn get_int32(&self, name: &str) -> Result<i32, SolContainerError> {
        let name_c = CString::new(name).unwrap();
        let mut value: i32 = 0;

        unsafe {
            let rt_code = rsolace_sys::solClient_container_getInt32(
                self.container_p,
                &mut value,
                name_c.as_ptr(),
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetFieldSnafu {
                    field: name.to_string(),
                    code
                }
            );

            Ok(value)
        }
    }

    /// Get an int64 field from the container
    pub fn get_int64(&self, name: &str) -> Result<i64, SolContainerError> {
        let name_c = CString::new(name).unwrap();
        let mut value: i64 = 0;

        unsafe {
            let rt_code = rsolace_sys::solClient_container_getInt64(
                self.container_p,
                &mut value,
                name_c.as_ptr(),
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetFieldSnafu {
                    field: name.to_string(),
                    code
                }
            );

            Ok(value)
        }
    }

    /// Get a double field from the container
    pub fn get_double(&self, name: &str) -> Result<f64, SolContainerError> {
        let name_c = CString::new(name).unwrap();
        let mut value: f64 = 0.0;

        unsafe {
            let rt_code = rsolace_sys::solClient_container_getDouble(
                self.container_p,
                &mut value,
                name_c.as_ptr(),
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetFieldSnafu {
                    field: name.to_string(),
                    code
                }
            );

            Ok(value)
        }
    }

    /// Get a boolean field from the container
    pub fn get_boolean(&self, name: &str) -> Result<bool, SolContainerError> {
        let name_c = CString::new(name).unwrap();
        let mut value: u8 = 0;

        unsafe {
            let rt_code = rsolace_sys::solClient_container_getBoolean(
                self.container_p,
                &mut value,
                name_c.as_ptr(),
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetFieldSnafu {
                    field: name.to_string(),
                    code
                }
            );

            Ok(value != 0)
        }
    }

    /// Get a byte array field from the container
    pub fn get_byte_array(&self, name: &str) -> Result<Vec<u8>, SolContainerError> {
        let name_c = CString::new(name).unwrap();
        let mut data_ptr: *mut u8 = null_mut();
        let mut data_len: u32 = 0;

        unsafe {
            let rt_code = rsolace_sys::solClient_container_getByteArrayPtr(
                self.container_p,
                &mut data_ptr,
                &mut data_len,
                name_c.as_ptr(),
            );

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                GetFieldSnafu {
                    field: name.to_string(),
                    code
                }
            );

            let data_slice = std::slice::from_raw_parts(data_ptr as *const u8, data_len as usize);
            Ok(data_slice.to_vec())
        }
    }

    /// Delete a field from the container
    pub fn delete_field(&mut self, name: &str) -> SolClientReturnCode {
        let name_c = CString::new(name).unwrap();

        unsafe {
            let rt_code =
                rsolace_sys::solClient_container_deleteField(self.container_p, name_c.as_ptr());
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Rewind the container iterator to the beginning
    pub fn rewind(&mut self) -> SolClientReturnCode {
        unsafe {
            let rt_code = rsolace_sys::solClient_container_rewind(self.container_p);
            SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
        }
    }

    /// Check if there is a next field in the container
    pub fn has_next_field(&self) -> bool {
        unsafe { rsolace_sys::solClient_container_hasNextField(self.container_p) == 1 }
    }

    /// Get the number of fields in the container
    pub fn get_size(&self) -> Result<u32, SolContainerError> {
        let mut size: usize = 0;

        unsafe {
            let rt_code = rsolace_sys::solClient_container_getSize(self.container_p, &mut size);

            let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
            ensure!(
                rt_code == (SolClientReturnCode::Ok as i32),
                OperationSnafu {
                    operation: "get_size".to_string(),
                    code
                }
            );

            Ok(size as u32)
        }
    }

    /// Get the next field from the container during iteration
    pub fn get_next_field(&mut self) -> Option<ContainerField> {
        use std::mem;
        use std::ffi::CStr;
        
        unsafe {
            let mut field: rsolace_sys::solClient_field_t = mem::zeroed();
            let mut name_ptr: *const c_char = null_mut();
            
            let result = rsolace_sys::solClient_container_getNextField(
                self.container_p,
                &mut field,
                mem::size_of::<rsolace_sys::solClient_field_t>(),
                &mut name_ptr,
            );
            
            if result != (SolClientReturnCode::Ok as i32) {
                return None;
            }
            
            // Get field name if available
            let field_name = if name_ptr.is_null() {
                None
            } else {
                CStr::from_ptr(name_ptr).to_str().ok().map(|s| s.to_string())
            };
            
            // Convert field value based on type
            let field_type = match field.type_ {
                rsolace_sys::solClient_fieldType_SOLCLIENT_NULL => ContainerFieldType::Null,
                rsolace_sys::solClient_fieldType_SOLCLIENT_BOOL => {
                    ContainerFieldType::Boolean(field.value.boolean != 0)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_UINT8 => {
                    ContainerFieldType::Uint8(field.value.uint8)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_INT8 => {
                    ContainerFieldType::Int8(field.value.int8)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_UINT16 => {
                    ContainerFieldType::Uint16(field.value.uint16)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_INT16 => {
                    ContainerFieldType::Int16(field.value.int16)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_UINT32 => {
                    ContainerFieldType::Uint32(field.value.uint32)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_INT32 => {
                    ContainerFieldType::Int32(field.value.int32)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_UINT64 => {
                    ContainerFieldType::Uint64(field.value.uint64)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_INT64 => {
                    ContainerFieldType::Int64(field.value.int64)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_FLOAT => {
                    ContainerFieldType::Float(field.value.float32)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_DOUBLE => {
                    ContainerFieldType::Double(field.value.float64)
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_STRING => {
                    if !field.value.string.is_null() {
                        let string_val = CStr::from_ptr(field.value.string)
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                        ContainerFieldType::String(string_val)
                    } else {
                        ContainerFieldType::String(String::new())
                    }
                }
                rsolace_sys::solClient_fieldType_SOLCLIENT_BYTEARRAY => {
                    if !field.value.bytearray.is_null() && field.length > 0 {
                        let byte_slice = std::slice::from_raw_parts(
                            field.value.bytearray as *const u8,
                            field.length as usize
                        );
                        ContainerFieldType::ByteArray(byte_slice.to_vec())
                    } else {
                        ContainerFieldType::ByteArray(Vec::new())
                    }
                }
                // For now, handle containers as unknown - we'll need recursive parsing
                _ => return None,
            };
            
            Some(ContainerField {
                name: field_name,
                field_type,
            })
        }
    }

    /// Iterate through all fields in the container, collecting them into a vector
    pub fn get_all_fields(&mut self) -> Result<Vec<ContainerField>, SolContainerError> {
        let mut fields = Vec::new();
        
        // Rewind to start
        self.rewind();
        
        // Collect all fields
        while let Some(field) = self.get_next_field() {
            fields.push(field);
        }
        
        Ok(fields)
    }

    /// Close the container (automatically called on drop)
    pub fn close(&mut self) -> SolClientReturnCode {
        if !self.container_p.is_null() {
            unsafe {
                let rt_code = rsolace_sys::solClient_container_closeMapStream(&mut self.container_p);
                SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail)
            }
        } else {
            SolClientReturnCode::Ok
        }
    }
}

impl Drop for SolContainer {
    fn drop(&mut self) {
        self.close();
    }
}

unsafe impl Send for SolContainer {}
unsafe impl Sync for SolContainer {}

impl std::fmt::Debug for SolContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolContainer")
            .field("container_p", &self.container_p)
            .field("container_type", &self.container_type)
            .field("buffer_size", &self.buffer.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_creation() {
        let map_container = SolContainer::create_map(1024);
        assert!(map_container.is_ok());
        let map = map_container.unwrap();
        assert_eq!(*map.get_type(), ContainerType::Map);

        let stream_container = SolContainer::create_stream(1024);
        assert!(stream_container.is_ok());
        let stream = stream_container.unwrap();
        assert_eq!(*stream.get_type(), ContainerType::Stream);
    }

    #[test]
    fn test_buffer_too_small() {
        let result = SolContainer::create_map(0);
        assert!(matches!(result, Err(SolContainerError::BufferTooSmall { .. })));
    }

    #[test]
    fn test_map_operations() {
        let mut map = SolContainer::create_map(1024).unwrap();
        
        // Add various types of data
        assert_eq!(map.add_string("hello", Some("greeting")), SolClientReturnCode::Ok);
        assert_eq!(map.add_int32(42, Some("answer")), SolClientReturnCode::Ok);
        assert_eq!(map.add_double(3.14159, Some("pi")), SolClientReturnCode::Ok);
        assert_eq!(map.add_boolean(true, Some("flag")), SolClientReturnCode::Ok);

        // Read back the data
        assert_eq!(map.get_string("greeting").unwrap(), "hello");
        assert_eq!(map.get_int32("answer").unwrap(), 42);
        assert!((map.get_double("pi").unwrap() - 3.14159).abs() < 0.0001);
        assert_eq!(map.get_boolean("flag").unwrap(), true);
    }

    #[test]
    fn test_stream_operations() {
        let mut stream = SolContainer::create_stream(1024).unwrap();
        
        // Add data to stream (no names for stream elements)
        assert_eq!(stream.add_string("first", None), SolClientReturnCode::Ok);
        assert_eq!(stream.add_int32(123, None), SolClientReturnCode::Ok);
        assert_eq!(stream.add_double(2.718, None), SolClientReturnCode::Ok);
    }

    #[test]
    fn test_container_management() {
        let mut map = SolContainer::create_map(1024).unwrap();
        
        // Add and then delete a field
        assert_eq!(map.add_string("temp", Some("temporary")), SolClientReturnCode::Ok);
        assert_eq!(map.delete_field("temporary"), SolClientReturnCode::Ok);
        
        // Should not be able to get deleted field
        assert!(map.get_string("temporary").is_err());
    }

    #[test]
    fn test_container_serialization_basic() {
        // Test that we can at least serialize containers
        let mut container = SolContainer::create_map(1024).unwrap();
        
        // Add test data
        assert_eq!(container.add_string("test_value", Some("test_key")), SolClientReturnCode::Ok);
        assert_eq!(container.add_int32(123, Some("number")), SolClientReturnCode::Ok);
        assert_eq!(container.add_boolean(true, Some("active")), SolClientReturnCode::Ok);
        
        // Verify original data can be read
        assert_eq!(container.get_string("test_key").unwrap(), "test_value");
        assert_eq!(container.get_int32("number").unwrap(), 123);
        assert_eq!(container.get_boolean("active").unwrap(), true);
        
        // Test serialization
        let serialized_bytes = container.to_bytes().unwrap();
        println!("Successfully serialized container to {} bytes", serialized_bytes.len());
        assert!(serialized_bytes.len() > 0);
        
        // For now, we know deserialization has issues with buffer management
        // Let's focus on getting the Python interface working first
    }

    #[test] 
    fn test_container_serialization_investigation() {
        // This test investigates the deserialization issue
        let mut original_container = SolContainer::create_map(1024).unwrap();
        
        // Add simple test data
        assert_eq!(original_container.add_string("hello", Some("greeting")), SolClientReturnCode::Ok);
        
        // Serialize
        let serialized_bytes = original_container.to_bytes().unwrap();
        println!("Serialization test:");
        println!("  Original container size: {:?}", original_container.get_size());
        println!("  Serialized {} bytes", serialized_bytes.len());
        
        // Try deserialization (knowing it may fail)
        match SolContainer::from_bytes(&serialized_bytes) {
            Ok(restored_container) => {
                println!("  Deserialization succeeded!");
                println!("  Restored type: {:?}", restored_container.get_type());
                match restored_container.get_size() {
                    Ok(size) => println!("  Restored size: {}", size),
                    Err(e) => println!("  Restored size error: {:?}", e),
                }
                match restored_container.get_string("greeting") {
                    Ok(val) => println!("  Restored greeting: {}", val),
                    Err(e) => println!("  Restored greeting error: {:?}", e),
                }
            }
            Err(e) => {
                println!("  Deserialization failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_container_stream_roundtrip() {
        // Create and populate a Stream container
        let mut original_container = SolContainer::create_stream(1024).unwrap();
        
        // Add test data (streams don't use field names)
        let result = original_container.add_string("first", None);
        assert_eq!(result, SolClientReturnCode::Ok);
        
        let result = original_container.add_int32(456, None);
        assert_eq!(result, SolClientReturnCode::Ok);
        
        // Serialize to bytes
        let serialized_bytes = original_container.to_bytes().unwrap();
        println!("Stream serialized {} bytes", serialized_bytes.len());
        
        // Deserialize back to container
        let restored_container = SolContainer::from_bytes(&serialized_bytes).unwrap();
        
        // Verify container type
        assert_eq!(restored_container.get_type(), &ContainerType::Stream);
        
        // Note: Stream field access would require iteration methods not yet implemented
        // For now, just verify we can deserialize without errors
    }

    #[test]
    fn test_container_field_iteration() {
        // Create and populate a Map container
        let mut container = SolContainer::create_map(1024).unwrap();
        
        // Add test data
        assert_eq!(container.add_string("hello", Some("greeting")), SolClientReturnCode::Ok);
        assert_eq!(container.add_int32(42, Some("answer")), SolClientReturnCode::Ok);
        assert_eq!(container.add_boolean(true, Some("flag")), SolClientReturnCode::Ok);
        assert_eq!(container.add_null(Some("empty")), SolClientReturnCode::Ok);
        
        // Get all fields
        let fields = container.get_all_fields().unwrap();
        assert_eq!(fields.len(), 4);
        
        // Verify field contents
        let mut found_greeting = false;
        let mut found_answer = false;
        let mut found_flag = false;
        let mut found_empty = false;
        
        for field in fields {
            match field.name.as_deref() {
                Some("greeting") => {
                    if let ContainerFieldType::String(s) = field.field_type {
                        assert_eq!(s, "hello");
                        found_greeting = true;
                    }
                }
                Some("answer") => {
                    if let ContainerFieldType::Int32(i) = field.field_type {
                        assert_eq!(i, 42);
                        found_answer = true;
                    }
                }
                Some("flag") => {
                    if let ContainerFieldType::Boolean(b) = field.field_type {
                        assert_eq!(b, true);
                        found_flag = true;
                    }
                }
                Some("empty") => {
                    if let ContainerFieldType::Null = field.field_type {
                        found_empty = true;
                    }
                }
                _ => {}
            }
        }
        
        assert!(found_greeting, "greeting field not found");
        assert!(found_answer, "answer field not found");
        assert!(found_flag, "flag field not found");
        assert!(found_empty, "empty field not found");
    }
}