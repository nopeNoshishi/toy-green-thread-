use crate::thread::colloction::MappedList;
use crate::thread::register::Registers;
use anyhow::Result;
use nix::sys::mman::{mprotect, ProtFlags};
use rand;
use std::{
    alloc::{alloc, dealloc, Layout},
    collections::{HashMap, HashSet, LinkedList},
    ffi::c_void,
    ptr,
};

pub static mut CTX_MAIN: Option<Box<Registers>> = None;

pub static mut UNUSED_STACK: (*mut u8, Layout) = (ptr::null_mut(), Layout::new::<u8>());

pub static mut CONTEXTS: LinkedList<Box<Context>> = LinkedList::new();

pub static mut ID: *mut HashSet<u64> = ptr::null_mut();

pub static mut MESSAGES: *mut MappedList<u64> = ptr::null_mut();

pub static mut WATTING: *mut HashMap<u64, Box<Context>> = ptr::null_mut();

extern "C" {
    pub fn set_context(ctx: *mut Registers) -> u64;
    pub fn switch_context(ctx: *const Registers) -> !;
}

pub type Entry = (String, fn());

const PAZE_SIZE: usize = 4 * 1024;

pub struct Context {
    registers: Registers,
    pub stack_pointer: *mut u8,
    pub stack_layout: Layout,
    pub entry: Entry,
    pub id: u64,
}

impl Context {
    pub fn get_regs_mut_ptr(&mut self) -> *mut Registers {
        &mut self.registers as *mut Registers
    }

    pub fn get_reg(&self) -> *const Registers {
        &self.registers as *const Registers
    }

    pub fn new(entry: Entry, stack_size: usize, id: u64) -> Result<Self> {
        let layout = Layout::from_size_align(stack_size, PAZE_SIZE)?;
        let stack = unsafe { alloc(layout) };

        unsafe { mprotect(stack as *mut c_void, PAZE_SIZE, ProtFlags::PROT_NONE)? };

        let regs = Registers::new(stack as u64 + stack_size as u64);

        Ok(Self {
            registers: regs,
            stack_pointer: stack,
            stack_layout: layout,
            entry,
            id,
        })
    }
}

pub unsafe fn rm_unused_stack() -> Result<()> {
    if UNUSED_STACK.0 != ptr::null_mut() {
        mprotect(
            UNUSED_STACK.0 as *mut c_void,
            PAZE_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        )?;

        dealloc(UNUSED_STACK.0, UNUSED_STACK.1);
        UNUSED_STACK = (ptr::null_mut(), Layout::new::<u8>())
    }

    Ok(())
}

pub fn get_id() -> u64 {
    loop {
        let rng = rand::random::<u64>();
        unsafe {
            if !(*ID).contains(&rng) {
                (*ID).insert(rng);
                return rng;
            }
        }
    }
}
