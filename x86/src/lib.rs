pub mod debug;
mod icache;
pub mod ops;
mod registers;
mod x86;

use memory::Mem;
use memory::VecMem;
pub use x86::{CPU, NULL_POINTER_REGION_SIZE, X86};
