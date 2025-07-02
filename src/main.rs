use std::ptr;
use winapi::um::winuser::{FindWindowExA, GetWindowTextA, IsWindowVisible, ShowWindow, SW_HIDE, SW_SHOW};
use winapi::shared::windef::HWND;


const MAX_PATH: usize = 260;

// 递归遍历子窗口，通过窗口文本（名称）查找句柄
fn find_child_window_handle_by_name(parent: HWND, win_name_buf: &str) -> HWND {
    unsafe {
        let mut child = ptr::null_mut();
        loop {
            child = FindWindowExA(parent, child, ptr::null_mut(), ptr::null_mut());
            if child.is_null() {
                break;
            }

            let mut buf: [u8; MAX_PATH] = [0; MAX_PATH];
            let ret = GetWindowTextA(child, buf.as_mut_ptr() as *mut i8, MAX_PATH as i32) as usize;
            let current_window_name = String::from_utf8_lossy(&buf[..ret]).to_string();

            if current_window_name == win_name_buf {
                return child;
            }

            let found_child = find_child_window_handle_by_name(child, win_name_buf);
            if !found_child.is_null() {
                return found_child;
            }
        }
        ptr::null_mut()
    }
}

// 遍历顶级窗口，然后对每个顶级窗口调用 find_child_window_handle_by_name，以查找指定名称的窗口句柄。
fn find_window_handle_by_name(win_name_buf: &str) -> HWND {
    unsafe {
        let mut child = ptr::null_mut();
        loop {
            child = FindWindowExA(ptr::null_mut(), child, ptr::null_mut(), ptr::null_mut());
            if child.is_null() {
                break;
            }

            let mut buf: [u8; MAX_PATH] = [0; MAX_PATH];
            let ret = GetWindowTextA(child, buf.as_mut_ptr() as *mut i8, MAX_PATH as i32) as usize;
            let current_window_name = String::from_utf8_lossy(&buf[..ret]).to_string();

            if current_window_name == win_name_buf {
                return child;
            }

            let found_child = find_child_window_handle_by_name(child, win_name_buf);
            if !found_child.is_null() {
                return found_child;
            }
        }
        ptr::null_mut()
    }
}

fn main() {
    unsafe {
        let desktop_name_buf = "FolderView";
        let hwnd = find_window_handle_by_name(desktop_name_buf);

        if !hwnd.is_null() {
            if IsWindowVisible(hwnd) != 0 { // 如果窗口可见
                ShowWindow(hwnd, SW_HIDE); // 则隐藏
            } else { // 如果窗口不可见
                ShowWindow(hwnd, SW_SHOW); // 则显示
            }
        } else {
            println!("无法找到桌面窗口句柄！");
        }
    }
}
