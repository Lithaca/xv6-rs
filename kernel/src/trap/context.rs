#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sepc: usize,
}

impl TrapContext {
    pub fn new() -> Self {
        Self {
            x: [0; 32],
            sepc: 0,
        }
    }

    pub fn w_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut cx = TrapContext::new();
        cx.w_sp(sp);
        cx.sepc = entry;
        cx
    }
}
