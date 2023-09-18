#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[doc = r" Generated code, do not edit."]
use crate::shims;
pub struct Symbol {
    pub ordinal: Option<usize>,
    pub shim: shims::Shim,
}
pub struct BuiltinDLL {
    pub file_name: &'static str,
    pub exports: &'static [Symbol],
}
pub mod bass {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::bass::*;
        pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, esp: u32) -> u32 {
            let arg1 = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::bass::BASS_ChannelGetPosition(machine, arg1).to_raw()
        }
        pub unsafe fn BASS_Init(machine: &mut Machine, esp: u32) -> u32 {
            let arg1 = <u32>::from_stack(machine.mem(), esp + 4u32);
            let arg2 = <u32>::from_stack(machine.mem(), esp + 8u32);
            let arg3 = <u32>::from_stack(machine.mem(), esp + 12u32);
            let arg4 = <u32>::from_stack(machine.mem(), esp + 16u32);
            winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4).to_raw()
        }
        pub unsafe fn BASS_MusicLoad(machine: &mut Machine, esp: u32) -> u32 {
            let arg1 = <u32>::from_stack(machine.mem(), esp + 4u32);
            let arg2 = <u32>::from_stack(machine.mem(), esp + 8u32);
            let arg3 = <u32>::from_stack(machine.mem(), esp + 12u32);
            let arg4 = <u32>::from_stack(machine.mem(), esp + 16u32);
            let arg5 = <u32>::from_stack(machine.mem(), esp + 20u32);
            winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5).to_raw()
        }
        pub unsafe fn BASS_MusicPlay(machine: &mut Machine, esp: u32) -> u32 {
            let arg1 = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::bass::BASS_MusicPlay(machine, arg1).to_raw()
        }
        pub unsafe fn BASS_Start(machine: &mut Machine, esp: u32) -> u32 {
            winapi::bass::BASS_Start(machine).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const BASS_ChannelGetPosition: Shim = Shim {
            name: "BASS_ChannelGetPosition",
            func: impls::BASS_ChannelGetPosition,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const BASS_Init: Shim = Shim {
            name: "BASS_Init",
            func: impls::BASS_Init,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const BASS_MusicLoad: Shim = Shim {
            name: "BASS_MusicLoad",
            func: impls::BASS_MusicLoad,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const BASS_MusicPlay: Shim = Shim {
            name: "BASS_MusicPlay",
            func: impls::BASS_MusicPlay,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const BASS_Start: Shim = Shim {
            name: "BASS_Start",
            func: impls::BASS_Start,
            stack_consumed: 4u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 5usize] = [
        Symbol {
            ordinal: None,
            shim: shims::BASS_ChannelGetPosition,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_Init,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_MusicLoad,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_MusicPlay,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_Start,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "bass.dll",
        exports: &EXPORTS,
    };
}
pub mod ddraw {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::ddraw::*;
        pub unsafe fn DirectDrawCreate(machine: &mut Machine, esp: u32) -> u32 {
            let lpGuid = <u32>::from_stack(machine.mem(), esp + 4u32);
            let lplpDD = <u32>::from_stack(machine.mem(), esp + 8u32);
            let pUnkOuter = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, esp: u32) -> u32 {
            let lpGuid = <u32>::from_stack(machine.mem(), esp + 4u32);
            let lplpDD = <u32>::from_stack(machine.mem(), esp + 8u32);
            let iid = <u32>::from_stack(machine.mem(), esp + 12u32);
            let pUnkOuter = <u32>::from_stack(machine.mem(), esp + 16u32);
            winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const DirectDrawCreate: Shim = Shim {
            name: "DirectDrawCreate",
            func: impls::DirectDrawCreate,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const DirectDrawCreateEx: Shim = Shim {
            name: "DirectDrawCreateEx",
            func: impls::DirectDrawCreateEx,
            stack_consumed: 20u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 2usize] = [
        Symbol {
            ordinal: None,
            shim: shims::DirectDrawCreate,
        },
        Symbol {
            ordinal: None,
            shim: shims::DirectDrawCreateEx,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        exports: &EXPORTS,
    };
}
pub mod dsound {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::dsound::*;
        pub unsafe fn DirectSoundCreate(machine: &mut Machine, esp: u32) -> u32 {
            let _lpGuid = <u32>::from_stack(machine.mem(), esp + 4u32);
            let ppDS = <u32>::from_stack(machine.mem(), esp + 8u32);
            let _pUnkOuter = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::dsound::DirectSoundCreate(machine, _lpGuid, ppDS, _pUnkOuter).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const DirectSoundCreate: Shim = Shim {
            name: "DirectSoundCreate",
            func: impls::DirectSoundCreate,
            stack_consumed: 16u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        ordinal: Some(1usize),
        shim: shims::DirectSoundCreate,
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dsound.dll",
        exports: &EXPORTS,
    };
}
pub mod gdi32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::gdi32::*;
        pub unsafe fn BitBlt(machine: &mut Machine, esp: u32) -> u32 {
            let hdc = <u32>::from_stack(machine.mem(), esp + 4u32);
            let x = <u32>::from_stack(machine.mem(), esp + 8u32);
            let y = <u32>::from_stack(machine.mem(), esp + 12u32);
            let cx = <u32>::from_stack(machine.mem(), esp + 16u32);
            let cy = <u32>::from_stack(machine.mem(), esp + 20u32);
            let hdcSrc = <u32>::from_stack(machine.mem(), esp + 24u32);
            let x1 = <u32>::from_stack(machine.mem(), esp + 28u32);
            let y1 = <u32>::from_stack(machine.mem(), esp + 32u32);
            let rop = <u32>::from_stack(machine.mem(), esp + 36u32);
            winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop).to_raw()
        }
        pub unsafe fn CreateCompatibleDC(machine: &mut Machine, esp: u32) -> u32 {
            let hdc = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::gdi32::CreateCompatibleDC(machine, hdc).to_raw()
        }
        pub unsafe fn CreateDIBSection(machine: &mut Machine, esp: u32) -> u32 {
            let hdc = <HDC>::from_stack(machine.mem(), esp + 4u32);
            let pbmi = <Option<&BITMAPINFO>>::from_stack(machine.mem(), esp + 8u32);
            let usage = <u32>::from_stack(machine.mem(), esp + 12u32);
            let ppvBits = <Option<&mut u32>>::from_stack(machine.mem(), esp + 16u32);
            let hSection = <u32>::from_stack(machine.mem(), esp + 20u32);
            let offset = <u32>::from_stack(machine.mem(), esp + 24u32);
            winapi::gdi32::CreateDIBSection(machine, hdc, pbmi, usage, ppvBits, hSection, offset)
                .to_raw()
        }
        pub unsafe fn DeleteDC(machine: &mut Machine, esp: u32) -> u32 {
            let hdc = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::gdi32::DeleteDC(machine, hdc).to_raw()
        }
        pub unsafe fn GetObjectA(machine: &mut Machine, esp: u32) -> u32 {
            let handle = <u32>::from_stack(machine.mem(), esp + 4u32);
            let _bytes = <u32>::from_stack(machine.mem(), esp + 8u32);
            let _out = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::gdi32::GetObjectA(machine, handle, _bytes, _out).to_raw()
        }
        pub unsafe fn GetStockObject(machine: &mut Machine, esp: u32) -> u32 {
            let _i = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::gdi32::GetStockObject(machine, _i).to_raw()
        }
        pub unsafe fn SelectObject(machine: &mut Machine, esp: u32) -> u32 {
            let hdc = <u32>::from_stack(machine.mem(), esp + 4u32);
            let hGdiObj = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::gdi32::SelectObject(machine, hdc, hGdiObj).to_raw()
        }
        pub unsafe fn StretchBlt(machine: &mut Machine, esp: u32) -> u32 {
            let hdcDest = <u32>::from_stack(machine.mem(), esp + 4u32);
            let xDest = <u32>::from_stack(machine.mem(), esp + 8u32);
            let yDest = <u32>::from_stack(machine.mem(), esp + 12u32);
            let wDest = <u32>::from_stack(machine.mem(), esp + 16u32);
            let hDest = <u32>::from_stack(machine.mem(), esp + 20u32);
            let hdcSrc = <u32>::from_stack(machine.mem(), esp + 24u32);
            let xSrc = <u32>::from_stack(machine.mem(), esp + 28u32);
            let ySrc = <u32>::from_stack(machine.mem(), esp + 32u32);
            let wSrc = <u32>::from_stack(machine.mem(), esp + 36u32);
            let hSrc = <u32>::from_stack(machine.mem(), esp + 40u32);
            let rop = <u32>::from_stack(machine.mem(), esp + 44u32);
            winapi::gdi32::StretchBlt(
                machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
            )
            .to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const BitBlt: Shim = Shim {
            name: "BitBlt",
            func: impls::BitBlt,
            stack_consumed: 40u32,
            is_async: false,
        };
        pub const CreateCompatibleDC: Shim = Shim {
            name: "CreateCompatibleDC",
            func: impls::CreateCompatibleDC,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const CreateDIBSection: Shim = Shim {
            name: "CreateDIBSection",
            func: impls::CreateDIBSection,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const DeleteDC: Shim = Shim {
            name: "DeleteDC",
            func: impls::DeleteDC,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetObjectA: Shim = Shim {
            name: "GetObjectA",
            func: impls::GetObjectA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetStockObject: Shim = Shim {
            name: "GetStockObject",
            func: impls::GetStockObject,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SelectObject: Shim = Shim {
            name: "SelectObject",
            func: impls::SelectObject,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const StretchBlt: Shim = Shim {
            name: "StretchBlt",
            func: impls::StretchBlt,
            stack_consumed: 48u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 8usize] = [
        Symbol {
            ordinal: None,
            shim: shims::BitBlt,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateCompatibleDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateDIBSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetObjectA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStockObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::SelectObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::StretchBlt,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "gdi32.dll",
        exports: &EXPORTS,
    };
}
pub mod kernel32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::kernel32::*;
        pub unsafe fn CreateEventA(machine: &mut Machine, esp: u32) -> u32 {
            let lpEventAttributes = <u32>::from_stack(machine.mem(), esp + 4u32);
            let bManualReset = <bool>::from_stack(machine.mem(), esp + 8u32);
            let bInitialState = <bool>::from_stack(machine.mem(), esp + 12u32);
            let lpName = <Option<&str>>::from_stack(machine.mem(), esp + 16u32);
            winapi::kernel32::CreateEventA(
                machine,
                lpEventAttributes,
                bManualReset,
                bInitialState,
                lpName,
            )
            .to_raw()
        }
        pub unsafe fn CreateFileA(machine: &mut Machine, esp: u32) -> u32 {
            let lpFileName = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(machine.mem(), esp + 8u32);
            let dwShareMode = <u32>::from_stack(machine.mem(), esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(machine.mem(), esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(machine.mem(), esp + 20u32);
            let dwFlagsAndAttributes = <u32>::from_stack(machine.mem(), esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(machine.mem(), esp + 28u32);
            winapi::kernel32::CreateFileA(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
        }
        pub unsafe fn CreateFileW(machine: &mut Machine, esp: u32) -> u32 {
            let lpFileName = <Option<Str16>>::from_stack(machine.mem(), esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(machine.mem(), esp + 8u32);
            let dwShareMode = <u32>::from_stack(machine.mem(), esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(machine.mem(), esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(machine.mem(), esp + 20u32);
            let dwFlagsAndAttributes = <u32>::from_stack(machine.mem(), esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(machine.mem(), esp + 28u32);
            winapi::kernel32::CreateFileW(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
        }
        pub unsafe fn CreateThread(machine: &mut Machine, esp: u32) -> u32 {
            let lpThreadAttributes = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwStackSize = <u32>::from_stack(machine.mem(), esp + 8u32);
            let lpStartAddress = <u32>::from_stack(machine.mem(), esp + 12u32);
            let lpParameter = <u32>::from_stack(machine.mem(), esp + 16u32);
            let dwCreationFlags = <u32>::from_stack(machine.mem(), esp + 20u32);
            let lpThreadId = <u32>::from_stack(machine.mem(), esp + 24u32);
            winapi::kernel32::CreateThread(
                machine,
                lpThreadAttributes,
                dwStackSize,
                lpStartAddress,
                lpParameter,
                dwCreationFlags,
                lpThreadId,
            )
            .to_raw()
        }
        pub unsafe fn DeleteCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::DeleteCriticalSection(machine, _lpCriticalSection).to_raw()
        }
        pub unsafe fn EnterCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::EnterCriticalSection(machine, _lpCriticalSection).to_raw()
        }
        pub unsafe fn ExitProcess(machine: &mut Machine, esp: u32) -> u32 {
            let uExitCode = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::ExitProcess(machine, uExitCode).to_raw()
        }
        pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, esp: u32) -> u32 {
            let _penv = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::FreeEnvironmentStringsA(machine, _penv).to_raw()
        }
        pub unsafe fn GetACP(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetACP(machine).to_raw()
        }
        pub unsafe fn GetCPInfo(machine: &mut Machine, esp: u32) -> u32 {
            let _CodePage = <u32>::from_stack(machine.mem(), esp + 4u32);
            let _lpCPInfo = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo).to_raw()
        }
        pub unsafe fn GetCommandLineA(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetCommandLineA(machine).to_raw()
        }
        pub unsafe fn GetCommandLineW(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetCommandLineW(machine).to_raw()
        }
        pub unsafe fn GetCurrentProcessId(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetCurrentProcessId(machine).to_raw()
        }
        pub unsafe fn GetCurrentThread(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetCurrentThread(machine).to_raw()
        }
        pub unsafe fn GetCurrentThreadId(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetCurrentThreadId(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetEnvironmentStrings(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetEnvironmentStringsW(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, esp: u32) -> u32 {
            let name = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
            let buf = <ArrayWithSize<u8>>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::GetEnvironmentVariableA(machine, name, buf).to_raw()
        }
        pub unsafe fn GetFileType(machine: &mut Machine, esp: u32) -> u32 {
            let hFile = <HFILE>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::GetFileType(machine, hFile).to_raw()
        }
        pub unsafe fn GetLastError(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetLastError(machine).to_raw()
        }
        pub unsafe fn GetModuleFileNameA(machine: &mut Machine, esp: u32) -> u32 {
            let hModule = <HMODULE>::from_stack(machine.mem(), esp + 4u32);
            let filename = <ArrayWithSizeMut<u8>>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::GetModuleFileNameA(machine, hModule, filename).to_raw()
        }
        pub unsafe fn GetModuleFileNameW(machine: &mut Machine, esp: u32) -> u32 {
            let hModule = <HMODULE>::from_stack(machine.mem(), esp + 4u32);
            let _lpFilename = <u32>::from_stack(machine.mem(), esp + 8u32);
            let _nSize = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize).to_raw()
        }
        pub unsafe fn GetModuleHandleA(machine: &mut Machine, esp: u32) -> u32 {
            let lpModuleName = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::GetModuleHandleA(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetModuleHandleExW(machine: &mut Machine, esp: u32) -> u32 {
            let dwFlags = <u32>::from_stack(machine.mem(), esp + 4u32);
            let lpModuleName = <Option<Str16>>::from_stack(machine.mem(), esp + 8u32);
            let hModule = <Option<&mut HMODULE>>::from_stack(machine.mem(), esp + 12u32);
            winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule).to_raw()
        }
        pub unsafe fn GetModuleHandleW(machine: &mut Machine, esp: u32) -> u32 {
            let lpModuleName = <Option<Str16>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::GetModuleHandleW(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetProcAddress(machine: &mut Machine, esp: u32) -> u32 {
            let hModule = <HMODULE>::from_stack(machine.mem(), esp + 4u32);
            let lpProcName = <Option<&str>>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::GetProcAddress(machine, hModule, lpProcName).to_raw()
        }
        pub unsafe fn GetProcessHeap(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetProcessHeap(machine).to_raw()
        }
        pub unsafe fn GetStartupInfoA(machine: &mut Machine, esp: u32) -> u32 {
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStartupInfoW(machine: &mut Machine, esp: u32) -> u32 {
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
            let nStdHandle = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::GetStdHandle(machine, nStdHandle).to_raw()
        }
        pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let _time = <Option<&mut FILETIME>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::GetSystemTimeAsFileTime(machine, _time).to_raw()
        }
        pub unsafe fn GetTickCount(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetTickCount(machine).to_raw()
        }
        pub unsafe fn GetVersion(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::GetVersion(machine).to_raw()
        }
        pub unsafe fn GetVersionExA(machine: &mut Machine, esp: u32) -> u32 {
            let lpVersionInformation =
                <Option<&mut OSVERSIONINFO>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::GetVersionExA(machine, lpVersionInformation).to_raw()
        }
        pub unsafe fn GlobalAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let uFlags = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwBytes = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::GlobalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn GlobalFree(machine: &mut Machine, esp: u32) -> u32 {
            let hMem = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::GlobalFree(machine, hMem).to_raw()
        }
        pub unsafe fn HeapAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwFlags = <Result<HeapAllocFlags, u32>>::from_stack(machine.mem(), esp + 8u32);
            let dwBytes = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes).to_raw()
        }
        pub unsafe fn HeapCreate(machine: &mut Machine, esp: u32) -> u32 {
            let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(machine.mem(), esp + 4u32);
            let dwInitialSize = <u32>::from_stack(machine.mem(), esp + 8u32);
            let dwMaximumSize = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize).to_raw()
        }
        pub unsafe fn HeapDestroy(machine: &mut Machine, esp: u32) -> u32 {
            let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::HeapDestroy(machine, hHeap).to_raw()
        }
        pub unsafe fn HeapFree(machine: &mut Machine, esp: u32) -> u32 {
            let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwFlags = <u32>::from_stack(machine.mem(), esp + 8u32);
            let lpMem = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn HeapReAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwFlags = <u32>::from_stack(machine.mem(), esp + 8u32);
            let lpMem = <u32>::from_stack(machine.mem(), esp + 12u32);
            let dwBytes = <u32>::from_stack(machine.mem(), esp + 16u32);
            winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes).to_raw()
        }
        pub unsafe fn HeapSize(machine: &mut Machine, esp: u32) -> u32 {
            let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwFlags = <u32>::from_stack(machine.mem(), esp + 8u32);
            let lpMem = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn InitializeCriticalSectionAndSpinCount(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + 4u32);
            let _dwSpinCount = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::InitializeCriticalSectionAndSpinCount(
                machine,
                _lpCriticalSection,
                _dwSpinCount,
            )
            .to_raw()
        }
        pub unsafe fn InitializeSListHead(machine: &mut Machine, esp: u32) -> u32 {
            let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::InitializeSListHead(machine, ListHead).to_raw()
        }
        pub unsafe fn InterlockedIncrement(machine: &mut Machine, esp: u32) -> u32 {
            let addend = <Option<&mut u32>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::InterlockedIncrement(machine, addend).to_raw()
        }
        pub unsafe fn IsBadReadPtr(machine: &mut Machine, esp: u32) -> u32 {
            let lp = <u32>::from_stack(machine.mem(), esp + 4u32);
            let ucb = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::IsBadReadPtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsBadWritePtr(machine: &mut Machine, esp: u32) -> u32 {
            let lp = <u32>::from_stack(machine.mem(), esp + 4u32);
            let ucb = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::IsBadWritePtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsDebuggerPresent(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::IsDebuggerPresent(machine).to_raw()
        }
        pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, esp: u32) -> u32 {
            let feature = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::IsProcessorFeaturePresent(machine, feature).to_raw()
        }
        pub unsafe fn IsValidCodePage(machine: &mut Machine, esp: u32) -> u32 {
            let CodePage = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::IsValidCodePage(machine, CodePage).to_raw()
        }
        pub unsafe fn LeaveCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::LeaveCriticalSection(machine, _lpCriticalSection).to_raw()
        }
        pub unsafe fn LoadLibraryA(machine: &mut Machine, esp: u32) -> u32 {
            let filename = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::LoadLibraryA(machine, filename).to_raw()
        }
        pub unsafe fn LoadLibraryExW(machine: &mut Machine, esp: u32) -> u32 {
            let lpLibFileName = <Option<Str16>>::from_stack(machine.mem(), esp + 4u32);
            let hFile = <HFILE>::from_stack(machine.mem(), esp + 8u32);
            let dwFlags = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags).to_raw()
        }
        pub unsafe fn MultiByteToWideChar(machine: &mut Machine, esp: u32) -> u32 {
            let CodePage = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwFlags = <u32>::from_stack(machine.mem(), esp + 8u32);
            let lpMultiByteStr = <u32>::from_stack(machine.mem(), esp + 12u32);
            let cbMultiByte = <i32>::from_stack(machine.mem(), esp + 16u32);
            let lpWideCharStr = <ArrayWithSizeMut<u16>>::from_stack(machine.mem(), esp + 20u32);
            winapi::kernel32::MultiByteToWideChar(
                machine,
                CodePage,
                dwFlags,
                lpMultiByteStr,
                cbMultiByte,
                lpWideCharStr,
            )
            .to_raw()
        }
        pub unsafe fn NtCurrentTeb(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::NtCurrentTeb(machine).to_raw()
        }
        pub unsafe fn OutputDebugStringA(machine: &mut Machine, esp: u32) -> u32 {
            let msg = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::OutputDebugStringA(machine, msg).to_raw()
        }
        pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, esp: u32) -> u32 {
            let lpPerformanceCount =
                <Option<&mut LARGE_INTEGER>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount).to_raw()
        }
        pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, esp: u32) -> u32 {
            let lpFrequency = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency).to_raw()
        }
        pub unsafe fn ReadFile(machine: &mut Machine, esp: u32) -> u32 {
            let hFile = <HFILE>::from_stack(machine.mem(), esp + 4u32);
            let lpBuffer = <ArrayWithSizeMut<u8>>::from_stack(machine.mem(), esp + 8u32);
            let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(machine.mem(), esp + 16u32);
            let lpOverlapped = <u32>::from_stack(machine.mem(), esp + 20u32);
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped)
                .to_raw()
        }
        pub unsafe fn SetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let hEvent = <HANDLE<()>>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::SetEvent(machine, hEvent).to_raw()
        }
        pub unsafe fn SetFilePointer(machine: &mut Machine, esp: u32) -> u32 {
            let hFile = <HFILE>::from_stack(machine.mem(), esp + 4u32);
            let lDistanceToMove = <u32>::from_stack(machine.mem(), esp + 8u32);
            let lpDistanceToMoveHigh = <Option<&mut u32>>::from_stack(machine.mem(), esp + 12u32);
            let dwMoveMethod = <u32>::from_stack(machine.mem(), esp + 16u32);
            winapi::kernel32::SetFilePointer(
                machine,
                hFile,
                lDistanceToMove,
                lpDistanceToMoveHigh,
                dwMoveMethod,
            )
            .to_raw()
        }
        pub unsafe fn SetHandleCount(machine: &mut Machine, esp: u32) -> u32 {
            let uNumber = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::SetHandleCount(machine, uNumber).to_raw()
        }
        pub unsafe fn SetLastError(machine: &mut Machine, esp: u32) -> u32 {
            let dwErrCode = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::SetLastError(machine, dwErrCode).to_raw()
        }
        pub unsafe fn SetPriorityClass(machine: &mut Machine, esp: u32) -> u32 {
            let hProcess = <HANDLE<()>>::from_stack(machine.mem(), esp + 4u32);
            let dwPriorityClass = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::SetPriorityClass(machine, hProcess, dwPriorityClass).to_raw()
        }
        pub unsafe fn SetThreadPriority(machine: &mut Machine, esp: u32) -> u32 {
            let _hThread = <u32>::from_stack(machine.mem(), esp + 4u32);
            let _nPriority = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::SetThreadPriority(machine, _hThread, _nPriority).to_raw()
        }
        pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let _lpTopLevelExceptionFilter = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter)
                .to_raw()
        }
        pub unsafe fn Sleep(machine: &mut Machine, esp: u32) -> u32 {
            let dwMilliseconds = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::Sleep(machine, dwMilliseconds).to_raw()
        }
        pub unsafe fn TlsAlloc(machine: &mut Machine, esp: u32) -> u32 {
            winapi::kernel32::TlsAlloc(machine).to_raw()
        }
        pub unsafe fn TlsFree(machine: &mut Machine, esp: u32) -> u32 {
            let dwTlsIndex = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::TlsFree(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsGetValue(machine: &mut Machine, esp: u32) -> u32 {
            let dwTlsIndex = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::TlsGetValue(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsSetValue(machine: &mut Machine, esp: u32) -> u32 {
            let dwTlsIndex = <u32>::from_stack(machine.mem(), esp + 4u32);
            let lpTlsValue = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue).to_raw()
        }
        pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let _exceptionInfo = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo).to_raw()
        }
        pub unsafe fn VirtualAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let lpAddress = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwSize = <u32>::from_stack(machine.mem(), esp + 8u32);
            let _flAllocationType = <u32>::from_stack(machine.mem(), esp + 12u32);
            let _flProtec = <u32>::from_stack(machine.mem(), esp + 16u32);
            winapi::kernel32::VirtualAlloc(machine, lpAddress, dwSize, _flAllocationType, _flProtec)
                .to_raw()
        }
        pub unsafe fn VirtualFree(machine: &mut Machine, esp: u32) -> u32 {
            let lpAddress = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwSize = <u32>::from_stack(machine.mem(), esp + 8u32);
            let dwFreeType = <u32>::from_stack(machine.mem(), esp + 12u32);
            winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType).to_raw()
        }
        pub unsafe fn VirtualProtect(machine: &mut Machine, esp: u32) -> u32 {
            let lpAddress = <u32>::from_stack(machine.mem(), esp + 4u32);
            let dwSize = <u32>::from_stack(machine.mem(), esp + 8u32);
            let flNewProtect = <u32>::from_stack(machine.mem(), esp + 12u32);
            let lpflOldProtect = <Option<&mut u32>>::from_stack(machine.mem(), esp + 16u32);
            winapi::kernel32::VirtualProtect(
                machine,
                lpAddress,
                dwSize,
                flNewProtect,
                lpflOldProtect,
            )
            .to_raw()
        }
        pub unsafe fn WaitForSingleObject(machine: &mut Machine, esp: u32) -> u32 {
            let hHandle = <HANDLE<()>>::from_stack(machine.mem(), esp + 4u32);
            let dwMilliseconds = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::kernel32::WaitForSingleObject(machine, hHandle, dwMilliseconds).to_raw()
        }
        pub unsafe fn WriteConsoleA(machine: &mut Machine, esp: u32) -> u32 {
            let hConsoleOutput = <HANDLE<()>>::from_stack(machine.mem(), esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(machine.mem(), esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(machine.mem(), esp + 16u32);
            let lpReserved = <u32>::from_stack(machine.mem(), esp + 20u32);
            winapi::kernel32::WriteConsoleA(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteConsoleW(machine: &mut Machine, esp: u32) -> u32 {
            let hConsoleOutput = <HFILE>::from_stack(machine.mem(), esp + 4u32);
            let lpBuffer = <ArrayWithSize<u16>>::from_stack(machine.mem(), esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(machine.mem(), esp + 16u32);
            let _lpReserved = <u32>::from_stack(machine.mem(), esp + 20u32);
            winapi::kernel32::WriteConsoleW(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                _lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteFile(machine: &mut Machine, esp: u32) -> u32 {
            let hFile = <HFILE>::from_stack(machine.mem(), esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(machine.mem(), esp + 8u32);
            let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(machine.mem(), esp + 16u32);
            let lpOverlapped = <u32>::from_stack(machine.mem(), esp + 20u32);
            winapi::kernel32::WriteFile(
                machine,
                hFile,
                lpBuffer,
                lpNumberOfBytesWritten,
                lpOverlapped,
            )
            .to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const CreateEventA: Shim = Shim {
            name: "CreateEventA",
            func: impls::CreateEventA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const CreateFileA: Shim = Shim {
            name: "CreateFileA",
            func: impls::CreateFileA,
            stack_consumed: 32u32,
            is_async: false,
        };
        pub const CreateFileW: Shim = Shim {
            name: "CreateFileW",
            func: impls::CreateFileW,
            stack_consumed: 32u32,
            is_async: false,
        };
        pub const CreateThread: Shim = Shim {
            name: "CreateThread",
            func: impls::CreateThread,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const DeleteCriticalSection: Shim = Shim {
            name: "DeleteCriticalSection",
            func: impls::DeleteCriticalSection,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const EnterCriticalSection: Shim = Shim {
            name: "EnterCriticalSection",
            func: impls::EnterCriticalSection,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const ExitProcess: Shim = Shim {
            name: "ExitProcess",
            func: impls::ExitProcess,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FreeEnvironmentStringsA: Shim = Shim {
            name: "FreeEnvironmentStringsA",
            func: impls::FreeEnvironmentStringsA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetACP: Shim = Shim {
            name: "GetACP",
            func: impls::GetACP,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetCPInfo: Shim = Shim {
            name: "GetCPInfo",
            func: impls::GetCPInfo,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetCommandLineA: Shim = Shim {
            name: "GetCommandLineA",
            func: impls::GetCommandLineA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetCommandLineW: Shim = Shim {
            name: "GetCommandLineW",
            func: impls::GetCommandLineW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetCurrentProcessId: Shim = Shim {
            name: "GetCurrentProcessId",
            func: impls::GetCurrentProcessId,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetCurrentThread: Shim = Shim {
            name: "GetCurrentThread",
            func: impls::GetCurrentThread,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetCurrentThreadId: Shim = Shim {
            name: "GetCurrentThreadId",
            func: impls::GetCurrentThreadId,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetEnvironmentStrings: Shim = Shim {
            name: "GetEnvironmentStrings",
            func: impls::GetEnvironmentStrings,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetEnvironmentStringsW: Shim = Shim {
            name: "GetEnvironmentStringsW",
            func: impls::GetEnvironmentStringsW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetEnvironmentVariableA: Shim = Shim {
            name: "GetEnvironmentVariableA",
            func: impls::GetEnvironmentVariableA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetFileType: Shim = Shim {
            name: "GetFileType",
            func: impls::GetFileType,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetLastError: Shim = Shim {
            name: "GetLastError",
            func: impls::GetLastError,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetModuleFileNameA: Shim = Shim {
            name: "GetModuleFileNameA",
            func: impls::GetModuleFileNameA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetModuleFileNameW: Shim = Shim {
            name: "GetModuleFileNameW",
            func: impls::GetModuleFileNameW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetModuleHandleA: Shim = Shim {
            name: "GetModuleHandleA",
            func: impls::GetModuleHandleA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetModuleHandleExW: Shim = Shim {
            name: "GetModuleHandleExW",
            func: impls::GetModuleHandleExW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetModuleHandleW: Shim = Shim {
            name: "GetModuleHandleW",
            func: impls::GetModuleHandleW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetProcAddress: Shim = Shim {
            name: "GetProcAddress",
            func: impls::GetProcAddress,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetProcessHeap: Shim = Shim {
            name: "GetProcessHeap",
            func: impls::GetProcessHeap,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetStartupInfoA: Shim = Shim {
            name: "GetStartupInfoA",
            func: impls::GetStartupInfoA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetStartupInfoW: Shim = Shim {
            name: "GetStartupInfoW",
            func: impls::GetStartupInfoW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetStdHandle: Shim = Shim {
            name: "GetStdHandle",
            func: impls::GetStdHandle,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetSystemTimeAsFileTime: Shim = Shim {
            name: "GetSystemTimeAsFileTime",
            func: impls::GetSystemTimeAsFileTime,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetTickCount: Shim = Shim {
            name: "GetTickCount",
            func: impls::GetTickCount,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetVersion: Shim = Shim {
            name: "GetVersion",
            func: impls::GetVersion,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetVersionExA: Shim = Shim {
            name: "GetVersionExA",
            func: impls::GetVersionExA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GlobalAlloc: Shim = Shim {
            name: "GlobalAlloc",
            func: impls::GlobalAlloc,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GlobalFree: Shim = Shim {
            name: "GlobalFree",
            func: impls::GlobalFree,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const HeapAlloc: Shim = Shim {
            name: "HeapAlloc",
            func: impls::HeapAlloc,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const HeapCreate: Shim = Shim {
            name: "HeapCreate",
            func: impls::HeapCreate,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const HeapDestroy: Shim = Shim {
            name: "HeapDestroy",
            func: impls::HeapDestroy,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const HeapFree: Shim = Shim {
            name: "HeapFree",
            func: impls::HeapFree,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const HeapReAlloc: Shim = Shim {
            name: "HeapReAlloc",
            func: impls::HeapReAlloc,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const HeapSize: Shim = Shim {
            name: "HeapSize",
            func: impls::HeapSize,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const InitializeCriticalSectionAndSpinCount: Shim = Shim {
            name: "InitializeCriticalSectionAndSpinCount",
            func: impls::InitializeCriticalSectionAndSpinCount,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InitializeSListHead: Shim = Shim {
            name: "InitializeSListHead",
            func: impls::InitializeSListHead,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const InterlockedIncrement: Shim = Shim {
            name: "InterlockedIncrement",
            func: impls::InterlockedIncrement,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IsBadReadPtr: Shim = Shim {
            name: "IsBadReadPtr",
            func: impls::IsBadReadPtr,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IsBadWritePtr: Shim = Shim {
            name: "IsBadWritePtr",
            func: impls::IsBadWritePtr,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IsDebuggerPresent: Shim = Shim {
            name: "IsDebuggerPresent",
            func: impls::IsDebuggerPresent,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IsProcessorFeaturePresent: Shim = Shim {
            name: "IsProcessorFeaturePresent",
            func: impls::IsProcessorFeaturePresent,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IsValidCodePage: Shim = Shim {
            name: "IsValidCodePage",
            func: impls::IsValidCodePage,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LeaveCriticalSection: Shim = Shim {
            name: "LeaveCriticalSection",
            func: impls::LeaveCriticalSection,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadLibraryA: Shim = Shim {
            name: "LoadLibraryA",
            func: impls::LoadLibraryA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadLibraryExW: Shim = Shim {
            name: "LoadLibraryExW",
            func: impls::LoadLibraryExW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const MultiByteToWideChar: Shim = Shim {
            name: "MultiByteToWideChar",
            func: impls::MultiByteToWideChar,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const NtCurrentTeb: Shim = Shim {
            name: "NtCurrentTeb",
            func: impls::NtCurrentTeb,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const OutputDebugStringA: Shim = Shim {
            name: "OutputDebugStringA",
            func: impls::OutputDebugStringA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const QueryPerformanceCounter: Shim = Shim {
            name: "QueryPerformanceCounter",
            func: impls::QueryPerformanceCounter,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const QueryPerformanceFrequency: Shim = Shim {
            name: "QueryPerformanceFrequency",
            func: impls::QueryPerformanceFrequency,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const ReadFile: Shim = Shim {
            name: "ReadFile",
            func: impls::ReadFile,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const SetEvent: Shim = Shim {
            name: "SetEvent",
            func: impls::SetEvent,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetFilePointer: Shim = Shim {
            name: "SetFilePointer",
            func: impls::SetFilePointer,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const SetHandleCount: Shim = Shim {
            name: "SetHandleCount",
            func: impls::SetHandleCount,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetLastError: Shim = Shim {
            name: "SetLastError",
            func: impls::SetLastError,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetPriorityClass: Shim = Shim {
            name: "SetPriorityClass",
            func: impls::SetPriorityClass,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const SetThreadPriority: Shim = Shim {
            name: "SetThreadPriority",
            func: impls::SetThreadPriority,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const SetUnhandledExceptionFilter: Shim = Shim {
            name: "SetUnhandledExceptionFilter",
            func: impls::SetUnhandledExceptionFilter,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const Sleep: Shim = Shim {
            name: "Sleep",
            func: impls::Sleep,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const TlsAlloc: Shim = Shim {
            name: "TlsAlloc",
            func: impls::TlsAlloc,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const TlsFree: Shim = Shim {
            name: "TlsFree",
            func: impls::TlsFree,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const TlsGetValue: Shim = Shim {
            name: "TlsGetValue",
            func: impls::TlsGetValue,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const TlsSetValue: Shim = Shim {
            name: "TlsSetValue",
            func: impls::TlsSetValue,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const UnhandledExceptionFilter: Shim = Shim {
            name: "UnhandledExceptionFilter",
            func: impls::UnhandledExceptionFilter,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const VirtualAlloc: Shim = Shim {
            name: "VirtualAlloc",
            func: impls::VirtualAlloc,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const VirtualFree: Shim = Shim {
            name: "VirtualFree",
            func: impls::VirtualFree,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const VirtualProtect: Shim = Shim {
            name: "VirtualProtect",
            func: impls::VirtualProtect,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const WaitForSingleObject: Shim = Shim {
            name: "WaitForSingleObject",
            func: impls::WaitForSingleObject,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const WriteConsoleA: Shim = Shim {
            name: "WriteConsoleA",
            func: impls::WriteConsoleA,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const WriteConsoleW: Shim = Shim {
            name: "WriteConsoleW",
            func: impls::WriteConsoleW,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const WriteFile: Shim = Shim {
            name: "WriteFile",
            func: impls::WriteFile,
            stack_consumed: 24u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 79usize] = [
        Symbol {
            ordinal: None,
            shim: shims::CreateEventA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateFileA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateFileW,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateThread,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::EnterCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::ExitProcess,
        },
        Symbol {
            ordinal: None,
            shim: shims::FreeEnvironmentStringsA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetACP,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCPInfo,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCommandLineA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCommandLineW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentProcessId,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentThread,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentThreadId,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentStrings,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentStringsW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentVariableA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileType,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLastError,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleFileNameA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleFileNameW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleHandleA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleHandleExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleHandleW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetProcAddress,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetProcessHeap,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStartupInfoA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStartupInfoW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStdHandle,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemTimeAsFileTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTickCount,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetVersion,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetVersionExA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GlobalAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::GlobalFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapCreate,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapDestroy,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapReAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapSize,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeCriticalSectionAndSpinCount,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeSListHead,
        },
        Symbol {
            ordinal: None,
            shim: shims::InterlockedIncrement,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsBadReadPtr,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsBadWritePtr,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsDebuggerPresent,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsProcessorFeaturePresent,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsValidCodePage,
        },
        Symbol {
            ordinal: None,
            shim: shims::LeaveCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadLibraryA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadLibraryExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::MultiByteToWideChar,
        },
        Symbol {
            ordinal: None,
            shim: shims::NtCurrentTeb,
        },
        Symbol {
            ordinal: None,
            shim: shims::OutputDebugStringA,
        },
        Symbol {
            ordinal: None,
            shim: shims::QueryPerformanceCounter,
        },
        Symbol {
            ordinal: None,
            shim: shims::QueryPerformanceFrequency,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReadFile,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetEvent,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetFilePointer,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetHandleCount,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetLastError,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetPriorityClass,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetThreadPriority,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetUnhandledExceptionFilter,
        },
        Symbol {
            ordinal: None,
            shim: shims::Sleep,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsGetValue,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsSetValue,
        },
        Symbol {
            ordinal: None,
            shim: shims::UnhandledExceptionFilter,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualProtect,
        },
        Symbol {
            ordinal: None,
            shim: shims::WaitForSingleObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::WriteConsoleA,
        },
        Symbol {
            ordinal: None,
            shim: shims::WriteConsoleW,
        },
        Symbol {
            ordinal: None,
            shim: shims::WriteFile,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "kernel32.dll",
        exports: &EXPORTS,
    };
}
pub mod ole32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::ole32::*;
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
    }
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ole32.dll",
        exports: &EXPORTS,
    };
}
pub mod oleaut32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::oleaut32::*;
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
    }
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "oleaut32.dll",
        exports: &EXPORTS,
    };
}
pub mod retrowin32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::retrowin32::*;
        pub unsafe fn retrowin32_callback1(machine: &mut Machine, esp: u32) -> u32 {
            let func = <u32>::from_stack(machine.mem(), esp + 4u32);
            let data = <u32>::from_stack(machine.mem(), esp + 8u32);
            #[cfg(feature = "cpuemu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::retrowin32::retrowin32_callback1(machine, func, data).await;
                    machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
                    machine.x86.cpu.regs.esp += 12u32;
                    machine.x86.cpu.regs.eax = result.to_raw();
                };
                crate::shims::become_async(machine, Box::pin(result));
                0
            }
            #[cfg(not(feature = "cpuemu"))]
            {
                let pin = std::pin::pin!(winapi::retrowin32::retrowin32_callback1(
                    machine, func, data
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const retrowin32_callback1: Shim = Shim {
            name: "retrowin32_callback1",
            func: impls::retrowin32_callback1,
            stack_consumed: 12u32,
            is_async: true,
        };
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        ordinal: None,
        shim: shims::retrowin32_callback1,
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "retrowin32.dll",
        exports: &EXPORTS,
    };
}
pub mod user32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::user32::*;
        pub unsafe fn CreateCursor(machine: &mut Machine, esp: u32) -> u32 {
            let hInst = <u32>::from_stack(machine.mem(), esp + 4u32);
            let xHotSpot = <u32>::from_stack(machine.mem(), esp + 8u32);
            let yHotSpot = <u32>::from_stack(machine.mem(), esp + 12u32);
            let nWidth = <u32>::from_stack(machine.mem(), esp + 16u32);
            let nHeight = <u32>::from_stack(machine.mem(), esp + 20u32);
            let pvANDPlane = <u32>::from_stack(machine.mem(), esp + 24u32);
            let pvXORPlane = <u32>::from_stack(machine.mem(), esp + 28u32);
            winapi::user32::CreateCursor(
                machine, hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
            )
            .to_raw()
        }
        pub unsafe fn CreateWindowExA(machine: &mut Machine, esp: u32) -> u32 {
            let dwExStyle = <u32>::from_stack(machine.mem(), esp + 4u32);
            let lpClassName = <u32>::from_stack(machine.mem(), esp + 8u32);
            let lpWindowName = <Option<&str>>::from_stack(machine.mem(), esp + 12u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(machine.mem(), esp + 16u32);
            let X = <u32>::from_stack(machine.mem(), esp + 20u32);
            let Y = <u32>::from_stack(machine.mem(), esp + 24u32);
            let nWidth = <u32>::from_stack(machine.mem(), esp + 28u32);
            let nHeight = <u32>::from_stack(machine.mem(), esp + 32u32);
            let hWndParent = <HWND>::from_stack(machine.mem(), esp + 36u32);
            let hMenu = <u32>::from_stack(machine.mem(), esp + 40u32);
            let hInstance = <u32>::from_stack(machine.mem(), esp + 44u32);
            let lpParam = <u32>::from_stack(machine.mem(), esp + 48u32);
            #[cfg(feature = "cpuemu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::CreateWindowExA(
                        machine,
                        dwExStyle,
                        lpClassName,
                        lpWindowName,
                        dwStyle,
                        X,
                        Y,
                        nWidth,
                        nHeight,
                        hWndParent,
                        hMenu,
                        hInstance,
                        lpParam,
                    )
                    .await;
                    machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
                    machine.x86.cpu.regs.esp += 52u32;
                    machine.x86.cpu.regs.eax = result.to_raw();
                };
                crate::shims::become_async(machine, Box::pin(result));
                0
            }
            #[cfg(not(feature = "cpuemu"))]
            {
                let pin = std::pin::pin!(winapi::user32::CreateWindowExA(
                    machine,
                    dwExStyle,
                    lpClassName,
                    lpWindowName,
                    dwStyle,
                    X,
                    Y,
                    nWidth,
                    nHeight,
                    hWndParent,
                    hMenu,
                    hInstance,
                    lpParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DefWindowProcA(machine: &mut Machine, esp: u32) -> u32 {
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
            let msg = <Result<WM, u32>>::from_stack(machine.mem(), esp + 8u32);
            let wParam = <u32>::from_stack(machine.mem(), esp + 12u32);
            let lParam = <u32>::from_stack(machine.mem(), esp + 16u32);
            winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam).to_raw()
        }
        pub unsafe fn DestroyWindow(machine: &mut Machine, esp: u32) -> u32 {
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::DestroyWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn DialogBoxParamA(machine: &mut Machine, esp: u32) -> u32 {
            let hInstance = <u32>::from_stack(machine.mem(), esp + 4u32);
            let lpTemplateName = <u32>::from_stack(machine.mem(), esp + 8u32);
            let hWndParent = <HWND>::from_stack(machine.mem(), esp + 12u32);
            let lpDialogFunc = <u32>::from_stack(machine.mem(), esp + 16u32);
            let dwInitParam = <u32>::from_stack(machine.mem(), esp + 20u32);
            winapi::user32::DialogBoxParamA(
                machine,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DispatchMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let lpMsg = <Option<&MSG>>::from_stack(machine.mem(), esp + 4u32);
            #[cfg(feature = "cpuemu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::DispatchMessageA(machine, lpMsg).await;
                    machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
                    machine.x86.cpu.regs.esp += 8u32;
                    machine.x86.cpu.regs.eax = result.to_raw();
                };
                crate::shims::become_async(machine, Box::pin(result));
                0
            }
            #[cfg(not(feature = "cpuemu"))]
            {
                let pin = std::pin::pin!(winapi::user32::DispatchMessageA(machine, lpMsg));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn GetActiveWindow(machine: &mut Machine, esp: u32) -> u32 {
            winapi::user32::GetActiveWindow(machine).to_raw()
        }
        pub unsafe fn GetClientRect(machine: &mut Machine, esp: u32) -> u32 {
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
            let lpRect = <Option<&mut RECT>>::from_stack(machine.mem(), esp + 8u32);
            winapi::user32::GetClientRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn GetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            winapi::user32::GetForegroundWindow(machine).to_raw()
        }
        pub unsafe fn GetLastActivePopup(machine: &mut Machine, esp: u32) -> u32 {
            winapi::user32::GetLastActivePopup(machine).to_raw()
        }
        pub unsafe fn GetMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let lpMsg = <Option<&mut MSG>>::from_stack(machine.mem(), esp + 4u32);
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(machine.mem(), esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(machine.mem(), esp + 16u32);
            winapi::user32::GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax).to_raw()
        }
        pub unsafe fn GetSystemMetrics(machine: &mut Machine, esp: u32) -> u32 {
            let nIndex = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::GetSystemMetrics(machine, nIndex).to_raw()
        }
        pub unsafe fn GetWindowDC(machine: &mut Machine, esp: u32) -> u32 {
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::GetWindowDC(machine, hWnd).to_raw()
        }
        pub unsafe fn LoadCursorA(machine: &mut Machine, esp: u32) -> u32 {
            let _hInstance = <u32>::from_stack(machine.mem(), esp + 4u32);
            let _lpCursorName = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::user32::LoadCursorA(machine, _hInstance, _lpCursorName).to_raw()
        }
        pub unsafe fn LoadIconA(machine: &mut Machine, esp: u32) -> u32 {
            let _hInstance = <u32>::from_stack(machine.mem(), esp + 4u32);
            let _lpIconName = <u32>::from_stack(machine.mem(), esp + 8u32);
            winapi::user32::LoadIconA(machine, _hInstance, _lpIconName).to_raw()
        }
        pub unsafe fn LoadImageA(machine: &mut Machine, esp: u32) -> u32 {
            let hInstance = <u32>::from_stack(machine.mem(), esp + 4u32);
            let name = <u32>::from_stack(machine.mem(), esp + 8u32);
            let typ = <u32>::from_stack(machine.mem(), esp + 12u32);
            let cx = <u32>::from_stack(machine.mem(), esp + 16u32);
            let cy = <u32>::from_stack(machine.mem(), esp + 20u32);
            let fuLoad = <u32>::from_stack(machine.mem(), esp + 24u32);
            winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn MessageBoxA(machine: &mut Machine, esp: u32) -> u32 {
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
            let lpText = <Option<&str>>::from_stack(machine.mem(), esp + 8u32);
            let lpCaption = <Option<&str>>::from_stack(machine.mem(), esp + 12u32);
            let uType = <u32>::from_stack(machine.mem(), esp + 16u32);
            winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn PeekMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let lpMsg = <Option<&mut MSG>>::from_stack(machine.mem(), esp + 4u32);
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(machine.mem(), esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(machine.mem(), esp + 16u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(machine.mem(), esp + 20u32);
            winapi::user32::PeekMessageA(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            )
            .to_raw()
        }
        pub unsafe fn RegisterClassA(machine: &mut Machine, esp: u32) -> u32 {
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::RegisterClassA(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterClassExA(machine: &mut Machine, esp: u32) -> u32 {
            let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::RegisterClassExA(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn ReleaseDC(machine: &mut Machine, esp: u32) -> u32 {
            let hdc = <HDC>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::ReleaseDC(machine, hdc).to_raw()
        }
        pub unsafe fn SetCursor(machine: &mut Machine, esp: u32) -> u32 {
            let hCursor = <u32>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::SetCursor(machine, hCursor).to_raw()
        }
        pub unsafe fn SetFocus(machine: &mut Machine, esp: u32) -> u32 {
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::SetFocus(machine, hWnd).to_raw()
        }
        pub unsafe fn SetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::SetForegroundWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn ShowCursor(machine: &mut Machine, esp: u32) -> u32 {
            let _bShow = <bool>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::ShowCursor(machine, _bShow).to_raw()
        }
        pub unsafe fn ShowWindow(machine: &mut Machine, esp: u32) -> u32 {
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
            let nCmdShow = <Result<SW, u32>>::from_stack(machine.mem(), esp + 8u32);
            winapi::user32::ShowWindow(machine, hWnd, nCmdShow).to_raw()
        }
        pub unsafe fn TranslateMessage(machine: &mut Machine, esp: u32) -> u32 {
            let lpMsg = <Option<&MSG>>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::TranslateMessage(machine, lpMsg).to_raw()
        }
        pub unsafe fn UpdateWindow(machine: &mut Machine, esp: u32) -> u32 {
            let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
            winapi::user32::UpdateWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn WaitMessage(machine: &mut Machine, esp: u32) -> u32 {
            winapi::user32::WaitMessage(machine).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const CreateCursor: Shim = Shim {
            name: "CreateCursor",
            func: impls::CreateCursor,
            stack_consumed: 32u32,
            is_async: false,
        };
        pub const CreateWindowExA: Shim = Shim {
            name: "CreateWindowExA",
            func: impls::CreateWindowExA,
            stack_consumed: 52u32,
            is_async: true,
        };
        pub const DefWindowProcA: Shim = Shim {
            name: "DefWindowProcA",
            func: impls::DefWindowProcA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const DestroyWindow: Shim = Shim {
            name: "DestroyWindow",
            func: impls::DestroyWindow,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const DialogBoxParamA: Shim = Shim {
            name: "DialogBoxParamA",
            func: impls::DialogBoxParamA,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const DispatchMessageA: Shim = Shim {
            name: "DispatchMessageA",
            func: impls::DispatchMessageA,
            stack_consumed: 8u32,
            is_async: true,
        };
        pub const GetActiveWindow: Shim = Shim {
            name: "GetActiveWindow",
            func: impls::GetActiveWindow,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetClientRect: Shim = Shim {
            name: "GetClientRect",
            func: impls::GetClientRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetForegroundWindow: Shim = Shim {
            name: "GetForegroundWindow",
            func: impls::GetForegroundWindow,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetLastActivePopup: Shim = Shim {
            name: "GetLastActivePopup",
            func: impls::GetLastActivePopup,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetMessageA: Shim = Shim {
            name: "GetMessageA",
            func: impls::GetMessageA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const GetSystemMetrics: Shim = Shim {
            name: "GetSystemMetrics",
            func: impls::GetSystemMetrics,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetWindowDC: Shim = Shim {
            name: "GetWindowDC",
            func: impls::GetWindowDC,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadCursorA: Shim = Shim {
            name: "LoadCursorA",
            func: impls::LoadCursorA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const LoadIconA: Shim = Shim {
            name: "LoadIconA",
            func: impls::LoadIconA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const LoadImageA: Shim = Shim {
            name: "LoadImageA",
            func: impls::LoadImageA,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const MessageBoxA: Shim = Shim {
            name: "MessageBoxA",
            func: impls::MessageBoxA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const PeekMessageA: Shim = Shim {
            name: "PeekMessageA",
            func: impls::PeekMessageA,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const RegisterClassA: Shim = Shim {
            name: "RegisterClassA",
            func: impls::RegisterClassA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const RegisterClassExA: Shim = Shim {
            name: "RegisterClassExA",
            func: impls::RegisterClassExA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const ReleaseDC: Shim = Shim {
            name: "ReleaseDC",
            func: impls::ReleaseDC,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetCursor: Shim = Shim {
            name: "SetCursor",
            func: impls::SetCursor,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetFocus: Shim = Shim {
            name: "SetFocus",
            func: impls::SetFocus,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetForegroundWindow: Shim = Shim {
            name: "SetForegroundWindow",
            func: impls::SetForegroundWindow,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const ShowCursor: Shim = Shim {
            name: "ShowCursor",
            func: impls::ShowCursor,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const ShowWindow: Shim = Shim {
            name: "ShowWindow",
            func: impls::ShowWindow,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const TranslateMessage: Shim = Shim {
            name: "TranslateMessage",
            func: impls::TranslateMessage,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const UpdateWindow: Shim = Shim {
            name: "UpdateWindow",
            func: impls::UpdateWindow,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const WaitMessage: Shim = Shim {
            name: "WaitMessage",
            func: impls::WaitMessage,
            stack_consumed: 4u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 29usize] = [
        Symbol {
            ordinal: None,
            shim: shims::CreateCursor,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateWindowExA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DefWindowProcA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DestroyWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::DialogBoxParamA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DispatchMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetActiveWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetClientRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetForegroundWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLastActivePopup,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemMetrics,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetWindowDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadCursorA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadIconA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadImageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::MessageBoxA,
        },
        Symbol {
            ordinal: None,
            shim: shims::PeekMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterClassA,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterClassExA,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReleaseDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetCursor,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetFocus,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetForegroundWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::ShowCursor,
        },
        Symbol {
            ordinal: None,
            shim: shims::ShowWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::TranslateMessage,
        },
        Symbol {
            ordinal: None,
            shim: shims::UpdateWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::WaitMessage,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "user32.dll",
        exports: &EXPORTS,
    };
}
pub mod winmm {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use winapi::winmm::*;
        pub unsafe fn timeGetTime(machine: &mut Machine, esp: u32) -> u32 {
            winapi::winmm::timeGetTime(machine).to_raw()
        }
        pub unsafe fn timeSetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let _uDelay = <u32>::from_stack(machine.mem(), esp + 4u32);
            let _uResolution = <u32>::from_stack(machine.mem(), esp + 8u32);
            let _lpTimeProc = <u32>::from_stack(machine.mem(), esp + 12u32);
            let _dwUser = <u32>::from_stack(machine.mem(), esp + 16u32);
            let _fuEvent = <u32>::from_stack(machine.mem(), esp + 20u32);
            winapi::winmm::timeSetEvent(
                machine,
                _uDelay,
                _uResolution,
                _lpTimeProc,
                _dwUser,
                _fuEvent,
            )
            .to_raw()
        }
        pub unsafe fn waveOutGetNumDevs(machine: &mut Machine, esp: u32) -> u32 {
            winapi::winmm::waveOutGetNumDevs(machine).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const timeGetTime: Shim = Shim {
            name: "timeGetTime",
            func: impls::timeGetTime,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const timeSetEvent: Shim = Shim {
            name: "timeSetEvent",
            func: impls::timeSetEvent,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const waveOutGetNumDevs: Shim = Shim {
            name: "waveOutGetNumDevs",
            func: impls::waveOutGetNumDevs,
            stack_consumed: 4u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 3usize] = [
        Symbol {
            ordinal: None,
            shim: shims::timeGetTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::timeSetEvent,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutGetNumDevs,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        exports: &EXPORTS,
    };
}
