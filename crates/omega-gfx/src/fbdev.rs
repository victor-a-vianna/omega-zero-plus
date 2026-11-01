use std::ffi::{c_char, c_void, CString};
use std::fs::File;
use std::os::fd::AsRawFd;

use libloading::Library;

use crate::surface::{GfxError, Surface};

const FALLBACK_PANEL: (u32, u32) = (720, 480);

const FB0: &str = "/sys/class/graphics/fb0";

const EGL_SUCCESS: i32 = 0x3000;
const EGL_NONE: i32 = 0x3038;
const EGL_ALPHA_SIZE: i32 = 0x3021;
const EGL_BLUE_SIZE: i32 = 0x3022;
const EGL_GREEN_SIZE: i32 = 0x3023;
const EGL_RED_SIZE: i32 = 0x3024;
const EGL_SURFACE_TYPE: i32 = 0x3033;
const EGL_HEIGHT: i32 = 0x3056;
const EGL_WIDTH: i32 = 0x3057;
const EGL_RENDERABLE_TYPE: i32 = 0x3040;
const EGL_CONTEXT_CLIENT_VERSION: i32 = 0x3098;
const EGL_OPENGL_ES_API: u32 = 0x30A0;
const EGL_WINDOW_BIT: i32 = 0x0004;
const EGL_OPENGL_ES2_BIT: i32 = 0x0004;
const EGL_PLATFORM_GBM_KHR: u32 = 0x31d7;
const GBM_FORMAT_XRGB8888: u32 = 0x3432_5258;
const GBM_BO_USE_SCANOUT: u32 = 1;
const GBM_BO_USE_RENDERING: u32 = 4;

type Ptr = *mut c_void;
type GetDisplay = unsafe extern "C" fn(Ptr) -> Ptr;
type Initialize = unsafe extern "C" fn(Ptr, *mut i32, *mut i32) -> u32;
type BindApi = unsafe extern "C" fn(u32) -> u32;
type ChooseConfig = unsafe extern "C" fn(Ptr, *const i32, *mut Ptr, i32, *mut i32) -> u32;
type CreateWindowSurface = unsafe extern "C" fn(Ptr, Ptr, Ptr, *const i32) -> Ptr;
type CreateContext = unsafe extern "C" fn(Ptr, Ptr, Ptr, *const i32) -> Ptr;
type MakeCurrent = unsafe extern "C" fn(Ptr, Ptr, Ptr, Ptr) -> u32;
type QuerySurface = unsafe extern "C" fn(Ptr, Ptr, i32, *mut i32) -> u32;
type SwapBuffers = unsafe extern "C" fn(Ptr, Ptr) -> u32;
type SwapInterval = unsafe extern "C" fn(Ptr, i32) -> u32;
type GetProcAddress = unsafe extern "C" fn(*const c_char) -> *const c_void;
type GetError = unsafe extern "C" fn() -> i32;
type Terminate = unsafe extern "C" fn(Ptr) -> u32;
type GetPlatformDisplayExt = unsafe extern "C" fn(u32, Ptr, *const i32) -> Ptr;
type GbmCreateDevice = unsafe extern "C" fn(i32) -> Ptr;
type GbmDeviceDestroy = unsafe extern "C" fn(Ptr);
type GbmSurfaceCreate = unsafe extern "C" fn(Ptr, u32, u32, u32, u32) -> Ptr;
type GbmSurfaceDestroy = unsafe extern "C" fn(Ptr);

struct Egl {
    get_display: GetDisplay,
    initialize: Initialize,
    bind_api: BindApi,
    choose_config: ChooseConfig,
    create_window_surface: CreateWindowSurface,
    create_context: CreateContext,
    make_current: MakeCurrent,
    query_surface: QuerySurface,
    swap_buffers: SwapBuffers,
    swap_interval: SwapInterval,
    get_proc_address: GetProcAddress,
    get_error: GetError,
    terminate: Terminate,
    _lib: Library,
}

#[repr(C)]
struct FbdevWindow {
    width: u16,
    height: u16,
}

pub struct FbdevSurface {
    egl: Egl,
    gles: Library,
    display: Ptr,
    surface: Ptr,
    context: Ptr,
    size: (u32, u32),
    _window: Box<FbdevWindow>,
}

pub fn panel_mode(text: &str) -> Option<(u32, u32)> {
    let body = text.lines().next()?.rsplit(':').next()?;
    let (w, rest) = body.split_once('x')?;
    let h: String = rest.chars().take_while(char::is_ascii_digit).collect();
    let (w, h) = (w.trim().parse().ok()?, h.parse().ok()?);
    (w > 0 && h > 0).then_some((w, h))
}

pub fn panel_size(text: &str) -> Option<(u32, u32)> {
    let (w, h) = text.trim().split_once(',')?;
    let (w, h) = (w.trim().parse().ok()?, h.trim().parse().ok()?);
    (w > 0 && h > 0).then_some((w, h))
}

pub fn egl_error(what: &str, code: i32) -> GfxError {
    match code {
        EGL_SUCCESS => GfxError::Context(format!("{what}: refused, egl flagged nothing")),
        _ => GfxError::Context(format!("{what}: egl error 0x{code:x}")),
    }
}

fn open(name: &str) -> Result<Library, GfxError> {
    unsafe { Library::new(name) }.map_err(|e| GfxError::Context(format!("{name}: {e}")))
}

unsafe fn sym<T: Copy>(lib: &Library, name: &str) -> Result<T, GfxError> {
    lib.get::<T>(name.as_bytes())
        .map(|s| *s)
        .map_err(|e| GfxError::Context(format!("{name}: {e}")))
}

impl Egl {
    fn load() -> Result<Self, GfxError> {
        let lib = open("libEGL.so.1").or_else(|_| open("libEGL.so"))?;
        unsafe {
            Ok(Egl {
                get_display: sym(&lib, "eglGetDisplay")?,
                initialize: sym(&lib, "eglInitialize")?,
                bind_api: sym(&lib, "eglBindAPI")?,
                choose_config: sym(&lib, "eglChooseConfig")?,
                create_window_surface: sym(&lib, "eglCreateWindowSurface")?,
                create_context: sym(&lib, "eglCreateContext")?,
                make_current: sym(&lib, "eglMakeCurrent")?,
                query_surface: sym(&lib, "eglQuerySurface")?,
                swap_buffers: sym(&lib, "eglSwapBuffers")?,
                swap_interval: sym(&lib, "eglSwapInterval")?,
                get_proc_address: sym(&lib, "eglGetProcAddress")?,
                get_error: sym(&lib, "eglGetError")?,
                terminate: sym(&lib, "eglTerminate")?,
                _lib: lib,
            })
        }
    }

    fn fail(&self, what: &str) -> GfxError {
        egl_error(what, unsafe { (self.get_error)() })
    }
}

impl FbdevSurface {
    pub fn new() -> Result<Self, GfxError> {
        let attr = |name: &str| std::fs::read_to_string(format!("{FB0}/{name}")).ok();
        let hint = attr("mode")
            .as_deref()
            .and_then(panel_mode)
            .or_else(|| attr("modes").as_deref().and_then(panel_mode))
            .or_else(|| attr("virtual_size").as_deref().and_then(panel_size))
            .unwrap_or(FALLBACK_PANEL);
        FbdevSurface::open(hint)
    }

    fn open(hint: (u32, u32)) -> Result<Self, GfxError> {
        let egl = Egl::load()?;
        let gles = open("libGLESv2.so.2").or_else(|_| open("libGLESv2.so"))?;
        unsafe {
            let display = (egl.get_display)(std::ptr::null_mut());
            if display.is_null() {
                return Err(egl.fail("eglGetDisplay"));
            }
            if (egl.initialize)(display, std::ptr::null_mut(), std::ptr::null_mut()) == 0 {
                return Err(egl.fail("eglInitialize"));
            }
            if (egl.bind_api)(EGL_OPENGL_ES_API) == 0 {
                return Err(egl.fail("eglBindAPI"));
            }
            let attrs = [
                EGL_SURFACE_TYPE,
                EGL_WINDOW_BIT,
                EGL_RED_SIZE,
                8,
                EGL_GREEN_SIZE,
                8,
                EGL_BLUE_SIZE,
                8,
                EGL_ALPHA_SIZE,
                0,
                EGL_RENDERABLE_TYPE,
                EGL_OPENGL_ES2_BIT,
                EGL_NONE,
            ];
            let mut config: Ptr = std::ptr::null_mut();
            let mut found = 0;
            if (egl.choose_config)(display, attrs.as_ptr(), &mut config, 1, &mut found) == 0 {
                return Err(egl.fail("eglChooseConfig"));
            }
            if found == 0 {
                return Err(GfxError::Context(
                    "eglChooseConfig: no es2 window config with 8/8/8 colour".into(),
                ));
            }
            let mut window = Box::new(FbdevWindow {
                width: hint.0 as u16,
                height: hint.1 as u16,
            });
            let native = &mut *window as *mut FbdevWindow as Ptr;
            let mut surface =
                (egl.create_window_surface)(display, config, native, std::ptr::null());
            if surface.is_null() {
                surface = (egl.create_window_surface)(
                    display,
                    config,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                );
            }
            if surface.is_null() {
                return Err(egl.fail("eglCreateWindowSurface"));
            }
            let context_attrs = [EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE];
            let context = (egl.create_context)(
                display,
                config,
                std::ptr::null_mut(),
                context_attrs.as_ptr(),
            );
            if context.is_null() {
                return Err(egl.fail("eglCreateContext"));
            }
            if (egl.make_current)(display, surface, surface, context) == 0 {
                return Err(egl.fail("eglMakeCurrent"));
            }
            (egl.swap_interval)(display, 1);
            let size = query_size(&egl, display, surface).unwrap_or(hint);
            Ok(FbdevSurface {
                egl,
                gles,
                display,
                surface,
                context,
                size,
                _window: window,
            })
        }
    }
}

/// A KMS/GBM surface for DRM devices.  It is deliberately opt-in: BaseOS H700
/// devices keep their fbdev path, while RK3566 can use Mesa's Panfrost driver.
pub struct DrmSurface {
    egl: Egl,
    gles: Library,
    _gbm: Library,
    _card: File,
    device: Ptr,
    gbm_surface: Ptr,
    display: Ptr,
    surface: Ptr,
    context: Ptr,
    size: (u32, u32),
    gbm_device_destroy: GbmDeviceDestroy,
    gbm_surface_destroy: GbmSurfaceDestroy,
}

impl DrmSurface {
    /// Open the KMS card.  `OMEGA_DRM_CARD` is useful for development; BaseOS
    /// RK3566 normally exposes its display controller as card0.
    pub fn new() -> Result<Self, GfxError> {
        let card = std::env::var("OMEGA_DRM_CARD").unwrap_or_else(|_| "/dev/dri/card0".into());
        let card = File::options()
            .read(true)
            .write(true)
            .open(&card)
            .map_err(|e| GfxError::Context(format!("open KMS card: {e}")))?;
        let egl = Egl::load()?;
        let gles = open("libGLESv2.so.2").or_else(|_| open("libGLESv2.so"))?;
        let gbm = open("libgbm.so.1").or_else(|_| open("libgbm.so"))?;
        unsafe {
            let create_device: GbmCreateDevice = sym(&gbm, "gbm_create_device")?;
            let destroy_device: GbmDeviceDestroy = sym(&gbm, "gbm_device_destroy")?;
            let create_surface: GbmSurfaceCreate = sym(&gbm, "gbm_surface_create")?;
            let destroy_surface: GbmSurfaceDestroy = sym(&gbm, "gbm_surface_destroy")?;
            let device = create_device(card.as_raw_fd());
            if device.is_null() {
                return Err(GfxError::Context("gbm_create_device failed".into()));
            }
            let gbm_surface = create_surface(
                device,
                720,
                720,
                GBM_FORMAT_XRGB8888,
                GBM_BO_USE_SCANOUT | GBM_BO_USE_RENDERING,
            );
            if gbm_surface.is_null() {
                destroy_device(device);
                return Err(GfxError::Context("gbm_surface_create failed".into()));
            }
            let get_platform: GetPlatformDisplayExt = sym(&egl._lib, "eglGetPlatformDisplayEXT")
                .or_else(|_| {
                    let name = CString::new("eglGetPlatformDisplayEXT").expect("static name");
                    let ptr = (egl.get_proc_address)(name.as_ptr());
                    if ptr.is_null() {
                        Err(GfxError::Context(
                            "eglGetPlatformDisplayEXT unavailable".into(),
                        ))
                    } else {
                        Ok(std::mem::transmute(ptr))
                    }
                })?;
            let display = get_platform(EGL_PLATFORM_GBM_KHR, device, std::ptr::null());
            if display.is_null()
                || (egl.initialize)(display, std::ptr::null_mut(), std::ptr::null_mut()) == 0
            {
                destroy_surface(gbm_surface);
                destroy_device(device);
                return Err(egl.fail("eglInitialize GBM"));
            }
            if (egl.bind_api)(EGL_OPENGL_ES_API) == 0 {
                (egl.terminate)(display);
                destroy_surface(gbm_surface);
                destroy_device(device);
                return Err(egl.fail("eglBindAPI"));
            }
            let attrs = [
                EGL_SURFACE_TYPE,
                EGL_WINDOW_BIT,
                EGL_RED_SIZE,
                8,
                EGL_GREEN_SIZE,
                8,
                EGL_BLUE_SIZE,
                8,
                EGL_RENDERABLE_TYPE,
                EGL_OPENGL_ES2_BIT,
                EGL_NONE,
            ];
            let mut config: Ptr = std::ptr::null_mut();
            let mut found = 0;
            if (egl.choose_config)(display, attrs.as_ptr(), &mut config, 1, &mut found) == 0
                || found == 0
            {
                (egl.terminate)(display);
                destroy_surface(gbm_surface);
                destroy_device(device);
                return Err(egl.fail("eglChooseConfig GBM"));
            }
            let surface =
                (egl.create_window_surface)(display, config, gbm_surface, std::ptr::null());
            if surface.is_null() {
                (egl.terminate)(display);
                destroy_surface(gbm_surface);
                destroy_device(device);
                return Err(egl.fail("eglCreateWindowSurface GBM"));
            }
            let context_attrs = [EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE];
            let context = (egl.create_context)(
                display,
                config,
                std::ptr::null_mut(),
                context_attrs.as_ptr(),
            );
            if context.is_null() || (egl.make_current)(display, surface, surface, context) == 0 {
                (egl.terminate)(display);
                destroy_surface(gbm_surface);
                destroy_device(device);
                return Err(egl.fail("eglMakeCurrent GBM"));
            }
            (egl.swap_interval)(display, 1);
            let size = query_size(&egl, display, surface).unwrap_or((720, 720));
            Ok(Self {
                egl,
                gles,
                _gbm: gbm,
                _card: card,
                device,
                gbm_surface,
                display,
                surface,
                context,
                size,
                gbm_device_destroy: destroy_device,
                gbm_surface_destroy: destroy_surface,
            })
        }
    }
}

fn query_size(egl: &Egl, display: Ptr, surface: Ptr) -> Option<(u32, u32)> {
    let (mut w, mut h) = (0, 0);
    unsafe {
        if (egl.query_surface)(display, surface, EGL_WIDTH, &mut w) == 0
            || (egl.query_surface)(display, surface, EGL_HEIGHT, &mut h) == 0
        {
            return None;
        }
    }
    (w > 0 && h > 0).then_some((w as u32, h as u32))
}

impl Surface for FbdevSurface {
    fn make_current(&mut self) -> Result<(), GfxError> {
        let ok = unsafe {
            (self.egl.make_current)(self.display, self.surface, self.surface, self.context)
        };
        match ok {
            0 => Err(self.egl.fail("eglMakeCurrent")),
            _ => Ok(()),
        }
    }

    fn window_size(&self) -> (u32, u32) {
        self.size
    }

    fn swap(&mut self) -> Result<(), GfxError> {
        match unsafe { (self.egl.swap_buffers)(self.display, self.surface) } {
            0 => Err(self.egl.fail("eglSwapBuffers")),
            _ => Ok(()),
        }
    }

    fn proc_address(&self, name: &str) -> *const c_void {
        let Ok(c) = CString::new(name) else {
            return std::ptr::null();
        };
        let exported = unsafe { self.gles.get::<unsafe extern "C" fn()>(name.as_bytes()) };
        match exported {
            Ok(f) => *f as *const c_void,
            Err(_) => unsafe { (self.egl.get_proc_address)(c.as_ptr()) },
        }
    }
}

impl Surface for DrmSurface {
    fn make_current(&mut self) -> Result<(), GfxError> {
        if unsafe {
            (self.egl.make_current)(self.display, self.surface, self.surface, self.context)
        } == 0
        {
            Err(self.egl.fail("eglMakeCurrent GBM"))
        } else {
            Ok(())
        }
    }

    fn window_size(&self) -> (u32, u32) {
        self.size
    }

    fn swap(&mut self) -> Result<(), GfxError> {
        if unsafe { (self.egl.swap_buffers)(self.display, self.surface) } == 0 {
            Err(self.egl.fail("eglSwapBuffers GBM"))
        } else {
            Ok(())
        }
    }

    fn proc_address(&self, name: &str) -> *const c_void {
        let Ok(c) = CString::new(name) else {
            return std::ptr::null();
        };
        let exported = unsafe { self.gles.get::<unsafe extern "C" fn()>(name.as_bytes()) };
        match exported {
            Ok(f) => *f as *const c_void,
            Err(_) => unsafe { (self.egl.get_proc_address)(c.as_ptr()) },
        }
    }
}

impl Drop for FbdevSurface {
    fn drop(&mut self) {
        unsafe {
            (self.egl.make_current)(
                self.display,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            (self.egl.terminate)(self.display);
        }
    }
}

impl Drop for DrmSurface {
    fn drop(&mut self) {
        unsafe {
            (self.egl.make_current)(
                self.display,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            (self.egl.terminate)(self.display);
            (self.gbm_surface_destroy)(self.gbm_surface);
            (self.gbm_device_destroy)(self.device);
        }
    }
}
