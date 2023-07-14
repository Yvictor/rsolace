use std::thread::JoinHandle;

use pyo3::prelude::*;
// use pyo3::types::{PyDict, PyTuple};
use rsolace::{solclient::{SessionProps, SolClient}, solevent::SolEvent};
use tracing_subscriber;

// #[pyclass]
// struct PySolEvent(SolEvent);

// impl Into<PyObject> for SolEvent {
//     fn into_py(self, py: Python<'_>) -> PyObject {
//         self.0
//     }
// }

#[pyclass(name = "SolCleint")]
struct PySolCleint {
    solclient: SolClient,
    event_callback: Option<PyObject>, // callable
    th_event_join: Option<JoinHandle<()>>
}

#[pyfunction]
fn init_tracing_logger() {
tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
}

#[pymethods]
impl PySolCleint {
    #[new]
    fn __new__() -> Self {
        let solclient = SolClient::default();
        PySolCleint {
            solclient: solclient,
            event_callback: None,
            th_event_join: None
        }
    }

    #[pyo3(signature = ())]
    fn set_event_callback(&mut self, py: Python<'_>,){
        match &self.th_event_join {
            Some(_) =>{  
            },
            None => {
                let event_recv = self.solclient.get_event_receiver();
                let th_event_join = std::thread::spawn(move || loop {
                    match event_recv.recv() {
                        Ok(event) => {
                            // if let Some(cb) = &self.event_callback {
                                // cb.call(py, event);
                            // }
                            tracing::info!("{:?}", event);
                        }
                        Err(e) => {
                            tracing::error!("recv event error: {:?}", e);
                            break;
                        }
                    }
                });
                self.th_event_join = Some(th_event_join);
            }
        }
    }

    #[pyo3(signature = (
        host, vpn, username, password, clientname="", connect_timeout_ms=3000, 
        reconnect_retries=10, keep_alive_ms=3000, reconnect_retry_wait=3000,
        keep_alive_limit=3, compression_level=1, connect_retries=3, 
        reapply_subscriptions=true, generate_sender_id=false, generate_sequence_number=false,
        generate_send_timestamps=false, generate_rcv_timestamps=false
    ))]
    fn connect(
        &mut self,
        host: &str,
        vpn: &str,
        username: &str,
        password: &str,
        clientname: &str,
        connect_timeout_ms: u32,
        reconnect_retries: u32,
        keep_alive_ms: u32,
        reconnect_retry_wait: u32,
        keep_alive_limit: u32,
        compression_level: u32,
        connect_retries: u32,
        reapply_subscriptions: bool,
        generate_sender_id: bool,
        generate_sequence_number: bool,
        generate_send_timestamps: bool,
        generate_rcv_timestamps: bool,
    ) -> bool {
        let props = SessionProps::default()
            .username(username)
            .password(password)
            .host(host)
            .vpn(vpn)
            .client_name(clientname)
            .compression_level(compression_level)
            .connect_timeout_ms(connect_timeout_ms)
            .connect_retries(connect_retries)
            .reconnect_retries(reconnect_retries)
            .reapply_subscriptions(reapply_subscriptions)
            .reconnect_retry_wait_ms(reconnect_retry_wait)
            .generate_rcv_timestamps(generate_rcv_timestamps)
            .generate_send_timestamps(generate_send_timestamps)
            .generate_sender_id(generate_sender_id)
            .generate_sequence_number(generate_sequence_number)
            .keep_alive_limit(keep_alive_limit)
            .keep_alive_int_ms(keep_alive_ms);
        self.solclient.connect(props)
    }

    #[pyo3(signature = ())]
    fn disconnect(&mut self) {
        self.solclient.disconnect()
    }
}



/// A Python module implemented in Rust.
#[pymodule]
fn pyrsolace(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySolCleint>()?;
    m.add_function(wrap_pyfunction!(init_tracing_logger, m)?)?;
    Ok(())
}
