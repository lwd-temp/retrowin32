const std = @import("std");
const windows = std.os.windows;

pub extern "retrowin32" fn retrowin32_callback1(
    func: *const fn (u32) callconv(.Stdcall) u32,
    data: u32,
) callconv(windows.WINAPI) u32;

fn callback0(data: u32) callconv(windows.WINAPI) u32 {
    std.debug.print("callback0 invoked: {x}\n", .{data});
    return 0x4567;
}

pub fn main() !void {
    const ret = retrowin32_callback1(callback0, 0x1234);
    std.debug.print("retrowin32_callback1 returned: {x}\n", .{ret});
}