use std::{
    collections::HashMap,
    ffi::{
        c_void,
        CStr,
        c_char,
    },
    sync::Mutex,
};
use dart_sys::{
    Dart_Handle,
    Dart_PersistentHandle,
    Dart_NewPersistentHandle_DL,
    Dart_HandleFromPersistent_DL,
    Dart_DeletePersistentHandle_DL,
    Dart_InitializeApiDL,
};
use once_cell::sync::Lazy;

pub type CallbackInvoker = unsafe extern "C" fn(callbackHandle: Dart_Handle, msg: *const c_char, msgSize: usize);

static mut HANDLES: Lazy<Mutex<HashMap<String, Dart_PersistentHandle>>> = Lazy::new(|| {
    let m: HashMap<String, Dart_PersistentHandle> = HashMap::new();
    Mutex::new(m)
});

static mut CB_INVOKER: Option<*const CallbackInvoker> = None;

#[no_mangle]
pub unsafe extern "C" fn init(data: *mut c_void) -> isize {
    unsafe {
        Dart_InitializeApiDL(data)
    }
}

unsafe fn add_callback(handle: Dart_Handle, key: *const c_char) {
    unsafe {
        let handles = HANDLES.get_mut();
        if let Ok(handles) = handles {
            let c_str: &CStr = CStr::from_ptr(key);
            if let Ok(id) = c_str.to_str() {
                if let Some(f) = Dart_NewPersistentHandle_DL {
                    let handle = f(handle);
                    handles.insert(String::from(id), handle);
                    println!("Registering callback \"{}\" with handle: {:#?}", id, handle);
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn register_invoker(invoker: *const CallbackInvoker) {
    unsafe {
        CB_INVOKER = Some(invoker);
    }
}

#[no_mangle]
pub unsafe extern "C" fn register_callback(handle: Dart_Handle, key: *const c_char) {
    unsafe {
        add_callback(handle, key);
    }
}

#[no_mangle]
pub unsafe extern "C" fn invoke(msg: *const c_char, msg_len: usize, key: *const c_char) {
    unsafe {
        let handles = HANDLES.get_mut();
        if let Ok(handles) = handles {
            let c_str: &CStr = CStr::from_ptr(key);
            if let Ok(id) = c_str.to_str() {
                if let Some(handle) = handles.get(id) {
                    if let Some(invoker) = CB_INVOKER {
                        if let Some(persistent_handle) = Dart_HandleFromPersistent_DL {
                            let handle = persistent_handle(*handle);
                            println!("Invoking callback \"{}\" with handle: {:#?}", id, handle);
                            (*invoker)(handle, msg, msg_len);
                        }
                    }
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
            for (id, handle) in &mut *handles {
                if let Some(free) = Dart_DeletePersistentHandle_DL {
                    println!("Releasing callback \"{}\" with handle: {:#?}", id, handle);
                    free(*handle);
                }
            }
            handles.clear();
        }
    }
}
