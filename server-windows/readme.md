# 🖥 server-windows

Windows-specific streaming backend for WebRTC Desktop Streamer.

This crate contains the native Windows implementation responsible for:

- Desktop capture (DXGI Output Duplication)
- NVIDIA NVENC hardware encoding
- RTP packetization
- WebRTC media transmission
- Synthetic touch / pen input injection

---

# ⚙ Architecture Overview

DXGI → NVENC → RTP → WebRTC → Browser

### Capture Layer
Uses:
- IDXGIOutputDuplication (DXGI 1.2)

Captures the GPU framebuffer directly without copying through CPU memory.

---

### Encoding Layer
Uses:
- NVIDIA NVENC
- nvenc-rs bindings

Configuration:
- H.264
- Ultra-low latency tuning
- Dynamic bitrate updates (TWCC)
- IDR frame trigger via RTCP (PLI / FIR)

If unsupported NVENC profiles are detected, safe fallback is applied.

---

### RTP Layer
- H264 NAL fragmentation
- MTU-safe packet splitting (~1200 bytes)
- Timestamp synchronization via QueryPerformanceCounter
- SSRC configurable

---

### WebRTC Layer
Uses:
- webrtc-rs
- RTCRtpTransceiver
- TrackLocalStaticRTP
- TWCC bandwidth estimation

Handles:
- ICE state monitoring
- RTCP PLI / FIR
- Bitrate adaptation
- Safe reconnect handling

---

### Input Injection
Uses:
- InjectSyntheticPointerInput

Supports:
- Touch
- Pen
- Stylus pressure

---

# 🖥 System Requirements

- Windows 10 / 11
- NVIDIA GPU with NVENC support
- Latest NVIDIA drivers
- Rust stable toolchain
- MSVC build tools

Not supported:
- Linux
- macOS
- GitHub Codespaces (no GPU)
- CPU-only systems

---

# 🛠 Build

From project root:

cargo build --release


Run:

cargo run --release


Server will bind to:

http://0.0.0.0:9090


Access via:

http://YOUR_PC_IP:9090


---

# ⚠ Common Issues

## Invalid codec guid
Cause:
- Unsupported NVENC profile
- Old driver
- No GPU available

Fix:
- Update NVIDIA driver
- Verify NVENC support
- Run on physical Windows machine

---

## 'cl' not recognized
Install Visual Studio Build Tools (Desktop development with C++).

---

## Build script errors (proc-macro, winapi, syn, etc.)
Cause:
- MSVC not installed
- Wrong Rust target

Fix:
rustup default stable-x86_64-pc-windows-msvc


---

# 📊 Performance

Typical LAN latency:

- ~16 ms encode
- <1 ms network
- ~30 ms browser decode

Total: ~50 ms

Latency depends on:
- GPU model
- Display refresh rate
- Network
- Browser

---

# 🚧 Current Limitations

- H.264 only
- No audio streaming
- No HEVC
- No AV1
- Windows only
- No authentication
- No TLS

---

# 🗺 Roadmap

- HEVC support
- AV1 support
- Audio streaming (Opus)
- Gamepad support
- TLS encryption
- Authentication
- Linux server (PipeWire)
- AMD / Intel encoder support
- CPU fallback

---

# 📜 License

Dual licensed under MIT OR Apache-2.0.
