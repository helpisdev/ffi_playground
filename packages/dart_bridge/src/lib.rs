use std::{
    collections::HashMap,
    ffi::c_void,
    sync::Mutex,
};
use dart_sys::{
    Dart_Handle,
    Dart_PersistentHandle,
    Dart_InitializeApiDL,
};
use once_cell::sync::Lazy;

pub type CallbackInvoker = fn(callbackHandle: Dart_Handle, msg: *const u8, msgSize: usize);

static mut HANDLES: Lazy<Mutex<HashMap<*const u8, Dart_PersistentHandle>>> = Lazy::new(|| {
    let m: HashMap<*const u8, Dart_PersistentHandle> = HashMap::new();
    Mutex::new(m)
});

static mut CB_INVOKER: Option<CallbackInvoker> = None;

#[no_mangle]
pub unsafe extern "C" fn init(data: *mut c_void) -> isize {
    unsafe {
        Dart_InitializeApiDL(data)
    }
}

unsafe fn add_callback(handle: Dart_PersistentHandle, key: *const u8) {
    unsafe {
        let handles = HANDLES.get_mut();
        if let Ok(handles) = handles {
            let entry = handles.entry(key).or_insert(handle);
            *entry = handle;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn register_invoker(invoker: *const CallbackInvoker) {
    unsafe {
        CB_INVOKER = Some(*invoker);
    }
}

#[no_mangle]
pub unsafe extern "C" fn register_callback(handle: Dart_PersistentHandle, key: *const u8) {
    unsafe {
        add_callback(handle, key);
    }
}

#[no_mangle]
pub unsafe extern "C" fn invoke(msg: *const u8, msg_len: usize, key: *const u8) {
    unsafe {
        let handles = HANDLES.get_mut();
        if let Ok(handles) = handles {
            if let Some(handle) = handles.get(&key) {
                if let Some(invoker) = CB_INVOKER {
                    invoker(*handle, msg, msg_len);
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn release() {
    unsafe {
        let handles = HANDLES.get_mut();
        if let Ok(handles) = handles {
            handles.clear();
        }
    }
}
