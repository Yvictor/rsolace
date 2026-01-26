//! Session properties for Solace client connections.
//!
//! This module provides the `SessionProps` struct for configuring Solace session connections,
//! including SSL/TLS and OAuth2 authentication options.

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;

use crate::utils::ConvertToCString;

/// Session properties for configuring a Solace client connection.
///
/// Use the builder pattern to configure the session:
/// ```ignore
/// let props = SessionProps::default()
///     .host("tcp://localhost:55555")
///     .vpn("default")
///     .username("user")
///     .password("pass");
/// ```
#[derive(Debug)]
pub struct SessionProps {
    // === Required fields ===
    username: CString,
    password: CString,
    host: CString,
    vpn: CString,
    client_name: CString,
    connect_timeout_ms: CString,
    tcp_nodelay: CString,
    keep_alive_int_ms: CString,
    keep_alive_limit: CString,
    compression_level: CString,
    generate_rcv_timestamps: CString,
    generate_send_timestamps: CString,
    generate_sender_id: CString,
    generate_sequence_number: CString,
    connect_retries: CString,
    reconnect_retries: CString,
    reconnect_retry_wait_ms: CString,
    reapply_subscriptions: CString,

    // === SSL optional fields ===
    ssl_trust_store_dir: Option<CString>,
    ssl_validate_certificate: Option<CString>,
    ssl_validate_certificate_date: Option<CString>,
    ssl_validate_certificate_host: Option<CString>,
    ssl_cipher_suites: Option<CString>,
    ssl_excluded_protocols: Option<CString>,
    ssl_client_certificate_file: Option<CString>,
    ssl_client_private_key_file: Option<CString>,
    ssl_client_private_key_file_password: Option<CString>,
    ssl_trusted_common_name_list: Option<CString>,

    // === OAuth2/Authentication optional fields ===
    authentication_scheme: Option<CString>,
    oauth2_access_token: Option<CString>,
    oauth2_issuer_identifier: Option<CString>,
    oidc_id_token: Option<CString>,
}

impl SessionProps {
    /// Convert session properties to a C-compatible array of property pointers.
    ///
    /// Returns a `Vec` of property key-value pairs terminated by a null pointer,
    /// suitable for passing to Solace C API functions.
    pub fn to_c(&self) -> Vec<*const c_char> {
        let mut props = Vec::with_capacity(60);

        macro_rules! add_prop {
            ($key:expr, $val:expr) => {
                props.push($key.as_ptr() as *const c_char);
                props.push($val.as_ptr() as *const c_char);
            };
        }

        macro_rules! add_optional {
            ($key:expr, $val:expr) => {
                if let Some(ref v) = $val {
                    props.push($key.as_ptr() as *const c_char);
                    props.push(v.as_ptr() as *const c_char);
                }
            };
        }

        // Required fields
        add_prop!(rsolace_sys::SOLCLIENT_SESSION_PROP_HOST, self.host);
        add_prop!(rsolace_sys::SOLCLIENT_SESSION_PROP_VPN_NAME, self.vpn);
        add_prop!(rsolace_sys::SOLCLIENT_SESSION_PROP_USERNAME, self.username);
        add_prop!(rsolace_sys::SOLCLIENT_SESSION_PROP_PASSWORD, self.password);
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_COMPRESSION_LEVEL,
            self.compression_level
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_CLIENT_NAME,
            self.client_name
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_CONNECT_TIMEOUT_MS,
            self.connect_timeout_ms
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_TCP_NODELAY,
            self.tcp_nodelay
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_GENERATE_RCV_TIMESTAMPS,
            self.generate_rcv_timestamps
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_GENERATE_SEND_TIMESTAMPS,
            self.generate_send_timestamps
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_GENERATE_SENDER_ID,
            self.generate_sender_id
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_GENERATE_SEQUENCE_NUMBER,
            self.generate_sequence_number
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_CONNECT_RETRIES,
            self.connect_retries
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_RECONNECT_RETRIES,
            self.reconnect_retries
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_RECONNECT_RETRY_WAIT_MS,
            self.reconnect_retry_wait_ms
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_REAPPLY_SUBSCRIPTIONS,
            self.reapply_subscriptions
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_KEEP_ALIVE_INT_MS,
            self.keep_alive_int_ms
        );
        add_prop!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_KEEP_ALIVE_LIMIT,
            self.keep_alive_limit
        );

        // SSL optional fields
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_TRUST_STORE_DIR,
            self.ssl_trust_store_dir
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_VALIDATE_CERTIFICATE,
            self.ssl_validate_certificate
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_VALIDATE_CERTIFICATE_DATE,
            self.ssl_validate_certificate_date
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_VALIDATE_CERTIFICATE_HOST,
            self.ssl_validate_certificate_host
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_CIPHER_SUITES,
            self.ssl_cipher_suites
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_EXCLUDED_PROTOCOLS,
            self.ssl_excluded_protocols
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_CLIENT_CERTIFICATE_FILE,
            self.ssl_client_certificate_file
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_CLIENT_PRIVATE_KEY_FILE,
            self.ssl_client_private_key_file
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_CLIENT_PRIVATE_KEY_FILE_PASSWORD,
            self.ssl_client_private_key_file_password
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_SSL_TRUSTED_COMMON_NAME_LIST,
            self.ssl_trusted_common_name_list
        );

        // OAuth2/Authentication optional fields
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_AUTHENTICATION_SCHEME,
            self.authentication_scheme
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_OAUTH2_ACCESS_TOKEN,
            self.oauth2_access_token
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_OAUTH2_ISSUER_IDENTIFIER,
            self.oauth2_issuer_identifier
        );
        add_optional!(
            rsolace_sys::SOLCLIENT_SESSION_PROP_OIDC_ID_TOKEN,
            self.oidc_id_token
        );

        // Null terminator
        props.push(null());
        props
    }

    // === Required field builder methods ===

    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_cstring();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_cstring();
        self
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_cstring();
        self
    }

    pub fn vpn(mut self, vpn: &str) -> Self {
        self.vpn = vpn.to_cstring();
        self
    }

    pub fn compression_level(mut self, compression_level: u32) -> Self {
        assert!(compression_level < 10);
        self.compression_level = compression_level.to_cstring();
        self
    }

    pub fn connect_timeout_ms(mut self, timeout: u32) -> Self {
        self.connect_timeout_ms = timeout.to_cstring();
        self
    }

    pub fn tcp_nodelay(mut self, enable: bool) -> Self {
        self.tcp_nodelay = enable.to_cstring();
        self
    }

    pub fn client_name(mut self, client_name: &str) -> Self {
        self.client_name = client_name.to_cstring();
        self
    }

    pub fn keep_alive_int_ms(mut self, keep_alive_int_ms: u32) -> Self {
        self.keep_alive_int_ms = keep_alive_int_ms.to_cstring();
        self
    }

    pub fn keep_alive_limit(mut self, keep_alive_limit: u32) -> Self {
        self.keep_alive_limit = keep_alive_limit.to_cstring();
        self
    }

    pub fn generate_rcv_timestamps(mut self, generate_rcv_timestamps: bool) -> Self {
        self.generate_rcv_timestamps = generate_rcv_timestamps.to_cstring();
        self
    }

    pub fn generate_send_timestamps(mut self, generate_send_timestamps: bool) -> Self {
        self.generate_send_timestamps = generate_send_timestamps.to_cstring();
        self
    }

    pub fn generate_sender_id(mut self, generate_sender_id: bool) -> Self {
        self.generate_sender_id = generate_sender_id.to_cstring();
        self
    }

    pub fn generate_sequence_number(mut self, generate_sequence_number: bool) -> Self {
        self.generate_sequence_number = generate_sequence_number.to_cstring();
        self
    }

    /// Set number of connection retries.
    /// - `0`: no retries (try once and give up)
    /// - `-1`: retry forever
    /// - positive: retry N times
    pub fn connect_retries(mut self, connect_retries: i32) -> Self {
        self.connect_retries = connect_retries.to_cstring();
        self
    }

    /// Set number of reconnection retries after connection goes down.
    /// - `0`: no automatic reconnection
    /// - `-1`: reconnect forever
    /// - positive: retry N times
    pub fn reconnect_retries(mut self, reconnect_retries: i32) -> Self {
        self.reconnect_retries = reconnect_retries.to_cstring();
        self
    }

    pub fn reconnect_retry_wait_ms(mut self, reconnect_retry_wait_ms: u32) -> Self {
        self.reconnect_retry_wait_ms = reconnect_retry_wait_ms.to_cstring();
        self
    }

    pub fn reapply_subscriptions(mut self, reapply_subscriptions: bool) -> Self {
        self.reapply_subscriptions = reapply_subscriptions.to_cstring();
        self
    }

    // === SSL builder methods ===

    /// Set the SSL trust store directory path.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn ssl_trust_store_dir(mut self, dir: Option<&str>) -> Self {
        self.ssl_trust_store_dir = dir.map(|v| v.to_cstring());
        self
    }

    /// Set whether to validate the server certificate.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value
    pub fn ssl_validate_certificate(mut self, validate: Option<bool>) -> Self {
        self.ssl_validate_certificate = validate.map(|v| v.to_cstring());
        self
    }

    /// Set whether to validate the certificate date.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value
    pub fn ssl_validate_certificate_date(mut self, validate: Option<bool>) -> Self {
        self.ssl_validate_certificate_date = validate.map(|v| v.to_cstring());
        self
    }

    /// Set whether to validate the certificate host name.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value
    pub fn ssl_validate_certificate_host(mut self, validate: Option<bool>) -> Self {
        self.ssl_validate_certificate_host = validate.map(|v| v.to_cstring());
        self
    }

    /// Set the SSL cipher suites to use.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn ssl_cipher_suites(mut self, suites: Option<&str>) -> Self {
        self.ssl_cipher_suites = suites.map(|v| v.to_cstring());
        self
    }

    /// Set the SSL protocols to exclude.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn ssl_excluded_protocols(mut self, protocols: Option<&str>) -> Self {
        self.ssl_excluded_protocols = protocols.map(|v| v.to_cstring());
        self
    }

    /// Set the path to the client certificate file for mutual TLS.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn ssl_client_certificate_file(mut self, path: Option<&str>) -> Self {
        self.ssl_client_certificate_file = path.map(|v| v.to_cstring());
        self
    }

    /// Set the path to the client private key file for mutual TLS.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn ssl_client_private_key_file(mut self, path: Option<&str>) -> Self {
        self.ssl_client_private_key_file = path.map(|v| v.to_cstring());
        self
    }

    /// Set the password for the client private key file.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn ssl_client_private_key_file_password(mut self, password: Option<&str>) -> Self {
        self.ssl_client_private_key_file_password = password.map(|v| v.to_cstring());
        self
    }

    /// Set the list of trusted common names for certificate validation.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn ssl_trusted_common_name_list(mut self, names: Option<&str>) -> Self {
        self.ssl_trusted_common_name_list = names.map(|v| v.to_cstring());
        self
    }

    // === OAuth2/Authentication builder methods ===

    /// Set the authentication scheme (BASIC, OAUTH2, CLIENT_CERTIFICATE, GSS_KRB).
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn authentication_scheme(mut self, scheme: Option<&str>) -> Self {
        self.authentication_scheme = scheme.map(|v| v.to_cstring());
        self
    }

    /// Set the OAuth2 access token.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn oauth2_access_token(mut self, token: Option<&str>) -> Self {
        self.oauth2_access_token = token.map(|v| v.to_cstring());
        self
    }

    /// Set the OAuth2 issuer identifier.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn oauth2_issuer_identifier(mut self, issuer: Option<&str>) -> Self {
        self.oauth2_issuer_identifier = issuer.map(|v| v.to_cstring());
        self
    }

    /// Set the OIDC ID token.
    /// - `None` → property not sent to Solace
    /// - `Some(value)` → property sent with the value (including empty string)
    pub fn oidc_id_token(mut self, token: Option<&str>) -> Self {
        self.oidc_id_token = token.map(|v| v.to_cstring());
        self
    }
}

impl Default for SessionProps {
    fn default() -> Self {
        Self {
            username: "".to_cstring(),
            password: "".to_cstring(),
            host: "".to_cstring(),
            vpn: "".to_cstring(),
            client_name: "".to_cstring(),
            connect_timeout_ms: 30000.to_cstring(),
            tcp_nodelay: true.to_cstring(),
            keep_alive_int_ms: 3000.to_cstring(),
            keep_alive_limit: 3.to_cstring(),
            compression_level: 0.to_cstring(),
            generate_rcv_timestamps: false.to_cstring(),
            generate_send_timestamps: false.to_cstring(),
            generate_sender_id: false.to_cstring(),
            generate_sequence_number: false.to_cstring(),
            connect_retries: 0.to_cstring(),
            reconnect_retries: 0.to_cstring(),
            reconnect_retry_wait_ms: 3000.to_cstring(),
            reapply_subscriptions: false.to_cstring(),
            // SSL optional fields - all None by default
            ssl_trust_store_dir: None,
            ssl_validate_certificate: None,
            ssl_validate_certificate_date: None,
            ssl_validate_certificate_host: None,
            ssl_cipher_suites: None,
            ssl_excluded_protocols: None,
            ssl_client_certificate_file: None,
            ssl_client_private_key_file: None,
            ssl_client_private_key_file_password: None,
            ssl_trusted_common_name_list: None,
            // OAuth2/Authentication optional fields - all None by default
            authentication_scheme: None,
            oauth2_access_token: None,
            oauth2_issuer_identifier: None,
            oidc_id_token: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_session_props() {
        let props = SessionProps::default();
        let c_props = props.to_c();
        // Should have required props (18 pairs = 36 entries) + null terminator
        assert!(c_props.len() >= 37);
        // Last element should be null
        assert!(c_props.last().unwrap().is_null());
    }

    #[test]
    fn test_ssl_trust_store_dir_none() {
        let props = SessionProps::default().ssl_trust_store_dir(None);
        assert!(props.ssl_trust_store_dir.is_none());
    }

    #[test]
    fn test_ssl_trust_store_dir_empty_string() {
        // Empty string should be set (not ignored)
        let props = SessionProps::default().ssl_trust_store_dir(Some(""));
        assert!(props.ssl_trust_store_dir.is_some());
    }

    #[test]
    fn test_ssl_trust_store_dir_with_path() {
        let props = SessionProps::default().ssl_trust_store_dir(Some("/path/to/certs"));
        assert!(props.ssl_trust_store_dir.is_some());
    }

    #[test]
    fn test_ssl_validate_certificate() {
        let props = SessionProps::default().ssl_validate_certificate(Some(false));
        assert!(props.ssl_validate_certificate.is_some());
    }

    #[test]
    fn test_ssl_validate_certificate_none() {
        let props = SessionProps::default().ssl_validate_certificate(None);
        assert!(props.ssl_validate_certificate.is_none());
    }

    #[test]
    fn test_builder_chain() {
        let props = SessionProps::default()
            .host("tcps://localhost:55443")
            .vpn("default")
            .username("user")
            .password("pass")
            .ssl_trust_store_dir(Some("/certs"))
            .ssl_validate_certificate(Some(true));

        let c_props = props.to_c();
        // Should have more entries than default due to SSL props
        assert!(c_props.len() > 37);
    }
}
