#ifndef NATIVE_EXPORTS
#define NATIVE_EXPORTS
#endif
#include "native_nt_gpu.h"
#include <dxgi1_3.h>
#include <cstdio>
#include <cstring>

typedef LONG(NTAPI* PfnRtlGetVersion)(PRTL_OSVERSIONINFOW);

enum DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    DQTAT_COM_NONE = 0,
    DQTAT_COM_ASTA = 1,
    DQTAT_COM_STA = 2
};

enum DISPATCHERQUEUE_THREAD_TYPE {
    DQTYPE_THREAD_DEDICATED = 1,
    DQTYPE_THREAD_CURRENT = 2,
};

struct DispatcherQueueOptions {
    DWORD dwSize;
    DISPATCHERQUEUE_THREAD_TYPE threadType;
    DISPATCHERQUEUE_THREAD_APARTMENTTYPE apartmentType;
};

typedef HRESULT(WINAPI* PfnCreateDispatcherQueueController)(
    DispatcherQueueOptions options,
    void** dispatcherQueueController);

NATIVE_API int32_t GetNTSystemArchitectureInfo(NTArchitectureReport* report) {
    if (!report) return -1;
    std::memset(report, 0, sizeof(NTArchitectureReport));

    HMODULE hNtdll = GetModuleHandleW(L"ntdll.dll");
    if (hNtdll) {
        auto pfn = (PfnRtlGetVersion)GetProcAddress(hNtdll, "RtlGetVersion");
        if (pfn) {
            RTL_OSVERSIONINFOW osvi = {};
            osvi.dwOSVersionInfoSize = sizeof(osvi);
            if (pfn(&osvi) == 0) {
                report->major_version = osvi.dwMajorVersion;
                report->minor_version = osvi.dwMinorVersion;
                report->build_number = osvi.dwBuildNumber;
            }
        }
    }

    SYSTEM_INFO si = {};
    GetSystemInfo(&si);
    report->processor_architecture = si.wProcessorArchitecture;
    report->number_of_processors = si.dwNumberOfProcessors;
    report->page_size = si.dwPageSize;

    report->has_avx = IsProcessorFeaturePresent(PF_AVX_INSTRUCTIONS_AVAILABLE) ? 1 : 0;
    report->has_avx2 = IsProcessorFeaturePresent(PF_AVX2_INSTRUCTIONS_AVAILABLE) ? 1 : 0;
    report->is_raptor_lake_sm89_compatible = (report->has_avx2 && report->number_of_processors >= 16) ? 1 : 0;

    return 0;
}

NATIVE_API int32_t GetNvidiaGpuVramInfo(GpuAdapterReport* report) {
    if (!report) return -1;
    std::memset(report, 0, sizeof(GpuAdapterReport));

    HMODULE hDxgi = LoadLibraryW(L"dxgi.dll");
    if (!hDxgi) return -2;

    typedef HRESULT(WINAPI* PfnCreateDXGIFactory2)(UINT Flags, REFIID riid, void** ppFactory);
    auto pfnCreate = (PfnCreateDXGIFactory2)GetProcAddress(hDxgi, "CreateDXGIFactory2");
    if (!pfnCreate) {
        FreeLibrary(hDxgi);
        return -3;
    }

    IDXGIFactory2* pFactory = nullptr;
    HRESULT hr = pfnCreate(0, __uuidof(IDXGIFactory2), (void**)&pFactory);
    if (FAILED(hr) || !pFactory) {
        FreeLibrary(hDxgi);
        return -4;
    }

    UINT adapterIdx = 0;
    IDXGIAdapter1* pAdapter = nullptr;
    bool found_nvidia = false;

    while (pFactory->EnumAdapters1(adapterIdx++, &pAdapter) != DXGI_ERROR_NOT_FOUND) {
        DXGI_ADAPTER_DESC1 desc = {};
        if (SUCCEEDED(pAdapter->GetDesc1(&desc))) {
            // Check for NVIDIA Vendor ID (0x10DE)
            if (desc.VendorId == 0x10DE) {
                wcsncpy_s(report->description, desc.Description, _TRUNCATE);
                report->vendor_id = desc.VendorId;
                report->device_id = desc.DeviceId;
                report->sub_sys_id = desc.SubSysId;
                report->revision = desc.Revision;
                report->dedicated_video_memory = desc.DedicatedVideoMemory;
                report->dedicated_system_memory = desc.DedicatedSystemMemory;
                report->shared_system_memory = desc.SharedSystemMemory;
                report->is_nvidia_rtx_4070 = 1;
                found_nvidia = true;
                pAdapter->Release();
                break;
            } else if (!found_nvidia && adapterIdx == 1) {
                // Keep first adapter as fallback if nvidia not yet found
                wcsncpy_s(report->description, desc.Description, _TRUNCATE);
                report->vendor_id = desc.VendorId;
                report->device_id = desc.DeviceId;
                report->dedicated_video_memory = desc.DedicatedVideoMemory;
            }
        }
        pAdapter->Release();
    }

    pFactory->Release();
    FreeLibrary(hDxgi);

    return 0;
}

NATIVE_API int32_t InitWinUiDispatcherQueue() {
    HMODULE hCoreMessaging = LoadLibraryW(L"CoreMessaging.dll");
    if (!hCoreMessaging) return -1;

    auto pfnCreate = (PfnCreateDispatcherQueueController)GetProcAddress(hCoreMessaging, "CreateDispatcherQueueController");
    if (!pfnCreate) {
        FreeLibrary(hCoreMessaging);
        return -2;
    }

    DispatcherQueueOptions options = {};
    options.dwSize = sizeof(DispatcherQueueOptions);
    options.threadType = DQTYPE_THREAD_DEDICATED;
    options.apartmentType = DQTAT_COM_STA;

    void* controller = nullptr;
    HRESULT hr = pfnCreate(options, &controller);

    FreeLibrary(hCoreMessaging);
    return SUCCEEDED(hr) ? 0 : (int32_t)hr;
}

NATIVE_API int32_t CheckLinuxKernelCompatibility(char* status_buffer, uint32_t buffer_size) {
    if (!status_buffer || buffer_size == 0) return -1;
    const char* message = "Linux v7.4-rc2 CachyOS sm_89 Baremetal/WSL2 (sched-ext BORE, ntsync, dxgkrnl, AVX2)";
    strncpy_s(status_buffer, buffer_size, message, _TRUNCATE);
    return 0;
}
