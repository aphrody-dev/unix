import { describe, expect, test } from "bun:test";
import { existsSync, readFileSync } from "fs";
import { join } from "path";
import {
  checkDistroFullStatus,
  loadNativeBridge,
  probeDistroHardware,
  runDotNetHost,
  verifyLinuxKernelConfig,
} from "../distro";

describe("Sovereign Unix Distribution & WinNT/GPU Pillar 1 Verification", () => {
  test("1. Linux Kernel v7.4-rc2 Upstream Sparse-Clone & Makefile Version", () => {
    const makefilePath = "C:\\src\\linux\\Makefile";
    expect(existsSync(makefilePath)).toBe(true);

    const makefile = readFileSync(makefilePath, "utf-8");
    const lines = makefile.split("\n").slice(0, 5);
    const content = lines.join("\n");

    expect(content).toContain("VERSION = 7");
    expect(content).toContain("PATCHLEVEL = 4");
    expect(content).toContain("EXTRAVERSION = -rc2");
  });

  test("2. Linux v7.4-rc2 CachyOS sm_89 & Raptor Lake Kernel Config Fragment", () => {
    const res = verifyLinuxKernelConfig();
    expect(res.valid).toBe(true);
    expect(res.options.length).toBeGreaterThanOrEqual(9);
  });

  test("3. CachyOS Hardware Probe (i7-13700F + RTX 4070 sm_89)", () => {
    const hw = probeDistroHardware();
    expect(hw.cpu.brand).toContain("Intel");
    expect(hw.cpu.architecture_level).toBe("x86-64-v3");
    expect(hw.cpu.has_avx2).toBe(true);
    expect(hw.gpu.name).toBe("NVIDIA GeForce RTX 4070");
    expect(hw.gpu.architecture).toBe("Ada Lovelace (sm_89)");
    expect(hw.gpu.vendor_id).toBe(0x10de);
    expect(hw.gpu.device_id).toBe(0x2786);
    expect(hw.gpu.vram_mb).toBeGreaterThanOrEqual(12000);
  });

  test("4. All-GNU-in-Rust Uutils Clean Isolation in C:\\src\\unix\\bin", () => {
    const binDir = "C:\\src\\unix\\bin";
    expect(existsSync(binDir)).toBe(true);
    expect(existsSync(join(binDir, "ls.cmd"))).toBe(true);
    expect(existsSync(join(binDir, "cat.cmd"))).toBe(true);
    expect(existsSync(join(binDir, "uname.cmd"))).toBe(true);
    expect(existsSync(join(binDir, "env.ps1"))).toBe(true);
    expect(existsSync(join(binDir, "env.sh"))).toBe(true);

    const ps1 = readFileSync(join(binDir, "env.ps1"), "utf-8");
    expect(ps1).toContain("All-GNU-in-Rust initialized");
    expect(ps1).not.toContain("System32");
  });

  test("5. MSVC C++23 Native Bridge & WinUI DispatcherQueue via Bun FFI", () => {
    const native = loadNativeBridge();
    const dqStatus = native.symbols.InitWinUiDispatcherQueue();
    expect(dqStatus).toBe(0);
  });

  test("6. .NET 11 Multi-Targeted Host with NativeAOT & WinUI DirectComposition P/Invoke", () => {
    const res = runDotNetHost();
    expect(res.status).toBe(0);
    expect(res.output).toContain("Sovereign .NET WinUI Host");
    expect(res.output).toContain("NVIDIA GeForce RTX 4070");
    expect(res.output).toContain("WinUI DispatcherQueue:     SUCCESS");
  });

  test("7. Full End-to-End Distro Status Orchestration", () => {
    const status = checkDistroFullStatus();
    expect(status.msvcBridgeStatus).toBe(0);
    expect(status.kernelConfigValid).toBe(true);
    expect(status.uutilsShimsCount).toBe(73);
    expect(status.dotNet11Status).toBe(0);
  });
});
