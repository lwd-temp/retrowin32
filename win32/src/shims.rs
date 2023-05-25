//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! In the simple case, we register Rust functions like kernel32.dll!ExitProcess
//! to associate with a special invalid x86 address.  If the x86 ever jumps to such an
//! address, we forward the call to the registered shim handler.
//!
//! The win32_derive::dllexport attribute on our shim functions wraps them with
//! a prologue/epilogue that does the required stack manipulation to read
//! arguments off the x86 stack and transform them into Rust types, so the Rust
//! functions can act as if they're just being called from Rust.
//!
//! The more complex case is when our Rust function needs to call back into x86
//! code.  x86 emulation executes one basic block of instructions at a time, while
//! our Rust shim functions execute to completion synchronously, so the latter
//! cannot directly call the former.
//!
//! To reconcile these, we make functions that call back into x86 into "async"
//! Rust functions, that return a Future.
//!
//! 1) A given shim winapi function like IDirectDraw7::EnumDisplayModes needs
//!    to call back to x86 with each available display mode.
//! 2) To do so, we change its body to the form:
//!
//!    #[win32_derive::dllexport]
//!    async fn EnumDisplayModes(...) -> u32 {
//!      ...setup code...
//!      // Call into x86 function and await its result.
//!      shims::async_call(some_ptr, vec![args]).await;
//!      // Return to x86 caller, as before.
//!      0
//!    }
//! 3) The dllexport transform notices that async type and forwards
//!    to push_async in this module, which redirects the x86 to call
//!    async_executor next.
//! 4) async_executor picks up the Future returned in step #2 and runs it.
//!
//! Concretely when EnumDisplayModes is called, the "simple case" shim logic as
//! described above runs as before, but rather than returning to the caller
//! we instead also push a call to async_executor, which adds itself to the call stack
//! and runs the async state machine.  In the case of async_call that means the
//! x86 code eventually invoked there will return control back to async_executor.

use crate::machine::Machine;

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
pub const SHIM_BASE: u32 = 0xF1A7_0000;

struct Shim {
    name: String,
    handler: Option<fn(&mut Machine)>,
}

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
pub struct Shims {
    shims: Vec<Shim>,
    /// Address of async_executor() shim entry point.
    async_executor: u32,
    /// Pending future for code being ran by async_executor().
    /// TODO: we will need a stack of these to handle multiple nested callbacks.
    future: Option<std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>>,
}

/// Redirect x86 control to async_executor.  Note this has particular requirements on the
/// state of the stack, and is called when a dllexport function is async.
pub fn become_async(
    machine: &mut Machine,
    future: std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>,
) {
    machine.x86.regs.eip = machine.shims.async_executor;
    machine.shims.future = Some(future);
}

impl Shims {
    pub fn new() -> Self {
        let mut shims = Shims {
            shims: Vec::new(),
            async_executor: 0,
            future: None,
        };
        shims.async_executor = shims.add("retrowin32 async helper".into(), Some(async_executor));
        shims
    }

    /// Returns the (fake) address of the registered function.
    pub fn add(&mut self, name: String, handler: Option<fn(&mut Machine)>) -> u32 {
        let id = SHIM_BASE | self.shims.len() as u32;
        self.shims.push(Shim { name, handler });
        id
    }

    pub fn get(&self, addr: u32) -> Option<&fn(&mut Machine)> {
        let index = (addr & 0x0000_FFFF) as usize;
        match self.shims.get(index) {
            Some(shim) => {
                if let Some(handler) = &shim.handler {
                    return Some(handler);
                }
                log::error!("unimplemented: {}", shim.name);
            }
            None => log::error!("unknown import reference at {:x}", addr),
        };
        None
    }

    pub fn lookup(&self, name: &str) -> Option<u32> {
        if let Some(idx) = self.shims.iter().position(|shim| shim.name == name) {
            return Some(SHIM_BASE | idx as u32);
        }
        None
    }
}

pub struct X86Future {
    // We know the Machine is around for the duration of the future execution.
    // https://github.com/rust-lang/futures-rs/issues/316
    m: *const Machine,
    esp: u32,
}
impl std::future::Future for X86Future {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let machine = unsafe { &*self.m };
        // log::info!("poll esp:{:x} want:{:x}", machine.x86.regs.esp, self.esp);
        if machine.x86.regs.esp == self.esp {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}

pub fn async_call(machine: &mut Machine, func: u32, args: Vec<u32>) -> X86Future {
    // Save original esp, as that's the marker that we use to know when the call is done.
    let esp = machine.x86.regs.esp;
    // Push the args in reverse order.
    for &arg in args.iter().rev() {
        x86::ops::push(&mut machine.x86, &mut machine.mem, arg);
    }
    x86::ops::push(
        &mut machine.x86,
        &mut machine.mem,
        machine.shims.async_executor,
    ); // return address
    machine.x86.regs.eip = func;
    X86Future { m: machine, esp }
}

#[allow(deref_nullptr)]
fn async_executor(machine: &mut Machine) {
    if let Some(mut future) = machine.shims.future.take() {
        // TODO: we don't use the waker at all.  Rust doesn't like us passing a random null pointer
        // here but it seems like nothing accesses it(?).
        //let c = unsafe { std::task::Context::from_waker(&Waker::from_raw(std::task::RawWaker::)) };
        let context: &mut std::task::Context = unsafe { &mut *std::ptr::null_mut() };
        match future.as_mut().poll(context) {
            std::task::Poll::Ready(()) => {}
            std::task::Poll::Pending => {
                if machine.shims.future.is_some() {
                    panic!("multiple pending futures");
                }
                machine.shims.future = Some(future);
            }
        }
    }
}
