
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate user32;

use self::winapi::*;

use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

fn to_wide(msg:&str) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    wide
}

pub unsafe extern "system" fn win_proc(
    h_wnd :HWND,
    msg :UINT,
    w_param :WPARAM,
    l_param :LPARAM) -> LRESULT {
    if msg == winapi::winuser::WM_DESTROY {
        user32::PostQuitMessage(0);
    }
    return user32::DefWindowProcW(h_wnd, msg, w_param, l_param);
}



pub struct WindowBuilder {
    title : Option<String>,
    size : Option<(i32, i32)>
}

impl WindowBuilder {

    pub fn new() -> WindowBuilder {
        WindowBuilder {
            title : None,
            size : None
        }
    }

    pub fn with_title<'a>(&'a mut self, string : String) -> &'a mut WindowBuilder {
        self.title = Some(string.clone());
        self
    }

    pub fn with_size<'a>(&'a mut self, width : i32, height : i32) -> &'a mut WindowBuilder {
        self.size = Some((width, height));
        self
    }

    pub fn build(&self) -> Window {
        let class_name = to_wide("my_window");
        let icon = unsafe {
            user32::LoadIconW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION)
        };
        let cursor = unsafe {
            user32::LoadCursorW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION)
        };
        let wnd = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(win_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: icon,
            hCursor: cursor,
            hbrBackground: 16 as HBRUSH,
            lpszMenuName: 0 as LPCWSTR,
            lpszClassName: class_name.as_ptr(),
        };
        let ret = unsafe {
            user32::RegisterClassW(&wnd)
        };
        if ret == 0 {
            let msg = to_wide("register failed.");
            unsafe {
                user32::MessageBoxW(null_mut(), msg.as_ptr(), msg.as_ptr(), winapi::MB_OK);
            }
        }
        let h_wnd_desktop = unsafe {
            user32::GetDesktopWindow()
        };
        unsafe {
            let hwnd = user32::CreateWindowExW(
                WS_EX_CLIENTEDGE,
                class_name.as_ptr(),
                class_name.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                self.size.unwrap_or((400, 400)).0,
                self.size.unwrap_or((400, 400)).1,
                h_wnd_desktop,
                0 as HMENU,
                0 as HINSTANCE,
                null_mut()
            );

            let new_title = self.title.clone().unwrap_or("Default Window Title".to_string());

            user32::SetWindowTextW(hwnd, to_wide(&new_title).as_ptr());
            user32::UpdateWindow(hwnd);

            Window {
                hwnd
            }
        }
    }

}

pub struct Window {
    hwnd : HWND,
}

impl Window {

    pub fn show(&mut self){
        unsafe {
            user32::ShowWindow(self.hwnd, SW_SHOWNORMAL);
        }
    }

}
