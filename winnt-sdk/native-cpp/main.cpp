#include "native_nt_gpu.h"
#include <iostream>
#include <iomanip>

int main() {
    std::cout << "========================================================\n";
    std::cout << "   MSVC C++23 Native NT / GPU / WinUI Hardware Bridge   \n";
    std::cout << "========================================================\n";

    NTArchitectureReport nt_report = {};
    if (GetNTSystemArchitectureInfo(&nt_report) == 0) {
        std::cout << "Windows NT Kernel Version: " << nt_report.major_version << "."
                  << nt_report.minor_version << " (Build " << nt_report.build_number << ")\n";
        std::cout << "Processor Cores:           " << nt_report.number_of_processors << "\n";
        std::cout << "AVX2 Available:            " << (nt_report.has_avx2 ? "YES" : "NO") << "\n";
        std::cout << "Raptor Lake Compatible:    " << (nt_report.is_raptor_lake_sm89_compatible ? "YES" : "NO") << "\n";
    }

    GpuAdapterReport gpu_report = {};
    if (GetNvidiaGpuVramInfo(&gpu_report) == 0) {
        std::wcout << L"Primary GPU:               " << gpu_report.description << L"\n";
        std::cout << "PCI Vendor / Device:       0x" << std::hex << gpu_report.vendor_id
                  << " / 0x" << gpu_report.device_id << std::dec << "\n";
        std::cout << "Dedicated VRAM:            "
                  << (gpu_report.dedicated_video_memory / (1024 * 1024)) << " MB ("
                  << (gpu_report.dedicated_video_memory / (1024 * 1024 * 1024)) << " GB)\n";
        std::cout << "NVIDIA RTX 4070 Identified: " << (gpu_report.is_nvidia_rtx_4070 ? "YES" : "NO") << "\n";
    }

    std::cout << "Testing WinUI DispatcherQueue initialization...\n";
    int32_t dq_status = InitWinUiDispatcherQueue();
    std::cout << "DispatcherQueue Status:    " << (dq_status == 0 ? "SUCCESS (STA Dedicated Thread)" : "FAILED") << "\n";

    char kbuf[256] = {};
    CheckLinuxKernelCompatibility(kbuf, sizeof(kbuf));
    std::cout << "Linux Kernel Bridge:       " << kbuf << "\n";
    std::cout << "========================================================\n";

    return 0;
}
