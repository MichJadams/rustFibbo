use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle ]
pub fn add(a: i32, b:i32) -> i32{
    return a+b
}

#[no_mangle ] //makes you be able to get it from js 
pub fn fib(input: u32) -> *mut c_char{
    // let val: u32 = input.trim().parse().unwrap();

    let mut f_zero: u64 = 0;
    let mut f_one: u64 = 1;
    let mut f_n: u64 = f_zero + f_one; 

    for _ in 1..input{
        f_n= f_zero + f_one;
        f_zero = f_one;
        f_one = f_n;
    } 

    return CString::new(f_n.to_string()).unwrap().into_raw();
}