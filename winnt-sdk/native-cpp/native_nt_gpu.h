#pragma once

#include <windows.h>
#include <cstdint>

#ifdef NATIVE_EXPORTS
#define NATIVE_API extern "C" __declspec(dllexport)
#else
#define NATIVE_API extern "C" __declspec(dllimport)
#endif

#pragma pack(push, 8)
struct NTArchitectureReport {
    uint32_t major_version;
    uint32_t minor_version;
    uint32_t build_number;
    uint32_t processor_architecture;
    uint32_t number_of_processors;
    uint32_t page_size;
    uint32_t has_avx2;
    uint32_t has_avx;
    uint32_t is_raptor_lake_sm89_compatible;
};

struct GpuAdapterReport {
    wchar_t description[128];
    uint32_t vendor_id;
    uint32_t device_id;
    uint32_t sub_sys_id;
    uint32_t revision;
    uint64_t dedicated_video_memory;
    uint64_t dedicated_system_memory;
    uint64_t shared_system_memory;
    uint32_t is_nvidia_rtx_4070;
};
#pragma pack(pop)

NATIVE_API int32_t GetNTSystemArchitectureInfo(NTArchitectureReport* report);
NATIVE_API int32_t GetNvidiaGpuVramInfo(GpuAdapterReport* report);
NATIVE_API int32_t InitWinUiDispatcherQueue();
NATIVE_API int32_t CheckLinuxKernelCompatibility(char* status_buffer, uint32_t buffer_size);
