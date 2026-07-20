#!/bin/bash
# VoiceDemo - macOS 打包脚本

echo "============================================"
echo "  VoiceDemo - macOS 安装包构建"
echo "============================================"

# 检查环境
echo "[1/5] 检查环境..."
if ! command -v rustc &> /dev/null; then
    echo "❌ 未安装 Rust！请执行: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo "  ✅ Rust: $(rustc --version)"

if ! command -v node &> /dev/null; then
    echo "❌ 未安装 Node.js！"
    exit 1
fi
echo "  ✅ Node.js: $(node --version)"

# 检查 Homebrew 依赖
echo "[2/5] 检查系统依赖..."
if ! command -v brew &> /dev/null; then
    echo "❌ 未安装 Homebrew！请执行: /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
    exit 1
fi
brew list portaudio &>/dev/null || brew install portaudio
echo "  ✅ 系统依赖就绪"

# 检查模型
echo "[3/5] 检查模型..."
if [ ! -f "src-tauri/models/sensevoice.onnx" ]; then
    echo "⚠️ 未检测到 sensevoice.onnx 模型文件"
    echo "   请放入 src-tauri/models/ 目录"
    echo "   下载: https://github.com/FunAudioLLM/SenseVoice"
    read -p "是否继续？(y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then exit 1; fi
fi

# 安装依赖
echo "[4/5] 安装前端依赖..."
npm install || { echo "❌ npm install 失败"; exit 1; }

# 构建
echo "[5/5] 构建安装包..."
npm run tauri build || { echo "❌ 构建失败"; exit 1; }

echo ""
echo "============================================"
echo "  ✅ 打包成功！"
echo "============================================"
echo "安装包: src-tauri/target/release/bundle/dmg/"
echo "  .dmg - 磁盘映像安装包"
echo "  .app - 应用程序包"
echo ""
