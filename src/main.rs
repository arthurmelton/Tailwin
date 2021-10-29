extern crate libc;
extern crate x11;

mod tailwin;

use std::{ffi::CString, process::Command};
use std::mem::zeroed;
use std::thread;

use libc::{c_int, c_uint};

use x11::xlib;

fn max(a : c_int, b : c_int) -> c_uint { if a > b { a as c_uint } else { b as c_uint } }

fn main() {
    let mut arg0 = 0x0 as i8;
    let display : *mut xlib::Display = unsafe { xlib::XOpenDisplay(&mut arg0) };

    let mut attr: xlib::XWindowAttributes = unsafe { zeroed() };
    let mut start: xlib::XButtonEvent = unsafe { zeroed() };

    if display.is_null() {
        std::process::exit(1);
    }

    thread::spawn(|| {
        tailwin::on_startup();
    });

    let modmask;

    match tailwin::mod_mask() {
        1 => {modmask = xlib::Mod1Mask;},
        2 => {modmask = xlib::Mod2Mask;},
        3 => {modmask = xlib::Mod3Mask;},
        4 => {modmask = xlib::Mod4Mask;},
        5 => {modmask = xlib::Mod5Mask;},
        _ => {modmask = xlib::Mod2Mask; Command::new("sh").args(&["-c", "zenity --info --text='Invalid modmask'"]).spawn().expect("failed to execute process");}
    }

    let keys = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "[", "]", "/", "=", "\\", "-", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "Home", "SPACE"];

    for i in keys {
        let x = CString::new(i.to_string()).unwrap();
        unsafe {
            xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, xlib::XStringToKeysym(x.as_ptr())) as c_int, modmask,
            xlib::XDefaultRootWindow(display), true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
        };
    }
    unsafe {
        xlib::XGrabButton(display, 1, modmask, xlib::XDefaultRootWindow(display), true as c_int,
        (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
        0, 0);
        xlib::XGrabButton(display, 3, modmask, xlib::XDefaultRootWindow(display), true as c_int,
        (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
        0, 0);
    };

    start.subwindow = 0;

    let mut event: xlib::XEvent = unsafe { zeroed() };

    loop {
        unsafe {
            xlib::XNextEvent(display, &mut event);

            match event.get_type() {
                xlib::KeyPress => {
                    let xkey: xlib::XKeyEvent = From::from(event);
                    match tailwin::on_key(xkey.keycode).as_str() {
                        "null" => {},
                        "destroy" => {xlib::XDestroyWindow(display, xkey.subwindow);},
                        _ => {Command::new("sh").args(&["-c", "zenity --info --text='Sorry but you returned something I dont understand in the on_key'"]).spawn().expect("failed to execute process");}
                    }
                },
                xlib::ButtonPress => {
                    let xbutton: xlib::XButtonEvent = From::from(event);
                    if xbutton.subwindow != 0 {
                        xlib::XGetWindowAttributes(display, xbutton.subwindow, &mut attr);
                        start = xbutton;
                    }
                },
                xlib::MotionNotify => {
                    if start.subwindow != 0 {
                        let xbutton: xlib::XButtonEvent = From::from(event);
                        let xdiff : c_int = xbutton.x_root - start.x_root;
                        let ydiff : c_int = xbutton.y_root - start.y_root;
                        xlib::XMoveResizeWindow(display, start.subwindow,
                                                attr.x + (if start.button==1 { xdiff } else { 0 }),
                                                attr.y + (if start.button==1 { ydiff } else { 0 }),
                                                max(1, attr.width + (if start.button==3 { xdiff } else { 0 })),
                                                max(1, attr.height + (if start.button==3 { ydiff } else { 0 })));
                    }
                },
                xlib::ButtonRelease => {
                    start.subwindow = 0;
                },
                _ => {}
            };
        }
    }
}
