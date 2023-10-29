use std::collections::{HashMap, HashSet};
use std::ptr;

use crate::thread::colloction::MappedList;
use crate::thread::context::{
    get_id, rm_unused_stack, set_context, switch_context, Context, Entry, CONTEXTS, CTX_MAIN, ID,
    MESSAGES, WATTING,
};
use crate::thread::register::Registers;
use crate::thread::schedule::schedule;
use anyhow::{bail, Context as _, Result};

pub fn spawn(entry: Entry, stack_size: usize) -> Result<u64> {
    unsafe {
        let id = get_id();
        let name = entry.0.clone();
        CONTEXTS.push_back(Box::new(Context::new(entry, stack_size, id)?));
        schedule(&name)?;
        Ok(id)
    }
}

pub fn spawn_from_main(entry: Entry, stack_size: usize) -> Result<()> {
    unsafe {
        if let Some(_) = &CTX_MAIN {
            bail!("spawn_from_main is called twice!")
        }

        CTX_MAIN = Some(Box::new(Registers::new(0)));

        if let Some(ctx) = &mut CTX_MAIN {
            let mut msgs = MappedList::new();
            MESSAGES = &mut msgs as *mut MappedList<u64>;

            let mut watting = HashMap::new();
            WATTING = &mut watting as *mut HashMap<u64, Box<Context>>;

            let mut ids = HashSet::new();
            ID = &mut ids as *mut HashSet<u64>;

            if set_context(&mut **ctx as *mut Registers) == 0 {
                CONTEXTS.push_back(Box::new(Context::new(entry, stack_size, get_id())?));
                let first = CONTEXTS.front().context("contexts has no")?;
                switch_context(first.get_reg());
            }

            rm_unused_stack()?;

            CTX_MAIN = None;
            CONTEXTS.clear();
            MESSAGES = ptr::null_mut();
            WATTING = ptr::null_mut();
            ID = ptr::null_mut();

            msgs.clear();
            watting.clear();
            ids.clear();
        }
    }

    Ok(())
}
