# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added

- Local APIC driver with xAPIC (memory-mapped) and x2APIC (MSR-based) support
- I/O APIC driver with redirection table management and multi-chip support
- AP boot trampoline (real-mode -> protected-mode -> long-mode transition)
- Per-CPU data structures with GS-base access (up to 256 cores)
- Multi-core task scheduler with per-core run queues and work-stealing
- Synchronization primitives: SpinLock, TicketLock, RwLock, Once
- Context switch via inline assembly (full x86_64 register save/restore)
- High-level SmpController API for MADT-based SMP initialization
- Memory fence helpers (mfence, sfence, lfence)
