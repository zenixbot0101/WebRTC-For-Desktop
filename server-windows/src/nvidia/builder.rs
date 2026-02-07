use super::encoder::start_encoder;
use crate::{capture::ScreenDuplicator, device::create_d3d11_device};
use std::{collections::HashMap, sync::Arc};
use webrtc::{
    rtp_transceiver::{rtp_codec::RTCRtpCodecCapability, RTCRtpTransceiver},
    track::track_local::track_local_static_rtp::TrackLocalStaticRTP,
};
use webrtc_helper::{
    codecs::{Codec, CodecType, H264Codec, H264Profile},
    encoder::EncoderBuilder,
    interceptor::twcc::TwccBandwidthEstimate,
    peer::IceConnectionState,
};
use windows::Win32::Graphics::{
    Direct3D11::ID3D11Device,
    Dxgi::Common::{
        DXGI_FORMAT, DXGI_FORMAT_B8G8R8A8_UNORM,
        DXGI_FORMAT_R10G10B10A2_UNORM,
        DXGI_FORMAT_R8G8B8A8_UNORM,
    },
};

pub struct NvidiaEncoderBuilder {
    inner_builder: nvenc::EncoderBuilder<nvenc::DirectX11Device>,
    device: ID3D11Device,
    id: String,
    stream_id: String,
    display_index: u32,
    display_formats: Vec<DXGI_FORMAT>,
    supported_codecs: Vec<Codec>,
}

impl EncoderBuilder for NvidiaEncoderBuilder {

    fn id(&self) -> &str { &self.id }

    fn stream_id(&self) -> &str { &self.stream_id }

    fn codec_type(&self) -> CodecType {
        CodecType::Video
    }

    fn supported_codecs(&self) -> &[Codec] {
        &self.supported_codecs
    }

    fn build(
        mut self: Box<Self>,
        rtp_track: Arc<TrackLocalStaticRTP>,
        transceiver: Arc<RTCRtpTransceiver>,
        ice_connection_state: IceConnectionState,
        bandwidth_estimate: TwccBandwidthEstimate,
        codec_capability: RTCRtpCodecCapability,
        ssrc: u32,
        payload_type: u8,
    ) {

        // ================= SAFE CODEC =================

        let codec = nvenc::Codec::H264;

        if let Err(e) = self.inner_builder.with_codec(codec) {
            panic!("H264 not supported by NVENC: {e}");
        }

        // ================= SAFE PROFILE =================

        let mut profile = h264_profile_from_sdp_fmtp_line(
            &codec_capability.sdp_fmtp_line
        ).unwrap_or(nvenc::CodecProfile::H264Main);

        let supported_profiles =
            self.inner_builder
                .supported_codec_profiles(codec)
                .unwrap_or_default();

        if !supported_profiles.contains(&profile) {
            log::warn!("Profile {:?} not supported. Fallback to Main", profile);
            profile = nvenc::CodecProfile::H264Main;
        }

        // ================= SAFE PRESET =================

        let supported_presets =
            self.inner_builder
                .supported_encode_presets(codec)
                .unwrap_or_default();

        let preset = if supported_presets.contains(&nvenc::EncodePreset::LowLatencyDefault) {
            nvenc::EncodePreset::LowLatencyDefault
        } else if supported_presets.contains(&nvenc::EncodePreset::LowLatencyHQ) {
            nvenc::EncodePreset::LowLatencyHQ
        } else if supported_presets.contains(&nvenc::EncodePreset::P4) {
            nvenc::EncodePreset::P4
        } else {
            supported_presets
                .first()
                .copied()
                .unwrap_or(nvenc::EncodePreset::LowLatencyDefault)
        };

        let tuning_info = nvenc::TuningInfo::UltraLowLatency;
        let multi_pass = nvenc::MultiPassSetting::Disabled;

        log::info!(
            "Using H264 profile {:?}, preset {:?}",
            profile,
            preset
        );

        // ================= CONFIGURE =================

        self.inner_builder
            .with_codec_profile(profile)
            .expect("Profile set failed");

        self.inner_builder
            .with_encode_preset(preset)
            .expect("Preset set failed");

        self.inner_builder
            .with_tuning_info(tuning_info)
            .expect("Tuning set failed");

        self.inner_builder
            .set_multi_pass(multi_pass)
            .expect("Multipass set failed");

        // ================= SCREEN =================

        let screen_duplicator =
            ScreenDuplicator::new(
                self.device,
                self.display_index,
                self.display_formats,
            ).expect("Failed to create ScreenDuplicator");

        let display_desc = screen_duplicator.desc();
        let mode_desc = &display_desc.ModeDesc;

        let width = mode_desc.Width;
        let height = mode_desc.Height;
        let texture_format = mode_desc.Format;

        // ================= BUILD ENCODER =================

        let (input, output) =
            self.inner_builder
                .build(width, height, texture_format)
                .expect("Failed to build NVENC encoder");

        // ================= START =================

        let handle = tokio::runtime::Handle::current();

        handle.spawn(start_encoder(
            screen_duplicator,
            input,
            output,
            rtp_track,
            transceiver,
            ice_connection_state,
            bandwidth_estimate,
            payload_type,
            ssrc,
            codec_capability.clock_rate,
        ));
    }
}

impl NvidiaEncoderBuilder {

    pub fn new(id: String, stream_id: String) -> NvidiaEncoderBuilder {

        let device =
            create_d3d11_device()
                .expect("Failed to create D3D11 device");

        let mut inner_builder =
            nvenc::EncoderBuilder::new(device.clone())
                .expect("Failed to create NVENC builder");

        inner_builder.repeat_csd(true).unwrap();

        let display_formats = vec![
            DXGI_FORMAT_B8G8R8A8_UNORM,
            DXGI_FORMAT_R10G10B10A2_UNORM,
            DXGI_FORMAT_R8G8B8A8_UNORM,
        ];

        let supported_codecs =
            list_supported_codecs(&mut inner_builder)
                .unwrap_or_default();

        NvidiaEncoderBuilder {
            inner_builder,
            device,
            id,
            stream_id,
            display_index: 0,
            display_formats,
            supported_codecs,
        }
    }
}

fn list_supported_codecs(
    inner_builder: &mut nvenc::EncoderBuilder<nvenc::DirectX11Device>,
) -> nvenc::Result<Vec<Codec>> {

    let mut codecs = Vec::new();

    for codec in inner_builder.supported_codecs()? {

        if codec == nvenc::Codec::H264 {

            let profiles =
                inner_builder.supported_codec_profiles(codec)?;

            for profile in profiles {

                match profile {
                    nvenc::CodecProfile::H264Baseline =>
                        codecs.push(H264Codec::new(H264Profile::Baseline).into()),
                    nvenc::CodecProfile::H264Main =>
                        codecs.push(H264Codec::new(H264Profile::Main).into()),
                    nvenc::CodecProfile::H264High =>
                        codecs.push(H264Codec::new(H264Profile::High).into()),
                    nvenc::CodecProfile::H264ConstrainedHigh =>
                        codecs.push(H264Codec::new(H264Profile::ConstrainedHigh).into()),
                    _ => {}
                }
            }
        }
    }

    Ok(codecs)
}
