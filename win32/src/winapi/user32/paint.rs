use super::*;
use crate::winapi::{
    gdi32::{self, COLORREF, HGDIOBJ},
    stack_args::FromArg,
};

const TRACE_CONTEXT: &'static str = "user32/paint";

#[win32_derive::dllexport]
pub fn InvalidateRect(
    machine: &mut Machine,
    hWnd: HWND,
    lpRect: Option<&RECT>,
    bErase: bool,
) -> bool {
    let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
    window.need_paint = true;
    true // success
}

#[derive(Debug)]
#[repr(C)]
pub struct PAINTSTRUCT {
    hdc: HDC,
    fErase: u32,
    rcPaint: RECT,
    fRestore: u32,
    fIncUpdate: u32,
    rgbReserved: [u8; 32],
}
unsafe impl memory::Pod for PAINTSTRUCT {}

#[win32_derive::dllexport]
pub fn BeginPaint(machine: &mut Machine, hWnd: HWND, lpPaint: Option<&mut PAINTSTRUCT>) -> HDC {
    let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
    let rect = RECT {
        left: 0,
        top: 0,
        right: window.width as i32,
        bottom: window.height as i32,
    };

    let hdc = window.hdc;
    if let Some(hbrush) = window.wndclass.background.to_option() {
        if let gdi32::Object::Brush(brush) = machine.state.gdi32.objects.get(hbrush).unwrap() {
            gdi32::fill_rect(machine, hdc, &rect, brush.color);
        }
    }
    *lpPaint.unwrap() = PAINTSTRUCT {
        hdc,
        fErase: 1, // todo
        rcPaint: rect,
        fRestore: 0,          // reserved
        fIncUpdate: 0,        // reserved
        rgbReserved: [0; 32], // reserved
    };
    hdc
}

#[win32_derive::dllexport]
pub fn EndPaint(machine: &mut Machine, hWnd: HWND, lpPaint: Option<&PAINTSTRUCT>) -> bool {
    let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
    window.flush_pixels();
    machine
        .state
        .user32
        .windows
        .get_mut(hWnd)
        .unwrap()
        .need_paint = false;
    true
}

/// COLOR_xxx for GetSysColor etc.
#[derive(Debug, Eq, PartialEq, win32_derive::TryFromEnum)]
pub enum COLOR {
    WINDOW = 5,
}

#[derive(Debug)]
pub enum BrushOrColor {
    Color(COLOR),
    Brush(HBRUSH),
}

impl<'a> FromArg<'a> for BrushOrColor {
    unsafe fn from_arg(_mem: memory::Mem<'a>, arg: u32) -> Self {
        if arg > 0 && arg < HGDIOBJ::lowest_value() {
            BrushOrColor::Color(COLOR::try_from(arg - 1).unwrap())
        } else {
            BrushOrColor::Brush(HBRUSH::from_raw(arg))
        }
    }
}

impl BrushOrColor {
    pub fn to_brush(&self, machine: &mut Machine) -> HGDIOBJ {
        match self {
            BrushOrColor::Brush(hbr) => *hbr,
            BrushOrColor::Color(c) => {
                let color = match c {
                    COLOR::WINDOW => COLORREF((0xc0, 0xc0, 0xc0)),
                };
                machine
                    .state
                    .gdi32
                    .objects
                    .add(gdi32::Object::Brush(gdi32::Brush { color }))
            }
        }
    }
}

#[win32_derive::dllexport]
pub fn FillRect(machine: &mut Machine, hDC: HDC, lprc: Option<&RECT>, hbr: BrushOrColor) -> bool {
    let brush = hbr.to_brush(machine);
    let color = match machine.state.gdi32.objects.get(brush).unwrap() {
        gdi32::Object::Brush(brush) => brush.color,
        _ => unimplemented!(),
    };
    let rect = lprc.unwrap();
    gdi32::fill_rect(machine, hDC, rect, color);
    true
}

#[win32_derive::dllexport]
pub fn FrameRect(_machine: &mut Machine, hDC: HDC, lprc: Option<&RECT>, hbr: HBRUSH) -> bool {
    // TODO
    true
}
