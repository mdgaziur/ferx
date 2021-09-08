use std::os::raw::c_char;
use x11::xlib::{XDefaultScreen, XDisplayHeight, XDisplayWidth, XOpenDisplay};

pub fn get_resolution() -> String {
    let display = unsafe { XOpenDisplay(0 as *const c_char) };

    if unsafe { display.as_ref() }.is_none() {
        return String::from("Not Available");
    }
    let snum = unsafe { XDefaultScreen(display) };
    let width = unsafe { XDisplayWidth(display, snum) };
    let height = unsafe { XDisplayHeight(display, snum) };

    format!("{}x{}", width, height)
}
