SenseVoice model files (model.int8.onnx + tokens.txt) are downloaded by the build workflow at build time.
Do NOT commit them here — they are ~230MB.

Local development:
  Run download-model.ps1 (Windows) or the equivalent curl command.

CI:
  See .github/workflows/build.yml
