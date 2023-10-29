use crate::thread::context::switch_context;
use crate::thread::context::{CONTEXTS, CTX_MAIN, ID, UNUSED_STACK};
use crate::thread::register::Registers;
use anyhow::{bail, Context as _, Result};

#[allow(improper_ctypes_definitions)]
pub extern "C" fn entry_point() -> Result<()> {
    unsafe {
        let ctx = CONTEXTS.front().context("contexts has no")?;
        ((**ctx).entry.1)();

        let ctx = CONTEXTS.pop_front().context("contexts has no")?;

        (*ID).remove(&ctx.id);

        UNUSED_STACK = ((*ctx).stack_pointer, (*ctx).stack_layout);

        match CONTEXTS.front() {
            Some(c) => {
                switch_context((**c).get_reg());
            }
            None => {
                if let Some(c) = &CTX_MAIN {
                    switch_context(&**c as *const Registers)
                }
            }
        }
    }
    bail!("[Error] entry_point")
}
