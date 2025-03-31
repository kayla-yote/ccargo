use libc;
use std::env;
use std::ffi;
use std::process;

extern "C" {
   fn c_main(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
}

fn main() {
   let arg_cstrs: Vec<ffi::CString> = env::args().map(|x| ffi::CString::new(x).unwrap()).collect();
   let arg_cstr_ptrs: Vec<*const libc::c_char> = arg_cstrs.iter().map(|x| x.as_ptr()).collect();
   let argc = arg_cstr_ptrs.len() as libc::c_int;
   let argv = arg_cstr_ptrs.as_ptr();
   let ret =  unsafe { c_main(argc, argv) };
   process::exit(ret);
}
