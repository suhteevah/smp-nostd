# smp-nostd

[![Crates.io](https://img.shields.io/crates/v/smp-nostd.svg)](https://crates.io/crates/smp-nostd)
[![Docs.rs](https://docs.rs/smp-nostd/badge.svg)](https://docs.rs/smp-nostd)
[![License](https://img.shields.io/crates/l/smp-nostd.svg)](https://github.com/suhteevah/smp-nostd)

Bare-metal `#![no_std]` SMP (Symmetric Multi-Processing) support for x86_64 operating systems.

## Features

- **Local APIC driver** -- xAPIC (memory-mapped) and x2APIC (MSR-based) with full register access, IPI delivery, and timer calibration
- **I/O APIC driver** -- indirect register access, redirection table management, IRQ routing, multi-I/O-APIC support
- **AP boot trampoline** -- hand-assembled real-mode -> protected-mode -> long-mode trampoline with INIT-SIPI-SIPI sequence
- **Per-CPU data** -- GS-base accessed `PerCpu` structures with cache-line alignment, up to 256 cores
- **Work-stealing scheduler** -- per-core run queues, least-loaded placement, half-queue stealing, CPU affinity, preemption support
- **Synchronization primitives** -- `SpinLock` (interrupt-disabling), `TicketLock` (fair FIFO), `RwLock` (reader-writer), `Once` (one-shot init)
- **Context switch** -- full x86_64 register save/restore via inline assembly
- **Memory fences** -- `mfence`, `sfence`, `lfence` wrappers plus Rust atomic fence helpers

## Requirements

- Target: `x86_64-unknown-none` (bare-metal)
- Rust nightly (inline assembly, `#![no_std]`)
- A global allocator must be set up before using types that allocate (scheduler, per-CPU registration, etc.)
- Identity-mapped lower 1 MiB for the AP trampoline (physical address 0x8000)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
smp-nostd = "0.1"
```

### Initialize SMP

```rust,ignore
use smp_nostd::{SmpController, MadtInfo, MadtLocalApic, MadtIoApic};

// Parse ACPI MADT to discover CPUs and I/O APICs
let madt_info = MadtInfo {
    local_apic_addr: 0xFEE0_0000,
    local_apics: vec![
        MadtLocalApic { processor_id: 0, apic_id: 0, enabled: true },
        MadtLocalApic { processor_id: 1, apic_id: 1, enabled: true },
    ],
    io_apics: vec![
        MadtIoApic { id: 0, address: 0xFEC0_0000, gsi_base: 0 },
    ],
};

let mut smp = SmpController::new(0xFEE0_0000);
smp.init(madt_info);

// Spawn work
smp.spawn("worker", worker_fn as u64, 42);
```

### Use synchronization primitives standalone

```rust,ignore
use smp_nostd::{SpinLock, TicketLock, RwLock, Once};

static DATA: SpinLock<u64> = SpinLock::new(0);
static FAIR: TicketLock<u64> = TicketLock::new(0);
static CONFIG: RwLock<u64> = RwLock::new(0);
static INIT: Once = Once::new();

// SpinLock disables interrupts while held
{
    let mut guard = DATA.lock();
    *guard += 1;
}

// Once runs the closure exactly once
INIT.call_once(|| {
    // one-time initialization
});
```

## Architecture

```
SmpController
  |-- LocalApic (BSP)         xAPIC / x2APIC register access
  |-- IoApicManager            Routes GSIs to CPU cores
  |   `-- IoApic[]             Per-chip redirection tables
  |-- ApTrampoline             Real-mode boot code at 0x8000
  |-- PerCpu[]                 GS-base per-core data (up to 256)
  `-- Scheduler                Per-core run queues + work stealing
       `-- RunQueue[]          SpinLock<VecDeque<Task>>
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Contributions welcome! Please open an issue or PR at <https://github.com/suhteevah/smp-nostd>.

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!
