# Leak demonstration inside `rayon`

Steps to reproduce the leak:

```sh
$ rustc --version
rustc 1.47.0 (18bf6b4f0 2020-10-07)

$ uname -v
Darwin Kernel Version 19.6.0: Mon Aug 31 22:12:52 PDT 2020; root:xnu-6153.141.2~1/RELEASE_X86_64

$ cargo build

$ valgrind target/debug/bug
valgrind --leak-check=full target/debug/bug
==16218== Memcheck, a memory error detector
==16218== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==16218== Using Valgrind-3.17.0.GIT and LibVEX; rerun with -h for copyright info
==16218== Command: target/debug/bug
==16218==
==16218==
==16218== HEAP SUMMARY:
==16218==     in use at exit: 38,162 bytes in 289 blocks
==16218==   total heap usage: 351 allocs, 62 frees, 48,808 bytes allocated
==16218==
==16218== 8 bytes in 1 blocks are definitely lost in loss record 1 of 59
==16218==    at 0x101FFE635: malloc (in /usr/local/Cellar/valgrind/HEAD-6049595/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==16218==    by 0x1001D3CF6: alloc::alloc::alloc (alloc.rs:74)
==16218==    by 0x1001D3DA9: alloc::alloc::Global::alloc_impl (alloc.rs:153)
==16218==    by 0x1001D4B1A: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:203)
==16218==    by 0x1001D3C55: alloc::alloc::exchange_malloc (alloc.rs:281)
==16218==    by 0x1001D52B3: rayon_core::util::leak (boxed.rs:175)
==16218==    by 0x1001C36B0: rayon_core::registry::set_global_registry::{{closure}}::{{closure}} (registry.rs:196)
==16218==    by 0x1001F0298: core::result::Result<T,E>::map (result.rs:508)
==16218==    by 0x1001C3708: rayon_core::registry::set_global_registry::{{closure}} (registry.rs:195)
==16218==    by 0x1001DB38B: std::sync::once::Once::call_once::{{closure}} (once.rs:265)
==16218==    by 0x100BF0510: std::sync::once::Once::call_inner (once.rs:421)
==16218==    by 0x1001DB317: std::sync::once::Once::call_once (once.rs:265)
==16218==
==16218== LEAK SUMMARY:
==16218==    definitely lost: 8 bytes in 1 blocks
==16218==    indirectly lost: 0 bytes in 0 blocks
==16218==      possibly lost: 0 bytes in 0 blocks
==16218==    still reachable: 22,521 bytes in 125 blocks
==16218==         suppressed: 15,633 bytes in 163 blocks
==16218== Reachable blocks (those to which a pointer was found) are not shown.
==16218== To see them, rerun with: --leak-check=full --show-leak-kinds=all
==16218==
==16218== For lists of detected and suppressed errors, rerun with: -s
==16218== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 5 from 5)
```

To solve the leak, edit `Cargo.toml` and uncomment as follows:

```diff
- #[patch.crates-io]
- #rayon-core = { git = "https://github.com/Hywan/rayon", branch = "fix-registry-leak" }
+ [patch.crates-io]
+ rayon-core = { git = "https://github.com/Hywan/rayon", branch = "fix-registry-leak" }
```

Then restart:

```sh
$ cargo build

$ valgrind --leak-check=full target/debug/bug
==16451== Memcheck, a memory error detector
==16451== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==16451== Using Valgrind-3.17.0.GIT and LibVEX; rerun with -h for copyright info
==16451== Command: target/debug/bug
==16451==
==16451==
==16451== HEAP SUMMARY:
==16451==     in use at exit: 38,154 bytes in 288 blocks
==16451==   total heap usage: 350 allocs, 62 frees, 48,800 bytes allocated
==16451==
==16451== LEAK SUMMARY:
==16451==    definitely lost: 0 bytes in 0 blocks
==16451==    indirectly lost: 0 bytes in 0 blocks
==16451==      possibly lost: 0 bytes in 0 blocks
==16451==    still reachable: 22,521 bytes in 125 blocks
==16451==         suppressed: 15,633 bytes in 163 blocks
==16451== Reachable blocks (those to which a pointer was found) are not shown.
==16451== To see them, rerun with: --leak-check=full --show-leak-kinds=all
==16451==
==16451== For lists of detected and suppressed errors, rerun with: -s
==16451== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 5 from 5)
```

Done.
