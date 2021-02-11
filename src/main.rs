#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
use ::c_buffer_overflow_c2rust::*;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn matches(mut pw: *const libc::c_char,
                                 mut upw: *mut libc::c_char,
                                 mut msg: *mut libc::c_char) -> libc::c_int {
    printf(msg);
    scanf(b"%s\x00" as *const u8 as *const libc::c_char, upw);
    return (strcmp(pw, upw) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pw_matches(mut pw: *const libc::c_char,
                                    mut upw: *mut libc::c_char)
 -> libc::c_int {
    if matches(pw, upw,
               b"enter password: \x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char) != 0 {
        return 1 as libc::c_int
    } else {
        return matches(pw, upw,
                       b"wrong, one more try: \x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char)
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut user_password: [libc::c_char; 16] = [0; 16];
    let password: [libc::c_char; 16] =
        *::std::mem::transmute::<&[u8; 16],
                                 &[libc::c_char; 16]>(b"secretpw\x00\x00\x00\x00\x00\x00\x00\x00");
    if pw_matches(password.as_ptr(), user_password.as_mut_ptr()) != 0 {
        printf(b"access granted!\n\x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"access denied.\n\x00" as *const u8 as *const libc::c_char);
    };
    return 0;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
