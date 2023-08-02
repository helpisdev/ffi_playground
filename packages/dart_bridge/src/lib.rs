#![allow(non_camel_case_types)]

use dart_sys::{
    Dart_PersistentHandle,
    Dart_InitializeApiDL,
    // Dart_NewPersistentHandle_DL,
    // Dart_HandleFromPersistent_DL,
    // Dart_DeletePersistentHandle_DL,
};
use std::{sync::Mutex, collections::HashMap, ffi::c_void};
use once_cell::sync::Lazy;

pub type Dart_Handle = dart_sys::Dart_Handle;
pub type CallbackInvoker = fn(handle: Dart_Handle, msg: *const u8, msgSize: usize) -> ();

static mut CALLBACK_INVOKER: Option<CallbackInvoker> = None;
static mut CALLBACKS: Lazy<Mutex<HashMap<*const u8, Dart_PersistentHandle>>> = Lazy::new(|| {
    let m: HashMap<*const u8, Dart_PersistentHandle> = HashMap::new();
    Mutex::new(m)
});

#[no_mangle]
pub unsafe extern "C" fn init(data: *mut c_void) -> isize {
    unsafe {
        Dart_InitializeApiDL(data)
    }
}

#[no_mangle]
pub unsafe extern "C" fn register_invoker(invoker: *const CallbackInvoker) -> () {
    CALLBACK_INVOKER = Some(*(invoker.clone()));
}

#[no_mangle]
pub unsafe extern "C" fn register_callback(cb: Dart_Handle, id: *const u8) -> () {
    match CALLBACKS.try_lock() {
        Ok(mut lock) => {
            lock.insert(id.clone(), cb);
        },
        Err(_) => return,
    }
}

#[no_mangle]
pub unsafe extern "C" fn invoke(msg: *const u8, msg_len: usize, id: *const u8) -> () {
    match CALLBACKS.try_lock() {
        Ok(lock) => {
            match lock.get(&id.clone()) {
                Some(cb) => {
                    match CALLBACK_INVOKER {
                        Some(invoker) => {
                            invoker(*cb, msg.clone(), msg_len)
                        },
                        None => return,
                    }
                },
                None => return,
            }
        },
        Err(_) => return,
    }
}

#[no_mangle]
pub unsafe extern "C" fn release() -> () {
    match CALLBACKS.try_lock() {
        Ok(lock) => {
            for (_, _handle) in lock.iter() {
                // unsafe {
                //     Dart_DeletePersistentHandle_DL(handle)
                // }
            }
        },
        Err(_) => return,
    }
}
