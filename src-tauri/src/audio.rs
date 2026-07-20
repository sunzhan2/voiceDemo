use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{self, SyncSender};
use std::sync::{Arc, Mutex};

/// Control messages sent to the dedicated audio thread.
enum AudioCmd {
    Start(SyncSender<Result<u32, String>>),
    Stop(SyncSender<Result<Vec<f32>, String>>),
}

/// Handle to a dedicated audio worker thread.
///
/// The `cpal::Stream` type is `!Send + !Sync` on some platforms (it holds
/// raw pointers / callbacks). Tauri's `.manage()` requires `Send + Sync`,
/// so we keep the Stream entirely inside a background thread and expose
/// only a channel-based control handle here, which IS `Send + Sync`.
pub struct AudioCapture {
    cmd_tx: mpsc::Sender<AudioCmd>,
    is_recording: Arc<AtomicBool>,
    sample_rate: Arc<AtomicU32>,
}

impl AudioCapture {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel::<AudioCmd>();
        let is_recording = Arc::new(AtomicBool::new(false));
        let sample_rate = Arc::new(AtomicU32::new(0));

        let is_recording_worker = is_recording.clone();
        let sample_rate_worker = sample_rate.clone();

        std::thread::spawn(move || {
            let buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
            let mut current: Option<Stream> = None;

            while let Ok(cmd) = cmd_rx.recv() {
                match cmd {
                    AudioCmd::Start(reply) => {
                        // Tear down any prior stream first.
                        current.take();

                        let result: Result<u32, String> = (|| {
                            let host = cpal::default_host();
                            let device = host
                                .default_input_device()
                                .ok_or_else(|| "未找到麦克风设备".to_string())?;
                            let config: StreamConfig = device
                                .default_input_config()
                                .map_err(|e| format!("无法获取输入配置: {}", e))?
                                .into();
                            let sr = config.sample_rate.0;

                            buffer.lock().unwrap().clear();
                            is_recording_worker.store(true, Ordering::SeqCst);

                            let buf_cb = buffer.clone();
                            let rec_cb = is_recording_worker.clone();
                            let err_fn = move |err| eprintln!("音频流错误: {}", err);

                            let stream = device
                                .build_input_stream(
                                    &config,
                                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                                        if rec_cb.load(Ordering::SeqCst) {
                                            if let Ok(mut b) = buf_cb.lock() {
                                                b.extend_from_slice(data);
                                            }
                                        }
                                    },
                                    err_fn,
                                    None,
                                )
                                .map_err(|e| format!("无法构建音频流: {}", e))?;

                            stream
                                .play()
                                .map_err(|e| format!("无法启动音频流: {}", e))?;

                            sample_rate_worker.store(sr, Ordering::SeqCst);
                            current = Some(stream);
                            Ok(sr)
                        })();

                        if result.is_err() {
                            is_recording_worker.store(false, Ordering::SeqCst);
                        }
                        let _ = reply.send(result);
                    }
                    AudioCmd::Stop(reply) => {
                        is_recording_worker.store(false, Ordering::SeqCst);

                        let pause_result = if let Some(s) = current.take() {
                            s.pause()
                                .map_err(|e| format!("无法暂停音频流: {}", e))
                        } else {
                            Ok(())
                        };

                        let captured = buffer.lock().unwrap().clone();
                        buffer.lock().unwrap().clear();

                        let result = pause_result.map(|_| captured);
                        let _ = reply.send(result);
                    }
                }
            }
        });

        Self {
            cmd_tx,
            is_recording,
            sample_rate,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let (tx, rx) = mpsc::sync_channel::<Result<u32, String>>(1);
        self.cmd_tx
            .send(AudioCmd::Start(tx))
            .map_err(|e| format!("音频线程已退出: {}", e))?;
        rx.recv()
            .map_err(|e| format!("音频线程无响应: {}", e))??;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<Vec<f32>, String> {
        let (tx, rx) = mpsc::sync_channel::<Result<Vec<f32>, String>>(1);
        self.cmd_tx
            .send(AudioCmd::Stop(tx))
            .map_err(|e| format!("音频线程已退出: {}", e))?;
        rx.recv().map_err(|e| format!("音频线程无响应: {}", e))?
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate.load(Ordering::SeqCst)
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        self.is_recording.store(false, Ordering::SeqCst);
        // Dropping cmd_tx will end the worker's `recv()` loop.
    }
}
