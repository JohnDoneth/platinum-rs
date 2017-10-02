
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate user32;

use self::winapi::*;

use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std;

pub struct EventLoop {

}

impl EventLoop {

    pub fn poll_events(){
        println!("init the msg structure success.");
        // Finally we run the standard application loop -

        let mut msg = winapi::winuser::MSG {
            hwnd : 0 as HWND,
            message : 0 as UINT,
            wParam : 0 as WPARAM,
            lParam : 0 as LPARAM,
            time : 0 as DWORD,
            pt : winapi::windef::POINT { x: 0, y: 0, },
        };

        loop
        {
            unsafe {
                println!("get msg from the queue");
                let pm = user32::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0);
                println!("msg received. {:?}", msg);
                if pm > 0 {
                    user32::TranslateMessage(&mut msg);
                    user32::DispatchMessageW(&mut msg);
                }
            }
        }
    }

}
