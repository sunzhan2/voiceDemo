# download-model.ps1 - 下载 SenseVoice 模型到 src-tauri/models/
#
# 用法：在项目根目录运行 PowerShell
#   .\download-model.ps1
#
# 需要能访问 GitHub，如果连不上请挂代理或使用镜像。

$ErrorActionPreference = "Stop"

$modelDir = Join-Path $PSScriptRoot "src-tauri\models"
if (!(Test-Path $modelDir)) {
    New-Item -ItemType Directory -Path $modelDir -Force | Out-Null
}

$url = "https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-sense-voice-zh-en-ja-ko-yue-int8-2024-07-17.tar.bz2"
$tarball = Join-Path $modelDir "sensevoice.tar.bz2"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " SenseVoice 模型下载脚本" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "目标目录: $modelDir"
Write-Host "下载地址: $url"
Write-Host ""

# 检查是否已下载
$modelFile = Join-Path $modelDir "model.int8.onnx"
$tokensFile = Join-Path $modelDir "tokens.txt"

if ((Test-Path $modelFile) -and (Test-Path $tokensFile)) {
    Write-Host "模型文件已存在，跳过下载。" -ForegroundColor Green
    Write-Host "  model.int8.onnx: $((Get-Item $modelFile).Length / 1MB) MB"
    Write-Host "  tokens.txt:      $((Get-Item $tokensFile).Length / 1KB) KB"
    exit 0
}

Write-Host "正在下载 SenseVoice 模型（约 230MB）..." -ForegroundColor Yellow

# 下载
try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri $url -OutFile $tarball -UseBasicParsing
} catch {
    Write-Host "下载失败: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "请尝试以下方法：" -ForegroundColor Yellow
    Write-Host "  1. 挂代理后重试"
    Write-Host "  2. 手动下载到 src-tauri/models/ 目录"
    Write-Host "  3. 使用镜像站下载"
    exit 1
}

Write-Host "下载完成，正在解压..." -ForegroundColor Green

# 解压
tar xjf $tarball -C $modelDir

# 移动文件到 models/ 根目录
$extractedDir = Join-Path $modelDir "sherpa-onnx-sense-voice-zh-en-ja-ko-yue-int8-2024-07-17"
if (Test-Path $extractedDir) {
    Copy-Item (Join-Path $extractedDir "model.int8.onnx") $modelDir -Force
    Copy-Item (Join-Path $extractedDir "tokens.txt") $modelDir -Force
    Remove-Item $extractedDir -Recurse -Force
}

# 清理 tarball
Remove-Item $tarball -Force -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host " 模型下载完成！" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "模型文件："
Get-ChildItem $modelDir | Where-Object { $_.Name -match "\.(onnx|txt)$" } | ForEach-Object {
    Write-Host "  $($_.Name) - $([math]::Round($_.Length / 1MB, 1)) MB"
}
Write-Host ""
Write-Host "现在可以运行 'npm run tauri dev' 启动应用了。"
