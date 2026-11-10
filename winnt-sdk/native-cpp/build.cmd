@echo off
setlocal enabledelayedexpansion

call "C:\Program Files\Microsoft Visual Studio\18\Insiders\VC\Auxiliary\Build\vcvars64.bat"

cd /d "%~dp0"

echo [MSVC] Compiling native_nt_gpu.dll with /std:c++latest /arch:AVX2 /O2...
cl.exe /nologo /std:c++latest /arch:AVX2 /O2 /MD /EHsc /DNATIVE_EXPORTS /LD native_nt_gpu.cpp /link dxgi.lib ntdll.lib /OUT:native_nt_gpu.dll
if %errorlevel% neq 0 (
    echo [ERROR] Failed to compile native_nt_gpu.dll
    exit /b %errorlevel%
)

echo [MSVC] Compiling native_test.exe...
cl.exe /nologo /std:c++latest /arch:AVX2 /O2 /MD /EHsc main.cpp native_nt_gpu.lib /link /OUT:native_test.exe
if %errorlevel% neq 0 (
    echo [ERROR] Failed to compile native_test.exe
    exit /b %errorlevel%
)

echo [MSVC] Build succeeded. Running native_test.exe...
native_test.exe
