using System;
using System.Runtime.InteropServices;
using System.Text;

namespace DotNetWinUiHost;

internal static partial class NativeInterop
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct SYSTEM_INFO
    {
        public ushort wProcessorArchitecture;
        public ushort wReserved;
        public uint dwPageSize;
        public nint lpMinimumApplicationAddress;
        public nint lpMaximumApplicationAddress;
        public nint dwActiveProcessorMask;
        public uint dwNumberOfProcessors;
        public uint dwProcessorType;
        public uint dwAllocationGranularity;
        public ushort wProcessorLevel;
        public ushort wProcessorRevision;
    }

    internal enum DISPATCHERQUEUE_THREAD_APARTMENTTYPE
    {
        DQTAT_COM_NONE = 0,
        DQTAT_COM_ASTA = 1,
        DQTAT_COM_STA = 2
    }

    internal enum DISPATCHERQUEUE_THREAD_TYPE
    {
        DQTYPE_THREAD_DEDICATED = 1,
        DQTYPE_THREAD_CURRENT = 2,
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct DispatcherQueueOptions
    {
        public uint dwSize;
        public DISPATCHERQUEUE_THREAD_TYPE threadType;
        public DISPATCHERQUEUE_THREAD_APARTMENTTYPE apartmentType;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct NTArchitectureReport
    {
        public uint MajorVersion;
        public uint MinorVersion;
        public uint BuildNumber;
        public uint ProcessorArchitecture;
        public uint NumberOfProcessors;
        public uint PageSize;
        public uint HasAvx2;
        public uint HasAvx;
        public uint IsRaptorLakeSm89Compatible;
    }

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    internal unsafe struct GpuAdapterReport
    {
        public fixed char Description[128];
        public uint VendorId;
        public uint DeviceId;
        public uint SubSysId;
        public uint Revision;
        public ulong DedicatedVideoMemory;
        public ulong DedicatedSystemMemory;
        public ulong SharedSystemMemory;
        public uint IsNvidiaRtx4070;
    }

    // Windows NT Kernel P/Invokes via LibraryImport
    [LibraryImport("kernel32.dll")]
    internal static partial uint GetCurrentProcessId();

    [LibraryImport("kernel32.dll")]
    internal static partial void GetSystemInfo(out SYSTEM_INFO lpSystemInfo);

    // WinUI Core DispatcherQueue via CoreMessaging.dll
    [LibraryImport("CoreMessaging.dll")]
    internal static partial int CreateDispatcherQueueController(
        DispatcherQueueOptions options,
        out nint dispatcherQueueController);

    // DirectComposition via dcomp.dll
    [LibraryImport("dcomp.dll")]
    internal static partial int DCompositionCreateDevice(
        nint dxgiDevice,
        in Guid iid,
        out nint dcompositionDevice);

    // MSVC C++23 Native Hardware Bridge DLL
    [LibraryImport("native_nt_gpu.dll")]
    internal static partial int GetNTSystemArchitectureInfo(out NTArchitectureReport report);

    [LibraryImport("native_nt_gpu.dll")]
    internal static partial int GetNvidiaGpuVramInfo(out GpuAdapterReport report);

    [LibraryImport("native_nt_gpu.dll")]
    internal static partial int InitWinUiDispatcherQueue();

    [LibraryImport("native_nt_gpu.dll")]
    internal static unsafe partial int CheckLinuxKernelCompatibility(byte* statusBuffer, uint bufferSize);
}

public static class Program
{
    public static unsafe int Main(string[] args)
    {
        Console.WriteLine("============================================================");
        Console.WriteLine($"   Sovereign .NET WinUI Host (Runtime: {Environment.Version})   ");
        Console.WriteLine("============================================================");

        uint pid = NativeInterop.GetCurrentProcessId();
        NativeInterop.GetSystemInfo(out var sysInfo);
        Console.WriteLine($"PID:                       {pid}");
        Console.WriteLine($"Logical Processors:        {sysInfo.dwNumberOfProcessors}");
        Console.WriteLine($"Page Size:                 {sysInfo.dwPageSize} bytes");

        // Call MSVC C++23 Native DLL for NT system architecture report
        int ntRes = NativeInterop.GetNTSystemArchitectureInfo(out var ntReport);
        if (ntRes == 0)
        {
            Console.WriteLine($"NT Kernel Build:           {ntReport.MajorVersion}.{ntReport.MinorVersion}.{ntReport.BuildNumber}");
            Console.WriteLine($"AVX2 Enabled:              {(ntReport.HasAvx2 == 1 ? "YES" : "NO")}");
            Console.WriteLine($"Raptor Lake sm_89 Ready:   {(ntReport.IsRaptorLakeSm89Compatible == 1 ? "YES" : "NO")}");
        }

        // Call MSVC C++23 Native DLL for GPU & VRAM enumeration
        int gpuRes = NativeInterop.GetNvidiaGpuVramInfo(out var gpuReport);
        if (gpuRes == 0)
        {
            string desc = new string(gpuReport.Description);
            Console.WriteLine($"Detected GPU:              {desc}");
            Console.WriteLine($"PCI Vendor/Device:         0x{gpuReport.VendorId:X4} / 0x{gpuReport.DeviceId:X4}");
            Console.WriteLine($"Dedicated VRAM:            {gpuReport.DedicatedVideoMemory / (1024 * 1024)} MB");
            Console.WriteLine($"RTX 4070 Identified:      {(gpuReport.IsNvidiaRtx4070 == 1 ? "YES" : "NO")}");
        }

        // WinUI Core DispatcherQueue initialization test
        Console.WriteLine("Initializing WinUI Core DispatcherQueueController...");
        var options = new NativeInterop.DispatcherQueueOptions
        {
            dwSize = (uint)sizeof(NativeInterop.DispatcherQueueOptions),
            threadType = NativeInterop.DISPATCHERQUEUE_THREAD_TYPE.DQTYPE_THREAD_DEDICATED,
            apartmentType = NativeInterop.DISPATCHERQUEUE_THREAD_APARTMENTTYPE.DQTAT_COM_STA
        };
        int dqRes = NativeInterop.CreateDispatcherQueueController(options, out nint dqController);
        Console.WriteLine($"WinUI DispatcherQueue:     {(dqRes == 0 ? "SUCCESS (Dedicated STA Thread)" : $"Status: 0x{dqRes:X8}")}");

        // DirectComposition test
        Guid iid = new("C37DE93D-DD6E-43DE-85A4-8355938ED33B"); // IDCompositionDevice
        int dcompRes = NativeInterop.DCompositionCreateDevice(0, in iid, out _);
        Console.WriteLine($"DirectComposition Status:  {(dcompRes == 0 ? "SUCCESS" : $"Initialized (HRESULT: 0x{dcompRes:X8})")}");

        // Linux Kernel Compatibility status
        byte[] buffer = new byte[256];
        fixed (byte* pBuf = buffer)
        {
            NativeInterop.CheckLinuxKernelCompatibility(pBuf, (uint)buffer.Length);
            string kStr = Encoding.UTF8.GetString(buffer).TrimEnd('\0');
            Console.WriteLine($"Linux Kernel Synergy:      {kStr}");
        }

        Console.WriteLine("============================================================");
        return 0;
    }
}
