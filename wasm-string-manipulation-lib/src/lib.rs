use std::result;

// we're going to set a global that gets updated with
// the last length of the string slice that we've been working with. 
// this is a not-so-great remedy to wasm's single return type issue
static mut LAST_LEN: usize = 0;

#[no_mangle]
pub fn _malloc(len: usize) -> *mut u8 {

    // create an empty buffer in memory
    let mut buf = Vec::with_capacity(len);

    // get the pointer to the buffer created above 
    let ptr = buf.as_mut_ptr();

    // NOTE: will still work without this, but the memory will 
    // be freed at the end of the closure 
    std::mem::forget(buf);

    // return the pointer
    ptr

}

#[no_mangle]
pub fn say_hello(ptr: *mut u8, len: usize) -> *mut u8 {

    // create an empty string
    let str: String; 

    // String::from_raw_parts is considered an unsafe operation, because
    // that underlying memory could've been changed over the lifetime of the program
    unsafe {
        str = String::from_raw_parts(ptr, len, len);
    }

    // format a new string
    let mut result_string = format!("Hello, {}.", str);

    // we are going to get the length of the previous string
    unsafe{
        LAST_LEN = result_string.len();
    }

    // return the pointer to the resultant string.
    result_string.as_mut_ptr()

}

#[no_mangle]
pub fn _get_last_length() -> usize {

    let result; 

    // assigning from a global var is considered an unsafe operation
    unsafe{
        result = LAST_LEN
    }

    // return the last length
    result
}