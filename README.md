# Sovereign Unix Distribution & Windows NT / GPU Runtime (Pillar 1)

Production-grade sovereign Unix distribution combining Linux Kernel `v7.4-rc2`, CachyOS BORE/sched-ext optimizations, All-GNU-in-Rust (`uutils`), .NET 11/.NET 10 multi-targeted WinUI host, and native MSVC C++23 hardware acceleration on Windows NT & NVIDIA Ada Lovelace (`sm_89`).

---

## Architecture Overview

```
C:\src\
├── linux\                                  # Linux Kernel v7.4-rc2 (Upstream Torvalds)
│   ├── Makefile                            # VERSION = 7, PATCHLEVEL = 4, EXTRAVERSION = -rc2
│   └── arch\x86\configs\
│       └── cachyos_sm89_wsl2_baremetal.config # Production CachyOS kernel config fragment
│
└── unix\                                   # Sovereign Unix Distribution Workspace
    ├── Cargo.toml                          # Unified Rust Workspace
    ├── distro.ts                           # Bun FFI & Orchestration Runner
    ├── native_nt_gpu.dll                   # Compiled MSVC C++23 Native Hardware Bridge
    ├── bin\                                # Isolated All-GNU-in-Rust executable shims (73 utilities)
    │   ├── env.ps1                         # PowerShell environment activation
    │   ├── env.sh                          # POSIX shell environment activation
    │   └── *.cmd                           # Non-intrusive command dispatchers
    │
    ├── crates\
    │   ├── unix-uutils\                    # All-GNU-in-Rust userland catalog & dispatch engine
    │   ├── unix-cachyos\                   # Hardware probe (chwd), pacman/makepkg & Proton orchestrator
    │   └── unix-distro\                    # Unified sovereign distro management CLI
    │
    ├── winnt-sdk\
    │   ├── native-cpp\                     # MSVC 14.51 C++23 / C23 Hardware Bridge
    │   │   ├── native_nt_gpu.h
    │   │   ├── native_nt_gpu.cpp
    │   │   ├── main.cpp
    │   │   └── build.cmd                   # cl.exe /std:c++latest /arch:AVX2 /O2
    │   │
    │   └── DotNetWinUiHost\                # .NET 11 & .NET 10 Multi-Targeted Host
    │       ├── DotNetWinUiHost.csproj      # net11.0 + net10.0 NativeAOT-ready
    │       └── Program.cs                  # LibraryImport P/Invoke into WinUI, dcomp, NT kernel
    │
    ├── upstream\                           # Minimal shallow clones of upstream references
    │   ├── chwd\                           # CachyOS Hardware Detection
    │   ├── CachyOS-Settings\               # Sysctl, udev, scheduler settings
    │   └── coreutils\                      # Upstream uutils/coreutils reference
    │
    └── tests\
        └── distro.test.ts                  # Bun test suite verifying all 7 subsystems
```

---

## Subsystem Details

### 1. Linux Kernel `v7.4-rc2` (`C:\src\linux`)
- Upstream Git tree: `https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git`
- Cone Sparse-Checkout: `rust`, `arch/x86/configs`, `drivers/hv/dxgkrnl`, `drivers/gpu/drm/nouveau`, `kernel/sched`, `include/uapi/linux`, `scripts`.
- Production Config Fragment: `arch/x86/configs/cachyos_sm89_wsl2_baremetal.config`:
  - `CONFIG_RUST=y`, `CONFIG_SCHED_CLASS_EXT=y`, `CONFIG_SCHED_BORE=y`
  - `CONFIG_MNATIVE_INTEL=y`, `CONFIG_X86_INTEL_PSTATE=y`, `CONFIG_INTEL_HFI_THERMAL=y`
  - `CONFIG_NTSYNC=y` (`/dev/ntsync` native Wine/Proton NT synchronization primitive)
  - `CONFIG_SECCOMP=y`, `CONFIG_GENERIC_ENTRY=y` (`PR_SET_SYSCALL_USER_DISPATCH` for Easy Anti-Cheat & BattlEye)
  - `CONFIG_DXGKRNL=y` (`/dev/dxg` DirectX 12 GPU-PV bridge)
  - `CONFIG_LRU_GEN=y` (MGLRU), `CONFIG_HZ_1000=y`, `CONFIG_PREEMPT_FULL=y`

### 2. CachyOS & All-GNU-in-Rust Distribution (`C:\src\unix`)
- **`unix-uutils`**: Catalogs and dispatches 73 GNU utilities re-implemented in pure Rust across Coreutils, Findutils, Diffutils, Sed, Procps, and Util-Linux. Binaries and shims are strictly isolated in `C:\src\unix\bin` without modifying `C:\Windows\System32`.
- **`unix-cachyos`**:
  - `chwd`-compatible hardware probe: detects Intel Core i7-13700F (`x86-64-v3` AVX2/FMA3/BMI2) and NVIDIA GeForce RTX 4070 (PCI `10de:2786`, Ada Lovelace `sm_89`, 12 GB VRAM).
  - Generates optimized `pacman.conf` with `[cachyos-v3]`, `[cachyos-core-v3]`, and `[cachyos-extra-v3]`.
  - Generates `makepkg.conf` targeting Raptor Lake: `-march=raptorlake -O3 -pipe -fno-plt -flto=thin -fexceptions -Wp,-D_FORTIFY_SOURCE=3`.
  - Generates scheduler configuration for `scx_lavd` and `BORE`.
  - Proton-CachyOS / Wine orchestrator: configures `PROTON_USE_NTSYNC=1`, `PROTON_EAC_RUNTIME=1`, `WINE_FULLSCREEN_FSR=1`, `DXVK_NVAPI_ALLOW_OTHER_DRIVERS=1`, `PROTON_ENABLE_NVAPI=1`, `PROTON_ENABLE_WAYLAND=1`, and verifies `PR_SET_SYSCALL_USER_DISPATCH`.

### 3. Windows NT SDK + WinUI Core + MSVC C++23 Native Bridge
- **MSVC 14.51 C++23 Library (`native_nt_gpu.dll`)**:
  - Compiled using `cl.exe /std:c++latest /arch:AVX2 /O2 /LD`.
  - Uses `ntdll.dll` (`RtlGetVersion`) to detect exact Windows NT build.
  - Uses `dxgi.dll` (`CreateDXGIFactory2` and `EnumAdapters1`) to query NVIDIA GeForce RTX 4070 dedicated VRAM (12010 MB).
  - Uses `CoreMessaging.dll` (`CreateDispatcherQueueController`) to initialize WinUI Core dedicated STA event loop.
- **.NET 11 & .NET 10 Multi-Targeted Host (`DotNetWinUiHost`)**:
  - Multi-targets `<TargetFrameworks>net11.0;net10.0</TargetFrameworks>` with `<LangVersion>preview</LangVersion>`.
  - Uses source-generated `[LibraryImport]` P/Invoke into `kernel32.dll`, `CoreMessaging.dll`, `dcomp.dll`, and `native_nt_gpu.dll`.
  - Ready for NativeAOT compilation.

### 4. Bun Native Orchestrator (`distro.ts`)
- Leverages `bun:ffi` (`dlopen`) to interact directly with `native_nt_gpu.dll`.
- Orchestrates Rust `unix-distro`, `.NET 11`, and Linux `v7.4-rc2` kernel configs.
- Validated via `bun test` in `tests/distro.test.ts`.

---

## Verification Commands

```powershell
# 1. Cargo Tests
cargo test --manifest-path C:\src\unix\Cargo.toml --target-dir C:\src\unix\target

# 2. MSVC C++23 Compile & Run
cmd /c "C:\src\unix\winnt-sdk\native-cpp\build.cmd"

# 3. .NET 11 & .NET 10 Multi-Target Build & Run
dotnet build C:\src\unix\winnt-sdk\DotNetWinUiHost\DotNetWinUiHost.csproj
dotnet run --project C:\src\unix\winnt-sdk\DotNetWinUiHost\DotNetWinUiHost.csproj --framework net11.0
dotnet run --project C:\src\unix\winnt-sdk\DotNetWinUiHost\DotNetWinUiHost.csproj --framework net10.0

# 4. Bun Test Suite
cd C:\src\unix
bun test
```
