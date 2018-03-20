// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

extern crate gstreamer_video_sys;
extern crate shell_words;
extern crate tempdir;
use std::env;
use std::error::Error;
use std::path::Path;
use std::mem::{align_of, size_of};
use std::process::Command;
use std::str;
use gstreamer_video_sys::*;

static PACKAGES: &[&str] = &["gstreamer-video-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var), 
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}",
                               &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", 
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout)?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed,
            self.failed,
            self.failed_to_compile)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!("1",
               get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
               "failed to obtain correct constant value for 1");

    let mut results : Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_value, c_value);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(Layout {size: 1, alignment: 1},
               get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
               "failed to obtain correct layout for char type");

    let mut results : Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_layout, &c_layout);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout {size, alignment})
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    Ok(str::from_utf8(&output.stdout)?.to_owned())
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("GstColorBalanceChannel", Layout {size: size_of::<GstColorBalanceChannel>(), alignment: align_of::<GstColorBalanceChannel>()}),
    ("GstColorBalanceChannelClass", Layout {size: size_of::<GstColorBalanceChannelClass>(), alignment: align_of::<GstColorBalanceChannelClass>()}),
    ("GstColorBalanceInterface", Layout {size: size_of::<GstColorBalanceInterface>(), alignment: align_of::<GstColorBalanceInterface>()}),
    ("GstColorBalanceType", Layout {size: size_of::<GstColorBalanceType>(), alignment: align_of::<GstColorBalanceType>()}),
    ("GstNavigationCommand", Layout {size: size_of::<GstNavigationCommand>(), alignment: align_of::<GstNavigationCommand>()}),
    ("GstNavigationEventType", Layout {size: size_of::<GstNavigationEventType>(), alignment: align_of::<GstNavigationEventType>()}),
    ("GstNavigationInterface", Layout {size: size_of::<GstNavigationInterface>(), alignment: align_of::<GstNavigationInterface>()}),
    ("GstNavigationMessageType", Layout {size: size_of::<GstNavigationMessageType>(), alignment: align_of::<GstNavigationMessageType>()}),
    ("GstNavigationQueryType", Layout {size: size_of::<GstNavigationQueryType>(), alignment: align_of::<GstNavigationQueryType>()}),
    ("GstVideoAffineTransformationMeta", Layout {size: size_of::<GstVideoAffineTransformationMeta>(), alignment: align_of::<GstVideoAffineTransformationMeta>()}),
    ("GstVideoAlignment", Layout {size: size_of::<GstVideoAlignment>(), alignment: align_of::<GstVideoAlignment>()}),
    ("GstVideoAlphaMode", Layout {size: size_of::<GstVideoAlphaMode>(), alignment: align_of::<GstVideoAlphaMode>()}),
    ("GstVideoBufferFlags", Layout {size: size_of::<GstVideoBufferFlags>(), alignment: align_of::<GstVideoBufferFlags>()}),
    ("GstVideoBufferPool", Layout {size: size_of::<GstVideoBufferPool>(), alignment: align_of::<GstVideoBufferPool>()}),
    ("GstVideoBufferPoolClass", Layout {size: size_of::<GstVideoBufferPoolClass>(), alignment: align_of::<GstVideoBufferPoolClass>()}),
    ("GstVideoChromaFlags", Layout {size: size_of::<GstVideoChromaFlags>(), alignment: align_of::<GstVideoChromaFlags>()}),
    ("GstVideoChromaMethod", Layout {size: size_of::<GstVideoChromaMethod>(), alignment: align_of::<GstVideoChromaMethod>()}),
    ("GstVideoChromaMode", Layout {size: size_of::<GstVideoChromaMode>(), alignment: align_of::<GstVideoChromaMode>()}),
    ("GstVideoChromaSite", Layout {size: size_of::<GstVideoChromaSite>(), alignment: align_of::<GstVideoChromaSite>()}),
    ("GstVideoCodecFrame", Layout {size: size_of::<GstVideoCodecFrame>(), alignment: align_of::<GstVideoCodecFrame>()}),
    ("GstVideoCodecFrameFlags", Layout {size: size_of::<GstVideoCodecFrameFlags>(), alignment: align_of::<GstVideoCodecFrameFlags>()}),
    ("GstVideoCodecState", Layout {size: size_of::<GstVideoCodecState>(), alignment: align_of::<GstVideoCodecState>()}),
    ("GstVideoColorMatrix", Layout {size: size_of::<GstVideoColorMatrix>(), alignment: align_of::<GstVideoColorMatrix>()}),
    ("GstVideoColorPrimaries", Layout {size: size_of::<GstVideoColorPrimaries>(), alignment: align_of::<GstVideoColorPrimaries>()}),
    ("GstVideoColorPrimariesInfo", Layout {size: size_of::<GstVideoColorPrimariesInfo>(), alignment: align_of::<GstVideoColorPrimariesInfo>()}),
    ("GstVideoColorRange", Layout {size: size_of::<GstVideoColorRange>(), alignment: align_of::<GstVideoColorRange>()}),
    ("GstVideoColorimetry", Layout {size: size_of::<GstVideoColorimetry>(), alignment: align_of::<GstVideoColorimetry>()}),
    ("GstVideoCropMeta", Layout {size: size_of::<GstVideoCropMeta>(), alignment: align_of::<GstVideoCropMeta>()}),
    ("GstVideoDecoder", Layout {size: size_of::<GstVideoDecoder>(), alignment: align_of::<GstVideoDecoder>()}),
    ("GstVideoDecoderClass", Layout {size: size_of::<GstVideoDecoderClass>(), alignment: align_of::<GstVideoDecoderClass>()}),
    ("GstVideoDirectionInterface", Layout {size: size_of::<GstVideoDirectionInterface>(), alignment: align_of::<GstVideoDirectionInterface>()}),
    ("GstVideoDitherFlags", Layout {size: size_of::<GstVideoDitherFlags>(), alignment: align_of::<GstVideoDitherFlags>()}),
    ("GstVideoDitherMethod", Layout {size: size_of::<GstVideoDitherMethod>(), alignment: align_of::<GstVideoDitherMethod>()}),
    ("GstVideoEncoder", Layout {size: size_of::<GstVideoEncoder>(), alignment: align_of::<GstVideoEncoder>()}),
    ("GstVideoEncoderClass", Layout {size: size_of::<GstVideoEncoderClass>(), alignment: align_of::<GstVideoEncoderClass>()}),
    ("GstVideoFieldOrder", Layout {size: size_of::<GstVideoFieldOrder>(), alignment: align_of::<GstVideoFieldOrder>()}),
    ("GstVideoFilter", Layout {size: size_of::<GstVideoFilter>(), alignment: align_of::<GstVideoFilter>()}),
    ("GstVideoFilterClass", Layout {size: size_of::<GstVideoFilterClass>(), alignment: align_of::<GstVideoFilterClass>()}),
    ("GstVideoFlags", Layout {size: size_of::<GstVideoFlags>(), alignment: align_of::<GstVideoFlags>()}),
    ("GstVideoFormat", Layout {size: size_of::<GstVideoFormat>(), alignment: align_of::<GstVideoFormat>()}),
    ("GstVideoFormatFlags", Layout {size: size_of::<GstVideoFormatFlags>(), alignment: align_of::<GstVideoFormatFlags>()}),
    ("GstVideoFormatInfo", Layout {size: size_of::<GstVideoFormatInfo>(), alignment: align_of::<GstVideoFormatInfo>()}),
    ("GstVideoFrame", Layout {size: size_of::<GstVideoFrame>(), alignment: align_of::<GstVideoFrame>()}),
    ("GstVideoFrameFlags", Layout {size: size_of::<GstVideoFrameFlags>(), alignment: align_of::<GstVideoFrameFlags>()}),
    ("GstVideoFrameMapFlags", Layout {size: size_of::<GstVideoFrameMapFlags>(), alignment: align_of::<GstVideoFrameMapFlags>()}),
    ("GstVideoGLTextureOrientation", Layout {size: size_of::<GstVideoGLTextureOrientation>(), alignment: align_of::<GstVideoGLTextureOrientation>()}),
    ("GstVideoGLTextureType", Layout {size: size_of::<GstVideoGLTextureType>(), alignment: align_of::<GstVideoGLTextureType>()}),
    ("GstVideoGLTextureUploadMeta", Layout {size: size_of::<GstVideoGLTextureUploadMeta>(), alignment: align_of::<GstVideoGLTextureUploadMeta>()}),
    ("GstVideoGammaMode", Layout {size: size_of::<GstVideoGammaMode>(), alignment: align_of::<GstVideoGammaMode>()}),
    ("GstVideoInfo", Layout {size: size_of::<GstVideoInfo>(), alignment: align_of::<GstVideoInfo>()}),
    ("GstVideoInterlaceMode", Layout {size: size_of::<GstVideoInterlaceMode>(), alignment: align_of::<GstVideoInterlaceMode>()}),
    ("GstVideoMatrixMode", Layout {size: size_of::<GstVideoMatrixMode>(), alignment: align_of::<GstVideoMatrixMode>()}),
    ("GstVideoMeta", Layout {size: size_of::<GstVideoMeta>(), alignment: align_of::<GstVideoMeta>()}),
    ("GstVideoMetaTransform", Layout {size: size_of::<GstVideoMetaTransform>(), alignment: align_of::<GstVideoMetaTransform>()}),
    ("GstVideoMultiviewFlags", Layout {size: size_of::<GstVideoMultiviewFlags>(), alignment: align_of::<GstVideoMultiviewFlags>()}),
    ("GstVideoMultiviewFramePacking", Layout {size: size_of::<GstVideoMultiviewFramePacking>(), alignment: align_of::<GstVideoMultiviewFramePacking>()}),
    ("GstVideoMultiviewMode", Layout {size: size_of::<GstVideoMultiviewMode>(), alignment: align_of::<GstVideoMultiviewMode>()}),
    ("GstVideoOrientationInterface", Layout {size: size_of::<GstVideoOrientationInterface>(), alignment: align_of::<GstVideoOrientationInterface>()}),
    ("GstVideoOrientationMethod", Layout {size: size_of::<GstVideoOrientationMethod>(), alignment: align_of::<GstVideoOrientationMethod>()}),
    ("GstVideoOverlayCompositionMeta", Layout {size: size_of::<GstVideoOverlayCompositionMeta>(), alignment: align_of::<GstVideoOverlayCompositionMeta>()}),
    ("GstVideoOverlayFormatFlags", Layout {size: size_of::<GstVideoOverlayFormatFlags>(), alignment: align_of::<GstVideoOverlayFormatFlags>()}),
    ("GstVideoOverlayInterface", Layout {size: size_of::<GstVideoOverlayInterface>(), alignment: align_of::<GstVideoOverlayInterface>()}),
    ("GstVideoPackFlags", Layout {size: size_of::<GstVideoPackFlags>(), alignment: align_of::<GstVideoPackFlags>()}),
    ("GstVideoPrimariesMode", Layout {size: size_of::<GstVideoPrimariesMode>(), alignment: align_of::<GstVideoPrimariesMode>()}),
    ("GstVideoRectangle", Layout {size: size_of::<GstVideoRectangle>(), alignment: align_of::<GstVideoRectangle>()}),
    ("GstVideoRegionOfInterestMeta", Layout {size: size_of::<GstVideoRegionOfInterestMeta>(), alignment: align_of::<GstVideoRegionOfInterestMeta>()}),
    ("GstVideoResampler", Layout {size: size_of::<GstVideoResampler>(), alignment: align_of::<GstVideoResampler>()}),
    ("GstVideoResamplerFlags", Layout {size: size_of::<GstVideoResamplerFlags>(), alignment: align_of::<GstVideoResamplerFlags>()}),
    ("GstVideoResamplerMethod", Layout {size: size_of::<GstVideoResamplerMethod>(), alignment: align_of::<GstVideoResamplerMethod>()}),
    ("GstVideoScalerFlags", Layout {size: size_of::<GstVideoScalerFlags>(), alignment: align_of::<GstVideoScalerFlags>()}),
    ("GstVideoSink", Layout {size: size_of::<GstVideoSink>(), alignment: align_of::<GstVideoSink>()}),
    ("GstVideoSinkClass", Layout {size: size_of::<GstVideoSinkClass>(), alignment: align_of::<GstVideoSinkClass>()}),
    ("GstVideoTileMode", Layout {size: size_of::<GstVideoTileMode>(), alignment: align_of::<GstVideoTileMode>()}),
    ("GstVideoTileType", Layout {size: size_of::<GstVideoTileType>(), alignment: align_of::<GstVideoTileType>()}),
    ("GstVideoTimeCode", Layout {size: size_of::<GstVideoTimeCode>(), alignment: align_of::<GstVideoTimeCode>()}),
    ("GstVideoTimeCodeConfig", Layout {size: size_of::<GstVideoTimeCodeConfig>(), alignment: align_of::<GstVideoTimeCodeConfig>()}),
    ("GstVideoTimeCodeFlags", Layout {size: size_of::<GstVideoTimeCodeFlags>(), alignment: align_of::<GstVideoTimeCodeFlags>()}),
    ("GstVideoTimeCodeInterval", Layout {size: size_of::<GstVideoTimeCodeInterval>(), alignment: align_of::<GstVideoTimeCodeInterval>()}),
    ("GstVideoTimeCodeMeta", Layout {size: size_of::<GstVideoTimeCodeMeta>(), alignment: align_of::<GstVideoTimeCodeMeta>()}),
    ("GstVideoTransferFunction", Layout {size: size_of::<GstVideoTransferFunction>(), alignment: align_of::<GstVideoTransferFunction>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("GST_BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META", "GstBufferPoolOptionVideoAffineTransformation"),
    ("GST_BUFFER_POOL_OPTION_VIDEO_ALIGNMENT", "GstBufferPoolOptionVideoAlignment"),
    ("GST_BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META", "GstBufferPoolOptionVideoGLTextureUploadMeta"),
    ("GST_BUFFER_POOL_OPTION_VIDEO_META", "GstBufferPoolOptionVideoMeta"),
    ("GST_CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META", "meta:GstVideoAffineTransformation"),
    ("GST_CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META", "meta:GstVideoGLTextureUploadMeta"),
    ("GST_CAPS_FEATURE_META_GST_VIDEO_META", "meta:GstVideoMeta"),
    ("GST_CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION", "meta:GstVideoOverlayComposition"),
    ("GST_COLOR_BALANCE_HARDWARE", "0"),
    ("GST_COLOR_BALANCE_SOFTWARE", "1"),
    ("GST_META_TAG_VIDEO_COLORSPACE_STR", "colorspace"),
    ("GST_META_TAG_VIDEO_ORIENTATION_STR", "orientation"),
    ("GST_META_TAG_VIDEO_SIZE_STR", "size"),
    ("GST_META_TAG_VIDEO_STR", "video"),
    ("GST_NAVIGATION_COMMAND_ACTIVATE", "24"),
    ("GST_NAVIGATION_COMMAND_DOWN", "23"),
    ("GST_NAVIGATION_COMMAND_INVALID", "0"),
    ("GST_NAVIGATION_COMMAND_LEFT", "20"),
    ("GST_NAVIGATION_COMMAND_MENU1", "1"),
    ("GST_NAVIGATION_COMMAND_MENU2", "2"),
    ("GST_NAVIGATION_COMMAND_MENU3", "3"),
    ("GST_NAVIGATION_COMMAND_MENU4", "4"),
    ("GST_NAVIGATION_COMMAND_MENU5", "5"),
    ("GST_NAVIGATION_COMMAND_MENU6", "6"),
    ("GST_NAVIGATION_COMMAND_MENU7", "7"),
    ("GST_NAVIGATION_COMMAND_NEXT_ANGLE", "31"),
    ("GST_NAVIGATION_COMMAND_PREV_ANGLE", "30"),
    ("GST_NAVIGATION_COMMAND_RIGHT", "21"),
    ("GST_NAVIGATION_COMMAND_UP", "22"),
    ("GST_NAVIGATION_EVENT_COMMAND", "6"),
    ("GST_NAVIGATION_EVENT_INVALID", "0"),
    ("GST_NAVIGATION_EVENT_KEY_PRESS", "1"),
    ("GST_NAVIGATION_EVENT_KEY_RELEASE", "2"),
    ("GST_NAVIGATION_EVENT_MOUSE_BUTTON_PRESS", "3"),
    ("GST_NAVIGATION_EVENT_MOUSE_BUTTON_RELEASE", "4"),
    ("GST_NAVIGATION_EVENT_MOUSE_MOVE", "5"),
    ("GST_NAVIGATION_MESSAGE_ANGLES_CHANGED", "3"),
    ("GST_NAVIGATION_MESSAGE_COMMANDS_CHANGED", "2"),
    ("GST_NAVIGATION_MESSAGE_EVENT", "4"),
    ("GST_NAVIGATION_MESSAGE_INVALID", "0"),
    ("GST_NAVIGATION_MESSAGE_MOUSE_OVER", "1"),
    ("GST_NAVIGATION_QUERY_ANGLES", "2"),
    ("GST_NAVIGATION_QUERY_COMMANDS", "1"),
    ("GST_NAVIGATION_QUERY_INVALID", "0"),
    ("GST_VIDEO_ALPHA_MODE_COPY", "0"),
    ("GST_VIDEO_ALPHA_MODE_MULT", "2"),
    ("GST_VIDEO_ALPHA_MODE_SET", "1"),
    ("GST_VIDEO_BUFFER_FLAG_FIRST_IN_BUNDLE", "33554432"),
    ("GST_VIDEO_BUFFER_FLAG_INTERLACED", "1048576"),
    ("GST_VIDEO_BUFFER_FLAG_LAST", "268435456"),
    ("GST_VIDEO_BUFFER_FLAG_MULTIPLE_VIEW", "16777216"),
    ("GST_VIDEO_BUFFER_FLAG_ONEFIELD", "8388608"),
    ("GST_VIDEO_BUFFER_FLAG_RFF", "4194304"),
    ("GST_VIDEO_BUFFER_FLAG_TFF", "2097152"),
    ("GST_VIDEO_CHROMA_FLAG_INTERLACED", "1"),
    ("GST_VIDEO_CHROMA_FLAG_NONE", "0"),
    ("GST_VIDEO_CHROMA_METHOD_LINEAR", "1"),
    ("GST_VIDEO_CHROMA_METHOD_NEAREST", "0"),
    ("GST_VIDEO_CHROMA_MODE_DOWNSAMPLE_ONLY", "2"),
    ("GST_VIDEO_CHROMA_MODE_FULL", "0"),
    ("GST_VIDEO_CHROMA_MODE_NONE", "3"),
    ("GST_VIDEO_CHROMA_MODE_UPSAMPLE_ONLY", "1"),
    ("GST_VIDEO_CHROMA_SITE_ALT_LINE", "8"),
    ("GST_VIDEO_CHROMA_SITE_COSITED", "6"),
    ("GST_VIDEO_CHROMA_SITE_DV", "14"),
    ("GST_VIDEO_CHROMA_SITE_H_COSITED", "2"),
    ("GST_VIDEO_CHROMA_SITE_JPEG", "1"),
    ("GST_VIDEO_CHROMA_SITE_MPEG2", "2"),
    ("GST_VIDEO_CHROMA_SITE_NONE", "1"),
    ("GST_VIDEO_CHROMA_SITE_UNKNOWN", "0"),
    ("GST_VIDEO_CHROMA_SITE_V_COSITED", "4"),
    ("GST_VIDEO_CODEC_FRAME_FLAG_DECODE_ONLY", "1"),
    ("GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME", "4"),
    ("GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME_HEADERS", "8"),
    ("GST_VIDEO_CODEC_FRAME_FLAG_SYNC_POINT", "2"),
    ("GST_VIDEO_COLORIMETRY_BT2020", "bt2020"),
    ("GST_VIDEO_COLORIMETRY_BT601", "bt601"),
    ("GST_VIDEO_COLORIMETRY_BT709", "bt709"),
    ("GST_VIDEO_COLORIMETRY_SMPTE240M", "smpte240m"),
    ("GST_VIDEO_COLORIMETRY_SRGB", "sRGB"),
    ("GST_VIDEO_COLOR_MATRIX_BT2020", "6"),
    ("GST_VIDEO_COLOR_MATRIX_BT601", "4"),
    ("GST_VIDEO_COLOR_MATRIX_BT709", "3"),
    ("GST_VIDEO_COLOR_MATRIX_FCC", "2"),
    ("GST_VIDEO_COLOR_MATRIX_RGB", "1"),
    ("GST_VIDEO_COLOR_MATRIX_SMPTE240M", "5"),
    ("GST_VIDEO_COLOR_MATRIX_UNKNOWN", "0"),
    ("GST_VIDEO_COLOR_PRIMARIES_ADOBERGB", "8"),
    ("GST_VIDEO_COLOR_PRIMARIES_BT2020", "7"),
    ("GST_VIDEO_COLOR_PRIMARIES_BT470BG", "3"),
    ("GST_VIDEO_COLOR_PRIMARIES_BT470M", "2"),
    ("GST_VIDEO_COLOR_PRIMARIES_BT709", "1"),
    ("GST_VIDEO_COLOR_PRIMARIES_FILM", "6"),
    ("GST_VIDEO_COLOR_PRIMARIES_SMPTE170M", "4"),
    ("GST_VIDEO_COLOR_PRIMARIES_SMPTE240M", "5"),
    ("GST_VIDEO_COLOR_PRIMARIES_UNKNOWN", "0"),
    ("GST_VIDEO_COLOR_RANGE_0_255", "1"),
    ("GST_VIDEO_COLOR_RANGE_16_235", "2"),
    ("GST_VIDEO_COLOR_RANGE_UNKNOWN", "0"),
    ("GST_VIDEO_COMP_A", "3"),
    ("GST_VIDEO_COMP_B", "2"),
    ("GST_VIDEO_COMP_G", "1"),
    ("GST_VIDEO_COMP_INDEX", "0"),
    ("GST_VIDEO_COMP_PALETTE", "1"),
    ("GST_VIDEO_COMP_R", "0"),
    ("GST_VIDEO_COMP_U", "1"),
    ("GST_VIDEO_COMP_V", "2"),
    ("GST_VIDEO_COMP_Y", "0"),
    ("GST_VIDEO_CONVERTER_OPT_ALPHA_MODE", "GstVideoConverter.alpha-mode"),
    ("GST_VIDEO_CONVERTER_OPT_ALPHA_VALUE", "GstVideoConverter.alpha-value"),
    ("GST_VIDEO_CONVERTER_OPT_BORDER_ARGB", "GstVideoConverter.border-argb"),
    ("GST_VIDEO_CONVERTER_OPT_CHROMA_MODE", "GstVideoConverter.chroma-mode"),
    ("GST_VIDEO_CONVERTER_OPT_CHROMA_RESAMPLER_METHOD", "GstVideoConverter.chroma-resampler-method"),
    ("GST_VIDEO_CONVERTER_OPT_DEST_HEIGHT", "GstVideoConverter.dest-height"),
    ("GST_VIDEO_CONVERTER_OPT_DEST_WIDTH", "GstVideoConverter.dest-width"),
    ("GST_VIDEO_CONVERTER_OPT_DEST_X", "GstVideoConverter.dest-x"),
    ("GST_VIDEO_CONVERTER_OPT_DEST_Y", "GstVideoConverter.dest-y"),
    ("GST_VIDEO_CONVERTER_OPT_DITHER_METHOD", "GstVideoConverter.dither-method"),
    ("GST_VIDEO_CONVERTER_OPT_DITHER_QUANTIZATION", "GstVideoConverter.dither-quantization"),
    ("GST_VIDEO_CONVERTER_OPT_FILL_BORDER", "GstVideoConverter.fill-border"),
    ("GST_VIDEO_CONVERTER_OPT_GAMMA_MODE", "GstVideoConverter.gamma-mode"),
    ("GST_VIDEO_CONVERTER_OPT_MATRIX_MODE", "GstVideoConverter.matrix-mode"),
    ("GST_VIDEO_CONVERTER_OPT_PRIMARIES_MODE", "GstVideoConverter.primaries-mode"),
    ("GST_VIDEO_CONVERTER_OPT_RESAMPLER_METHOD", "GstVideoConverter.resampler-method"),
    ("GST_VIDEO_CONVERTER_OPT_RESAMPLER_TAPS", "GstVideoConverter.resampler-taps"),
    ("GST_VIDEO_CONVERTER_OPT_SRC_HEIGHT", "GstVideoConverter.src-height"),
    ("GST_VIDEO_CONVERTER_OPT_SRC_WIDTH", "GstVideoConverter.src-width"),
    ("GST_VIDEO_CONVERTER_OPT_SRC_X", "GstVideoConverter.src-x"),
    ("GST_VIDEO_CONVERTER_OPT_SRC_Y", "GstVideoConverter.src-y"),
    ("GST_VIDEO_CONVERTER_OPT_THREADS", "GstVideoConverter.threads"),
    ("GST_VIDEO_DECODER_MAX_ERRORS", "10"),
    ("GST_VIDEO_DECODER_SINK_NAME", "sink"),
    ("GST_VIDEO_DECODER_SRC_NAME", "src"),
    ("GST_VIDEO_DITHER_BAYER", "4"),
    ("GST_VIDEO_DITHER_FLAG_INTERLACED", "1"),
    ("GST_VIDEO_DITHER_FLAG_NONE", "0"),
    ("GST_VIDEO_DITHER_FLAG_QUANTIZE", "2"),
    ("GST_VIDEO_DITHER_FLOYD_STEINBERG", "2"),
    ("GST_VIDEO_DITHER_NONE", "0"),
    ("GST_VIDEO_DITHER_SIERRA_LITE", "3"),
    ("GST_VIDEO_DITHER_VERTERR", "1"),
    ("GST_VIDEO_ENCODER_SINK_NAME", "sink"),
    ("GST_VIDEO_ENCODER_SRC_NAME", "src"),
    ("GST_VIDEO_FIELD_ORDER_BOTTOM_FIELD_FIRST", "2"),
    ("GST_VIDEO_FIELD_ORDER_TOP_FIELD_FIRST", "1"),
    ("GST_VIDEO_FIELD_ORDER_UNKNOWN", "0"),
    ("GST_VIDEO_FLAG_NONE", "0"),
    ("GST_VIDEO_FLAG_PREMULTIPLIED_ALPHA", "2"),
    ("GST_VIDEO_FLAG_VARIABLE_FPS", "1"),
    ("GST_VIDEO_FORMATS_ALL", "{ I420, YV12, YUY2, UYVY, AYUV, RGBx, BGRx, xRGB, xBGR, RGBA, BGRA, ARGB, ABGR, RGB, BGR, Y41B, Y42B, YVYU, Y444, v210, v216, NV12, NV21, GRAY8, GRAY16_BE, GRAY16_LE, v308, RGB16, BGR16, RGB15, BGR15, UYVP, A420, RGB8P, YUV9, YVU9, IYU1, ARGB64, AYUV64, r210, I420_10BE, I420_10LE, I422_10BE, I422_10LE, Y444_10BE, Y444_10LE, GBR, GBR_10BE, GBR_10LE, NV16, NV24, NV12_64Z32, A420_10BE, A420_10LE, A422_10BE, A422_10LE, A444_10BE, A444_10LE, NV61, P010_10BE, P010_10LE, IYU2, VYUY, GBRA, GBRA_10BE, GBRA_10LE, GBR_12BE, GBR_12LE, GBRA_12BE, GBRA_12LE, I420_12BE, I420_12LE, I422_12BE, I422_12LE, Y444_12BE, Y444_12LE, GRAY10_LE32, NV12_10LE32, NV16_10LE32 }"),
    ("GST_VIDEO_FORMAT_A420", "34"),
    ("GST_VIDEO_FORMAT_A420_10BE", "54"),
    ("GST_VIDEO_FORMAT_A420_10LE", "55"),
    ("GST_VIDEO_FORMAT_A422_10BE", "56"),
    ("GST_VIDEO_FORMAT_A422_10LE", "57"),
    ("GST_VIDEO_FORMAT_A444_10BE", "58"),
    ("GST_VIDEO_FORMAT_A444_10LE", "59"),
    ("GST_VIDEO_FORMAT_ABGR", "14"),
    ("GST_VIDEO_FORMAT_ARGB", "13"),
    ("GST_VIDEO_FORMAT_ARGB64", "39"),
    ("GST_VIDEO_FORMAT_AYUV", "6"),
    ("GST_VIDEO_FORMAT_AYUV64", "40"),
    ("GST_VIDEO_FORMAT_BGR", "16"),
    ("GST_VIDEO_FORMAT_BGR15", "32"),
    ("GST_VIDEO_FORMAT_BGR16", "30"),
    ("GST_VIDEO_FORMAT_BGRA", "12"),
    ("GST_VIDEO_FORMAT_BGRx", "8"),
    ("GST_VIDEO_FORMAT_ENCODED", "1"),
    ("GST_VIDEO_FORMAT_FLAG_ALPHA", "8"),
    ("GST_VIDEO_FORMAT_FLAG_COMPLEX", "64"),
    ("GST_VIDEO_FORMAT_FLAG_GRAY", "4"),
    ("GST_VIDEO_FORMAT_FLAG_LE", "16"),
    ("GST_VIDEO_FORMAT_FLAG_PALETTE", "32"),
    ("GST_VIDEO_FORMAT_FLAG_RGB", "2"),
    ("GST_VIDEO_FORMAT_FLAG_TILED", "256"),
    ("GST_VIDEO_FORMAT_FLAG_UNPACK", "128"),
    ("GST_VIDEO_FORMAT_FLAG_YUV", "1"),
    ("GST_VIDEO_FORMAT_GBR", "48"),
    ("GST_VIDEO_FORMAT_GBRA", "65"),
    ("GST_VIDEO_FORMAT_GBRA_10BE", "66"),
    ("GST_VIDEO_FORMAT_GBRA_10LE", "67"),
    ("GST_VIDEO_FORMAT_GBRA_12BE", "70"),
    ("GST_VIDEO_FORMAT_GBRA_12LE", "71"),
    ("GST_VIDEO_FORMAT_GBR_10BE", "49"),
    ("GST_VIDEO_FORMAT_GBR_10LE", "50"),
    ("GST_VIDEO_FORMAT_GBR_12BE", "68"),
    ("GST_VIDEO_FORMAT_GBR_12LE", "69"),
    ("GST_VIDEO_FORMAT_GRAY10_LE32", "78"),
    ("GST_VIDEO_FORMAT_GRAY16_BE", "26"),
    ("GST_VIDEO_FORMAT_GRAY16_LE", "27"),
    ("GST_VIDEO_FORMAT_GRAY8", "25"),
    ("GST_VIDEO_FORMAT_I420", "2"),
    ("GST_VIDEO_FORMAT_I420_10BE", "42"),
    ("GST_VIDEO_FORMAT_I420_10LE", "43"),
    ("GST_VIDEO_FORMAT_I420_12BE", "72"),
    ("GST_VIDEO_FORMAT_I420_12LE", "73"),
    ("GST_VIDEO_FORMAT_I422_10BE", "44"),
    ("GST_VIDEO_FORMAT_I422_10LE", "45"),
    ("GST_VIDEO_FORMAT_I422_12BE", "74"),
    ("GST_VIDEO_FORMAT_I422_12LE", "75"),
    ("GST_VIDEO_FORMAT_IYU1", "38"),
    ("GST_VIDEO_FORMAT_IYU2", "63"),
    ("GST_VIDEO_FORMAT_NV12", "23"),
    ("GST_VIDEO_FORMAT_NV12_10LE32", "79"),
    ("GST_VIDEO_FORMAT_NV12_64Z32", "53"),
    ("GST_VIDEO_FORMAT_NV16", "51"),
    ("GST_VIDEO_FORMAT_NV16_10LE32", "80"),
    ("GST_VIDEO_FORMAT_NV21", "24"),
    ("GST_VIDEO_FORMAT_NV24", "52"),
    ("GST_VIDEO_FORMAT_NV61", "60"),
    ("GST_VIDEO_FORMAT_P010_10BE", "61"),
    ("GST_VIDEO_FORMAT_P010_10LE", "62"),
    ("GST_VIDEO_FORMAT_RGB", "15"),
    ("GST_VIDEO_FORMAT_RGB15", "31"),
    ("GST_VIDEO_FORMAT_RGB16", "29"),
    ("GST_VIDEO_FORMAT_RGB8P", "35"),
    ("GST_VIDEO_FORMAT_RGBA", "11"),
    ("GST_VIDEO_FORMAT_RGBx", "7"),
    ("GST_VIDEO_FORMAT_UNKNOWN", "0"),
    ("GST_VIDEO_FORMAT_UYVP", "33"),
    ("GST_VIDEO_FORMAT_UYVY", "5"),
    ("GST_VIDEO_FORMAT_VYUY", "64"),
    ("GST_VIDEO_FORMAT_Y41B", "17"),
    ("GST_VIDEO_FORMAT_Y42B", "18"),
    ("GST_VIDEO_FORMAT_Y444", "20"),
    ("GST_VIDEO_FORMAT_Y444_10BE", "46"),
    ("GST_VIDEO_FORMAT_Y444_10LE", "47"),
    ("GST_VIDEO_FORMAT_Y444_12BE", "76"),
    ("GST_VIDEO_FORMAT_Y444_12LE", "77"),
    ("GST_VIDEO_FORMAT_YUV9", "36"),
    ("GST_VIDEO_FORMAT_YUY2", "4"),
    ("GST_VIDEO_FORMAT_YV12", "3"),
    ("GST_VIDEO_FORMAT_YVU9", "37"),
    ("GST_VIDEO_FORMAT_YVYU", "19"),
    ("GST_VIDEO_FORMAT_r210", "41"),
    ("GST_VIDEO_FORMAT_v210", "21"),
    ("GST_VIDEO_FORMAT_v216", "22"),
    ("GST_VIDEO_FORMAT_v308", "28"),
    ("GST_VIDEO_FORMAT_xBGR", "10"),
    ("GST_VIDEO_FORMAT_xRGB", "9"),
    ("GST_VIDEO_FPS_RANGE", "(fraction) [ 0, max ]"),
    ("GST_VIDEO_FRAME_FLAG_FIRST_IN_BUNDLE", "32"),
    ("GST_VIDEO_FRAME_FLAG_INTERLACED", "1"),
    ("GST_VIDEO_FRAME_FLAG_MULTIPLE_VIEW", "16"),
    ("GST_VIDEO_FRAME_FLAG_NONE", "0"),
    ("GST_VIDEO_FRAME_FLAG_ONEFIELD", "8"),
    ("GST_VIDEO_FRAME_FLAG_RFF", "4"),
    ("GST_VIDEO_FRAME_FLAG_TFF", "2"),
    ("GST_VIDEO_FRAME_MAP_FLAG_LAST", "16777216"),
    ("GST_VIDEO_FRAME_MAP_FLAG_NO_REF", "65536"),
    ("GST_VIDEO_GAMMA_MODE_NONE", "0"),
    ("GST_VIDEO_GAMMA_MODE_REMAP", "1"),
    ("GST_VIDEO_GL_TEXTURE_ORIENTATION_X_FLIP_Y_FLIP", "3"),
    ("GST_VIDEO_GL_TEXTURE_ORIENTATION_X_FLIP_Y_NORMAL", "2"),
    ("GST_VIDEO_GL_TEXTURE_ORIENTATION_X_NORMAL_Y_FLIP", "1"),
    ("GST_VIDEO_GL_TEXTURE_ORIENTATION_X_NORMAL_Y_NORMAL", "0"),
    ("GST_VIDEO_GL_TEXTURE_TYPE_LUMINANCE", "0"),
    ("GST_VIDEO_GL_TEXTURE_TYPE_LUMINANCE_ALPHA", "1"),
    ("GST_VIDEO_GL_TEXTURE_TYPE_R", "5"),
    ("GST_VIDEO_GL_TEXTURE_TYPE_RG", "6"),
    ("GST_VIDEO_GL_TEXTURE_TYPE_RGB", "3"),
    ("GST_VIDEO_GL_TEXTURE_TYPE_RGB16", "2"),
    ("GST_VIDEO_GL_TEXTURE_TYPE_RGBA", "4"),
    ("GST_VIDEO_INTERLACE_MODE_FIELDS", "3"),
    ("GST_VIDEO_INTERLACE_MODE_INTERLEAVED", "1"),
    ("GST_VIDEO_INTERLACE_MODE_MIXED", "2"),
    ("GST_VIDEO_INTERLACE_MODE_PROGRESSIVE", "0"),
    ("GST_VIDEO_MATRIX_MODE_FULL", "0"),
    ("GST_VIDEO_MATRIX_MODE_INPUT_ONLY", "1"),
    ("GST_VIDEO_MATRIX_MODE_NONE", "3"),
    ("GST_VIDEO_MATRIX_MODE_OUTPUT_ONLY", "2"),
    ("GST_VIDEO_MAX_COMPONENTS", "4"),
    ("GST_VIDEO_MAX_PLANES", "4"),
    ("GST_VIDEO_MULTIVIEW_FLAGS_HALF_ASPECT", "16384"),
    ("GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLIPPED", "2"),
    ("GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLOPPED", "4"),
    ("GST_VIDEO_MULTIVIEW_FLAGS_MIXED_MONO", "32768"),
    ("GST_VIDEO_MULTIVIEW_FLAGS_NONE", "0"),
    ("GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLIPPED", "8"),
    ("GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLOPPED", "16"),
    ("GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_VIEW_FIRST", "1"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_CHECKERBOARD", "8"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_COLUMN_INTERLEAVED", "5"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_LEFT", "1"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_MONO", "0"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_NONE", "-1"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_RIGHT", "2"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_ROW_INTERLEAVED", "6"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_SIDE_BY_SIDE", "3"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_SIDE_BY_SIDE_QUINCUNX", "4"),
    ("GST_VIDEO_MULTIVIEW_FRAME_PACKING_TOP_BOTTOM", "7"),
    ("GST_VIDEO_MULTIVIEW_MODE_CHECKERBOARD", "8"),
    ("GST_VIDEO_MULTIVIEW_MODE_COLUMN_INTERLEAVED", "5"),
    ("GST_VIDEO_MULTIVIEW_MODE_FRAME_BY_FRAME", "32"),
    ("GST_VIDEO_MULTIVIEW_MODE_LEFT", "1"),
    ("GST_VIDEO_MULTIVIEW_MODE_MONO", "0"),
    ("GST_VIDEO_MULTIVIEW_MODE_MULTIVIEW_FRAME_BY_FRAME", "33"),
    ("GST_VIDEO_MULTIVIEW_MODE_NONE", "-1"),
    ("GST_VIDEO_MULTIVIEW_MODE_RIGHT", "2"),
    ("GST_VIDEO_MULTIVIEW_MODE_ROW_INTERLEAVED", "6"),
    ("GST_VIDEO_MULTIVIEW_MODE_SEPARATED", "34"),
    ("GST_VIDEO_MULTIVIEW_MODE_SIDE_BY_SIDE", "3"),
    ("GST_VIDEO_MULTIVIEW_MODE_SIDE_BY_SIDE_QUINCUNX", "4"),
    ("GST_VIDEO_MULTIVIEW_MODE_TOP_BOTTOM", "7"),
    ("GST_VIDEO_ORIENTATION_180", "2"),
    ("GST_VIDEO_ORIENTATION_90L", "3"),
    ("GST_VIDEO_ORIENTATION_90R", "1"),
    ("GST_VIDEO_ORIENTATION_AUTO", "8"),
    ("GST_VIDEO_ORIENTATION_CUSTOM", "9"),
    ("GST_VIDEO_ORIENTATION_HORIZ", "4"),
    ("GST_VIDEO_ORIENTATION_IDENTITY", "0"),
    ("GST_VIDEO_ORIENTATION_UL_LR", "6"),
    ("GST_VIDEO_ORIENTATION_UR_LL", "7"),
    ("GST_VIDEO_ORIENTATION_VERT", "5"),
    ("GST_VIDEO_OVERLAY_COMPOSITION_BLEND_FORMATS", "{ BGRx, RGBx, xRGB, xBGR, RGBA, BGRA, ARGB, ABGR, RGB, BGR, I420, YV12, AYUV, YUY2, UYVY, v308, Y41B, Y42B, Y444, NV12, NV21, A420, YUV9, YVU9, IYU1, GRAY8 }"),
    ("GST_VIDEO_OVERLAY_FORMAT_FLAG_GLOBAL_ALPHA", "2"),
    ("GST_VIDEO_OVERLAY_FORMAT_FLAG_NONE", "0"),
    ("GST_VIDEO_OVERLAY_FORMAT_FLAG_PREMULTIPLIED_ALPHA", "1"),
    ("GST_VIDEO_PACK_FLAG_INTERLACED", "2"),
    ("GST_VIDEO_PACK_FLAG_NONE", "0"),
    ("GST_VIDEO_PACK_FLAG_TRUNCATE_RANGE", "1"),
    ("GST_VIDEO_PRIMARIES_MODE_FAST", "2"),
    ("GST_VIDEO_PRIMARIES_MODE_MERGE_ONLY", "1"),
    ("GST_VIDEO_PRIMARIES_MODE_NONE", "0"),
    ("GST_VIDEO_RESAMPLER_FLAG_HALF_TAPS", "1"),
    ("GST_VIDEO_RESAMPLER_FLAG_NONE", "0"),
    ("GST_VIDEO_RESAMPLER_METHOD_CUBIC", "2"),
    ("GST_VIDEO_RESAMPLER_METHOD_LANCZOS", "4"),
    ("GST_VIDEO_RESAMPLER_METHOD_LINEAR", "1"),
    ("GST_VIDEO_RESAMPLER_METHOD_NEAREST", "0"),
    ("GST_VIDEO_RESAMPLER_METHOD_SINC", "3"),
    ("GST_VIDEO_RESAMPLER_OPT_CUBIC_B", "GstVideoResampler.cubic-b"),
    ("GST_VIDEO_RESAMPLER_OPT_CUBIC_C", "GstVideoResampler.cubic-c"),
    ("GST_VIDEO_RESAMPLER_OPT_ENVELOPE", "GstVideoResampler.envelope"),
    ("GST_VIDEO_RESAMPLER_OPT_MAX_TAPS", "GstVideoResampler.max-taps"),
    ("GST_VIDEO_RESAMPLER_OPT_SHARPEN", "GstVideoResampler.sharpen"),
    ("GST_VIDEO_RESAMPLER_OPT_SHARPNESS", "GstVideoResampler.sharpness"),
    ("GST_VIDEO_SCALER_FLAG_INTERLACED", "1"),
    ("GST_VIDEO_SCALER_FLAG_NONE", "0"),
    ("GST_VIDEO_SCALER_OPT_DITHER_METHOD", "GstVideoScaler.dither-method"),
    ("GST_VIDEO_SIZE_RANGE", "(int) [ 1, max ]"),
    ("GST_VIDEO_TILE_MODE_UNKNOWN", "0"),
    ("GST_VIDEO_TILE_MODE_ZFLIPZ_2X2", "65536"),
    ("GST_VIDEO_TILE_TYPE_INDEXED", "0"),
    ("GST_VIDEO_TILE_TYPE_MASK", "0"),
    ("GST_VIDEO_TILE_TYPE_SHIFT", "16"),
    ("GST_VIDEO_TILE_X_TILES_MASK", "0"),
    ("GST_VIDEO_TILE_Y_TILES_SHIFT", "16"),
    ("GST_VIDEO_TIME_CODE_FLAGS_DROP_FRAME", "1"),
    ("GST_VIDEO_TIME_CODE_FLAGS_INTERLACED", "2"),
    ("GST_VIDEO_TIME_CODE_FLAGS_NONE", "0"),
    ("GST_VIDEO_TRANSFER_ADOBERGB", "12"),
    ("GST_VIDEO_TRANSFER_BT2020_12", "11"),
    ("GST_VIDEO_TRANSFER_BT709", "5"),
    ("GST_VIDEO_TRANSFER_GAMMA10", "1"),
    ("GST_VIDEO_TRANSFER_GAMMA18", "2"),
    ("GST_VIDEO_TRANSFER_GAMMA20", "3"),
    ("GST_VIDEO_TRANSFER_GAMMA22", "4"),
    ("GST_VIDEO_TRANSFER_GAMMA28", "8"),
    ("GST_VIDEO_TRANSFER_LOG100", "9"),
    ("GST_VIDEO_TRANSFER_LOG316", "10"),
    ("GST_VIDEO_TRANSFER_SMPTE240M", "6"),
    ("GST_VIDEO_TRANSFER_SRGB", "7"),
    ("GST_VIDEO_TRANSFER_UNKNOWN", "0"),
];


