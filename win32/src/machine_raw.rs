use std::collections::HashMap;

use crate::{
    host,
    machine::{LoadedAddrs, MachineX},
    pe,
    shims::Shims,
    winapi,
};
use memory::MemImpl;

pub type Machine = MachineX<()>;

impl MachineX<()> {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        let mut memory = MemImpl::default();
        let mut kernel32 = winapi::kernel32::State::new(&mut memory, cmdline);
        let mapping = kernel32
            .mappings
            .alloc(0x4000, "shims x64 trampoline".into(), &mut memory);
        let shims = Shims::new(
            &mut kernel32.ldt,
            mapping.addr as u64 as *mut u8,
            mapping.size,
        );
        let state = winapi::State::new(kernel32);

        Machine {
            emu: (),
            memory,
            host,
            state,
            shims,
            labels: HashMap::new(),
        }
    }

    #[allow(non_snake_case)]
    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
        relocate: bool,
    ) -> anyhow::Result<LoadedAddrs> {
        let exe = pe::load_exe(self, buf, cmdline, relocate)?;

        let stack =
            self.state
                .kernel32
                .mappings
                .alloc(exe.stack_size, "stack".into(), &mut self.memory);
        let stack_pointer = stack.addr + stack.size - 4;

        Ok(LoadedAddrs {
            entry_point: exe.entry_point,
            stack_pointer,
        })
    }
}
