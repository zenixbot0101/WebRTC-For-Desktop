# 🚀 WebRTC Desktop Streamer (Rust + NVENC)

A high-performance Windows desktop streaming server written in Rust.  
Designed similarly to Steam Link, Parsec, and Moonlight — but built entirely using:

- DXGI Desktop Duplication
- NVIDIA NVENC
- webrtc-rs
- WebSocket signaling

Supports low-latency desktop streaming directly to a browser.

---

# ✨ Features

- ⚡ Ultra-low latency (~50ms achievable on LAN)
- 🎮 NVIDIA NVENC hardware encoding (H.264)
- 🖥 DXGI desktop capture (zero-copy GPU capture)
- 🌐 WebRTC streaming
- 📡 TWCC bandwidth estimation
- 🔄 Safe reconnect support
- ✍ Touch / pen passthrough (InjectSyntheticPointerInput)
- 🎯 Auto profile fallback for unsupported NVENC configs

---

# 🖥 System Requirements

## Required

- Windows 10 / 11
- NVIDIA GPU with NVENC support
- Latest NVIDIA drivers
- Rust (stable toolchain)
- Visual Studio Build Tools (MSVC)

## Not Supported

- ❌ Linux server
- ❌ macOS
- ❌ GitHub Codespaces (no GPU access)
- ❌ Systems without NVENC

---

# 📦 Installation

 1️⃣ Install Rust

https://rustup.rs/

Verify:

```bash
rustc --version
2️⃣ Install Visual Studio Build Tools
Install:

Desktop development with C++

MSVC toolchain

Verify:

cl
3️⃣ Clone Project
git clone --recursive <your-repository-url>
cd desktop-streaming
4️⃣ Build
cargo build --release
5️⃣ Run
cargo run --release
Open browser:

http://YOUR_PC_IP:9090
Example:

http://192.168.1.5:9090
🎯 Performance
Typical latency breakdown (LAN):

Stage	Approx
NVENC encode	~16ms
Network	<1ms
Browser decode	~30ms
Total	~50ms
Actual latency depends on:

GPU generation

Display refresh rate

Network conditions

Browser performance

🧠 How It Works
Capture
Uses:

IDXGIOutputDuplication

Captures GPU framebuffer directly without CPU screen copy.

Encoding
Uses:

NVIDIA NVENC via nvenc-rs

Configuration:

H.264

UltraLowLatency tuning

Preset P4 (with safe fallback)

Dynamic bitrate update via TWCC

If a requested codec profile is unsupported,
the encoder safely falls back instead of panicking.

Streaming Pipeline
Desktop → DXGI → NVENC → NAL units → RTP → WebRTC → Browser
Input Passthrough
Browser sends PointerEvent.

Server injects synthetic input using:

InjectSyntheticPointerInput

Supports:

Touch

Pen

Stylus pressure

🔐 Signaling
WebRTC signaling via WebSocket

Future plan: TLS + authentication

⚠ Common Errors
❌ Invalid codec guid
Caused by:

Unsupported NVENC profile

Old NVIDIA driver

No GPU available

Running in Codespaces

Fix:

Update drivers

Ensure GPU supports H.264 NVENC

Run on real Windows machine with NVIDIA GPU

❌ 'cl' not recognized
Install Visual Studio Build Tools with MSVC.

❌ libclang not found
Install LLVM and set:

set LIBCLANG_PATH=C:\Program Files\LLVM\bin
📌 Current Limitations
H.264 only

No audio streaming

No HEVC

No AV1

Windows only

🗺 Roadmap
 Firefox support

 Native client (lower latency)

 HEVC (H.265)

 AV1

 Audio streaming (Opus)

 Gamepad support

 TLS encryption

 Authentication

 Linux server (PipeWire)

 AMD / Intel encoder support

 CPU fallback (x264)

🛠 Architecture
+--------------------+
| DXGI Capture       |
+--------------------+
           ↓
+--------------------+
| NVENC Encoder      |
+--------------------+
           ↓
+--------------------+
| RTP Fragmentation  |
+--------------------+
           ↓
+--------------------+
| WebRTC (webrtc-rs) |
+--------------------+
           ↓
+--------------------+
| Browser            |
+--------------------+
📜 License
MIT (or same as original project license)

💡 Notes
This project is intended for:

Local network streaming

Remote desktop experiments

Cloud gaming infrastructure research

Low-latency GPU streaming systems

Not production-ready yet.

Built with ❤️ using Rust.
