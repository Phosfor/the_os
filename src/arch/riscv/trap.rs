
global_asm!(include_str!("trap.asm"));

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Context {
    kernel_pc: usize,
    x: [usize; 31],
    epc: usize,
    status: usize,
}

impl Context {
    pub fn x(&self, idx: usize) -> usize {
        self.x[idx - 1]
    }

    pub fn set_x(&mut self, idx: usize, value: usize) {
        self.x[idx - 1] = value;
    }

    pub fn epc(&self) -> usize {
        self.epc
    }

    pub fn set_epc(&mut self, pc: usize) {
        self.epc = pc;
    }

    pub fn status(&self) -> usize {
        self.status
    }

    pub fn set_status(&mut self, status: usize) {
        self.status = status;
    }
}

impl Context {
    pub unsafe fn run(&mut self) {
        extern "C" {
            fn trap_context_run(ctx: *mut Context);
        }
        trap_context_run(self)
    }
}

pub unsafe fn init() {
    use riscv::register::*;
    extern "C" { fn trap_vector(); }
    mtvec::write(trap_vector as _, mtvec::TrapMode::Direct);
}