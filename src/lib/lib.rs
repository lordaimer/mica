pub mod server;
pub mod client;

use std::ffi::CStr;
use libc::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn run_server(port: u16) {
    server::start_server(port);
}

#[unsafe(no_mangle)]
pub extern "C" fn run_client(addr_ptr: *const c_char) {
    // convert C string → &str
    let c_str = unsafe { CStr::from_ptr(addr_ptr) };
    let addr = c_str.to_str().unwrap_or_default();
    // block_on since this is a C boundary
    futures::executor::block_on(client::connect(addr));
}