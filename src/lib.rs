//! # smp-nostd -- Bare-metal SMP support for x86_64
//!
//! This crate provides `no_std` symmetric multi-processing support including:
//! - Local APIC and I/O APIC drivers (xAPIC + x2APIC)
//! - AP (Application Processor) boot via real-mode trampoline
//! - Per-CPU data structures with GS-base access
//! - SMP-safe synchronization primitives (SpinLock, TicketLock, RwLock, Once)
//! - Multi-core task scheduler with work stealing
//! - High-level SMP controller API
//!
//! # Target
//!
//! This crate targets `x86_64-unknown-none` (bare-metal x86_64). It requires
//! `#![no_std]` and uses `extern crate alloc` for heap allocation.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use smp_nostd::{SmpController, MadtInfo};
//!
//! // Create SMP controller with APIC MMIO base (typically 0xFEE0_0000)
//! let mut smp = SmpController::new(0xFEE0_0000);
//!
//! // Parse ACPI MADT and initialize all cores
//! smp.init(madt_info);
//!
//! // Spawn a task on the least-loaded core
//! smp.spawn("my_task", entry_fn as u64, 0);
//! ```

#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate alloc;

pub mod apic;
pub mod driver;
pub mod ioapic;
pub mod percpu;
pub mod scheduler;
pub mod spinlock;
pub mod trampoline;

pub use apic::LocalApic;
pub use driver::SmpController;
pub use ioapic::IoApic;
pub use percpu::{get_current_cpu, PerCpu};
pub use scheduler::{Scheduler, Task, TaskId, TaskState};
pub use spinlock::{Once, RwLock, SpinLock, TicketLock};
pub use trampoline::ApTrampoline;

// Re-export driver types used in the public API
pub use driver::{MadtInfo, MadtIoApic, MadtLocalApic};
pub use ioapic::{IoApicManager, RedirectionEntry};
pub use scheduler::CpuContext;
