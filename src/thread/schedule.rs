use crate::thread::context::{rm_unused_stack, set_context, switch_context, CONTEXTS};
use anyhow::{Context as _, Result};

pub fn schedule(task_name: &str) -> Result<()> {
    unsafe {
        // 最後のタスクだった時にはまるところ
        if CONTEXTS.len() == 1 {
            return Ok(());
        }

        let mut ctx = CONTEXTS.pop_front().context("No context")?;
        let regs = ctx.get_regs_mut_ptr();
        CONTEXTS.push_back(ctx);
        println!("Scheduled: {task_name}");

        if set_context(regs) == 0 {
            let next = CONTEXTS.front().context("No context")?;
            switch_context((**next).get_reg());
        }

        rm_unused_stack()?;
    }
    Ok(())
}
