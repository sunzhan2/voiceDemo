@echo off
chcp 65001 >nul
title VoiceDemo 跨平台打包工具
color 0B

echo ============================================
echo   VoiceDemo - 跨平台语音转文字 打包工具
echo ============================================
echo.
echo 注意：Tauri 不支持交叉编译！
echo 本脚本只能在当前平台生成对应平台的安装包。
echo.
echo   Windows 10+ → 生成 .msi/.exe
echo   macOS 12+  → 请运行 build-macos.sh
echo   Linux      → 请运行 build-linux.sh
echo.
echo 如需一键生成三平台安装包，推荐使用 GitHub Actions
echo （已提供 .github/workflows/build.yml）
echo.
echo ============================================
echo.

:check_deps
echo [1/5] 检查环境依赖...
where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] 未安装 Rust！
    echo 请访问 https://rustup.rs 下载安装
    pause
    exit /b 1
)
echo   ✅ Rust: %errorlevel%

where node >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] 未安装 Node.js！
    pause
    exit /b 1
)
echo   ✅ Node.js

where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] 未找到 cargo！
    pause
    exit /b 1
)
echo   ✅ Cargo

echo.

:check_model
echo [2/5] 检查语音模型...
if not exist "src-tauri\models\sensevoice.onnx" (
    echo [警告] 未检测到 sensevoice.onnx 模型文件！
    echo 请将模型文件放入 src-tauri\models\ 目录
    echo 下载地址: https://github.com/FunAudioLLM/SenseVoice
    echo.
    choice /c YN /m "是否继续（构建后无法使用语音识别）？"
    if errorlevel 2 exit /b 1
) else (
    echo   ✅ 模型文件已就绪
)
echo.

:install_deps
echo [3/5] 安装前端依赖...
call npm install
if %errorlevel% neq 0 (
    echo [错误] npm install 失败！
    pause
    exit /b 1
)
echo   ✅ 前端依赖安装完成
echo.

:build_frontend
echo [4/5] 构建前端...
call npm run build
if %errorlevel% neq 0 (
    echo [错误] 前端构建失败！
    pause
    exit /b 1
)
echo   ✅ 前端构建完成
echo.

:build_tauri
echo [5/5] 编译 Rust + 生成安装包...
echo 提示：首次编译会下载大量依赖，可能需要较长时间
echo.
npm run tauri build
if %errorlevel% neq 0 (
    echo [错误] 打包失败！
    pause
    exit /b 1
)
echo.
echo ============================================
echo   ✅ 打包成功！
echo ============================================
echo.
echo 安装包位置:
echo   src-tauri\target\release\bundle\
echo.
echo 请查看对应平台目录:
echo   msi\    - Windows 安装包
echo   nsis\   - Windows 安装包（NSIS）
echo.
pause
