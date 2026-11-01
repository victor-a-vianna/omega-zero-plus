use std::ffi::{c_char, c_int, c_uint, c_void};
use std::ptr;

use libloading::Library;

use crate::core::CoreError;

type EglDisplay = *mut c_void;
type EglConfig = *mut c_void;
type EglSurface = *mut c_void;
type EglContext = *mut c_void;
type EglBoolean = c_uint;

const EGL_NONE: c_int = 0x3038;
const EGL_SURFACE_TYPE: c_int = 0x3033;
const EGL_PBUFFER_BIT: c_int = 0x0001;
const EGL_RENDERABLE_TYPE: c_int = 0x3040;
const EGL_OPENGL_ES2_BIT: c_int = 0x0004;
const EGL_RED_SIZE: c_int = 0x3024;
const EGL_GREEN_SIZE: c_int = 0x3023;
const EGL_BLUE_SIZE: c_int = 0x3022;
const EGL_ALPHA_SIZE: c_int = 0x3021;
const EGL_WIDTH: c_int = 0x3057;
const EGL_HEIGHT: c_int = 0x3056;
const EGL_CONTEXT_CLIENT_VERSION: c_int = 0x3098;
const EGL_OPENGL_ES_API: c_uint = 0x30a0;
const GL_RGBA: c_uint = 0x1908;
const GL_UNSIGNED_BYTE: c_uint = 0x1401;

type EglGetDisplay = unsafe extern "C" fn(*mut c_void) -> EglDisplay;
type EglInitialize = unsafe extern "C" fn(EglDisplay, *mut c_int, *mut c_int) -> EglBoolean;
type EglBindApi = unsafe extern "C" fn(c_uint) -> EglBoolean;
type EglChooseConfig =
    unsafe extern "C" fn(EglDisplay, *const c_int, *mut EglConfig, c_int, *mut c_int) -> EglBoolean;
type EglCreatePbufferSurface =
    unsafe extern "C" fn(EglDisplay, EglConfig, *const c_int) -> EglSurface;
type EglCreateContext =
    unsafe extern "C" fn(EglDisplay, EglConfig, EglContext, *const c_int) -> EglContext;
type EglMakeCurrent =
    unsafe extern "C" fn(EglDisplay, EglSurface, EglSurface, EglContext) -> EglBoolean;
type EglDestroySurface = unsafe extern "C" fn(EglDisplay, EglSurface) -> EglBoolean;
type EglDestroyContext = unsafe extern "C" fn(EglDisplay, EglContext) -> EglBoolean;
type EglTerminate = unsafe extern "C" fn(EglDisplay) -> EglBoolean;
type EglGetProcAddress = unsafe extern "C" fn(*const c_char) -> *mut c_void;
type GlReadPixels = unsafe extern "C" fn(c_int, c_int, c_int, c_int, c_uint, c_uint, *mut c_void);

pub struct HwContext {
    _egl: Library,
    gles: Library,
    display: EglDisplay,
    surface: EglSurface,
    context: EglContext,
    make_current: EglMakeCurrent,
    destroy_surface: EglDestroySurface,
    destroy_context: EglDestroyContext,
    terminate: EglTerminate,
    get_proc_address: EglGetProcAddress,
    read_pixels: GlReadPixels,
}

// The core is moved to its emulation thread before a context is created and all EGL calls then
// remain on that thread. These opaque handles are never dereferenced by Rust.
unsafe impl Send for HwContext {}

impl HwContext {
    pub unsafe fn new(width: u32, height: u32) -> Result<Self, CoreError> {
        let egl = open_any(&["libEGL.so.1", "libEGL.so", "libmali.so"])?;
        let gles = open_any(&["libGLESv2.so.2", "libGLESv2.so", "libmali.so"])?;
        let get_display: EglGetDisplay = symbol(&egl, b"eglGetDisplay\0")?;
        let initialize: EglInitialize = symbol(&egl, b"eglInitialize\0")?;
        let bind_api: EglBindApi = symbol(&egl, b"eglBindAPI\0")?;
        let choose_config: EglChooseConfig = symbol(&egl, b"eglChooseConfig\0")?;
        let create_surface: EglCreatePbufferSurface = symbol(&egl, b"eglCreatePbufferSurface\0")?;
        let create_context: EglCreateContext = symbol(&egl, b"eglCreateContext\0")?;
        let make_current: EglMakeCurrent = symbol(&egl, b"eglMakeCurrent\0")?;
        let destroy_surface: EglDestroySurface = symbol(&egl, b"eglDestroySurface\0")?;
        let destroy_context: EglDestroyContext = symbol(&egl, b"eglDestroyContext\0")?;
        let terminate: EglTerminate = symbol(&egl, b"eglTerminate\0")?;
        let get_proc_address: EglGetProcAddress = symbol(&egl, b"eglGetProcAddress\0")?;

        let display = get_display(ptr::null_mut());
        if display.is_null() || initialize(display, ptr::null_mut(), ptr::null_mut()) == 0 {
            return Err(CoreError::Load("EGL display unavailable".into()));
        }
        if bind_api(EGL_OPENGL_ES_API) == 0 {
            terminate(display);
            return Err(CoreError::Load("EGL cannot bind OpenGL ES".into()));
        }
        let config_attrs = [
            EGL_SURFACE_TYPE,
            EGL_PBUFFER_BIT,
            EGL_RENDERABLE_TYPE,
            EGL_OPENGL_ES2_BIT,
            EGL_RED_SIZE,
            8,
            EGL_GREEN_SIZE,
            8,
            EGL_BLUE_SIZE,
            8,
            EGL_ALPHA_SIZE,
            8,
            EGL_NONE,
        ];
        let mut config = ptr::null_mut();
        let mut count = 0;
        if choose_config(display, config_attrs.as_ptr(), &mut config, 1, &mut count) == 0
            || count == 0
        {
            terminate(display);
            return Err(CoreError::Load(
                "EGL has no GLES2 pbuffer configuration".into(),
            ));
        }
        let surface_attrs = [
            EGL_WIDTH,
            width as c_int,
            EGL_HEIGHT,
            height as c_int,
            EGL_NONE,
        ];
        let surface = create_surface(display, config, surface_attrs.as_ptr());
        let context_attrs = [EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE];
        let context = create_context(display, config, ptr::null_mut(), context_attrs.as_ptr());
        if surface.is_null()
            || context.is_null()
            || make_current(display, surface, surface, context) == 0
        {
            if !surface.is_null() {
                destroy_surface(display, surface);
            }
            if !context.is_null() {
                destroy_context(display, context);
            }
            terminate(display);
            return Err(CoreError::Load(
                "EGL could not create a GLES2 context".into(),
            ));
        }
        let read_pixels: GlReadPixels = symbol(&gles, b"glReadPixels\0")?;
        Ok(Self {
            _egl: egl,
            gles,
            display,
            surface,
            context,
            make_current,
            destroy_surface,
            destroy_context,
            terminate,
            get_proc_address,
            read_pixels,
        })
    }

    pub unsafe fn make_current(&self) -> bool {
        (self.make_current)(self.display, self.surface, self.surface, self.context) != 0
    }

    pub unsafe fn proc_address(&self, name: *const c_char) -> *mut c_void {
        let p = (self.get_proc_address)(name);
        if !p.is_null() {
            return p;
        }
        if name.is_null() {
            return ptr::null_mut();
        }
        self.gles
            .get::<*mut c_void>(std::ffi::CStr::from_ptr(name).to_bytes_with_nul())
            .map(|p| *p)
            .unwrap_or(ptr::null_mut())
    }

    pub unsafe fn frame(&self, width: u32, height: u32, bottom_left: bool) -> Vec<u8> {
        let mut rgba = vec![0; width as usize * height as usize * 4];
        (self.read_pixels)(
            0,
            0,
            width as c_int,
            height as c_int,
            GL_RGBA,
            GL_UNSIGNED_BYTE,
            rgba.as_mut_ptr() as *mut c_void,
        );
        let mut bgra = vec![0; rgba.len()];
        let stride = width as usize * 4;
        for y in 0..height as usize {
            let source_y = if bottom_left {
                height as usize - 1 - y
            } else {
                y
            };
            for x in 0..width as usize {
                let src = source_y * stride + x * 4;
                let dst = y * stride + x * 4;
                bgra[dst] = rgba[src + 2];
                bgra[dst + 1] = rgba[src + 1];
                bgra[dst + 2] = rgba[src];
                bgra[dst + 3] = 0;
            }
        }
        bgra
    }
}

impl Drop for HwContext {
    fn drop(&mut self) {
        unsafe {
            let _ = (self.make_current)(
                self.display,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            let _ = (self.destroy_context)(self.display, self.context);
            let _ = (self.destroy_surface)(self.display, self.surface);
            let _ = (self.terminate)(self.display);
        }
    }
}

unsafe fn open_any(names: &[&str]) -> Result<Library, CoreError> {
    names
        .iter()
        .find_map(|name| Library::new(name).ok())
        .ok_or_else(|| CoreError::Load(format!("none of {} is available", names.join(", "))))
}

unsafe fn symbol<T: Copy>(lib: &Library, name: &[u8]) -> Result<T, CoreError> {
    lib.get::<T>(name)
        .map(|s| *s)
        .map_err(|e| CoreError::Load(e.to_string()))
}
