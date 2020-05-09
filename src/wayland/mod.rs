use std::{
    str,
    ffi::{CStr, CString},
    ptr::{NonNull, null, null_mut},
    os::raw::{c_char, c_int, c_uint, c_void, c_long},
    mem::transmute,
    pin::Pin,
};

/* * Dynamic Loading Into Separate Namespace * */

#[repr(transparent)]
struct DlObj(c_void);

fn dlopen(name: &str) -> Option<NonNull<DlObj>> {
    let name = CStr::from_bytes_with_nul(name.as_bytes())
        .expect("Missing Null Byte!");
    extern "C" {
        fn dlmopen(ns: c_long, name: *const c_char, flags: c_int) -> *mut DlObj;
    }
    unsafe {
        NonNull::new(dlmopen(-1 /*NEW*/, name.as_ptr(), 0x00002 /*NOW*/))
    }
}

fn dlsym(handle: NonNull<DlObj>, name: &str) -> Option<NonNull<c_void>> {
    let name = CStr::from_bytes_with_nul(name.as_bytes())
        .expect("Missing Null Byte!");
    extern "C" {
        fn dlsym(dlobj: *mut DlObj, symbol: *const c_char) -> *mut c_void;
    }
    unsafe {
        NonNull::new(dlsym(handle.as_ptr(), name.as_ptr()))
    }
}

/* * From wayland-client-core.h  * */

#[repr(transparent)]
struct WlProxy(c_void);

#[repr(transparent)]
struct WlDisplay(c_void);

/* * From wayland-util.h  * */

#[repr(C)]
struct WlArray {
    size: usize,
    alloc: usize,
    data: *mut c_void,
}

#[repr(C)]
struct WlMessage {
    // Message name
    name: *const c_char,
    // Message signature
    signature: *const c_char,
    // Object argument interfaces
    wl_interface: *const *const WlInterface,
}

#[repr(C)]
struct WlInterface {
    // Interface name
    name: *const c_char,
    // Interface version
    version: c_int,
    // Number of methods (requests)
    method_count: c_int,
    // Method (request) signatures
    methods: *const WlMessage,
    // Number of events
    event_count: c_int,
    // Event signatures
    events: *const WlMessage,
}

/* * From wayland-client-protocol.h  * */

#[repr(transparent)]
struct WlSurface(c_void);
#[repr(transparent)]
struct WlRegistry(c_void);
#[repr(transparent)]
struct WlCompositor(c_void);
#[repr(transparent)]
struct WlShellSurface(c_void);
#[repr(transparent)]
struct WlShell(c_void);
#[repr(transparent)]
struct WlSeat(c_void);
#[repr(transparent)]
struct WlCallback(c_void);
#[repr(transparent)]
struct WlOutput(c_void);
#[repr(transparent)]
struct WlKeyboard(c_void);
#[repr(transparent)]
struct WlPointer(c_void);
#[repr(transparent)]
struct WlTouch(c_void);

#[repr(C)]
#[derive(Copy, Clone)]
enum WlSeatCapability {
    Pointer = 1,
    Keyboard = 2,
    Touch = 4,
}

#[repr(C)]
struct WlRegistryListener {
    global: extern fn(data: *mut c_void, wl_registry: *mut WlRegistry, name: u32, interface: *const c_char, version: u32) -> (),
    global_remove: extern fn(data: *mut c_void, wl_registry: *mut WlRegistry, name: u32),
}

#[repr(C)]
struct WlCallbackListener {
    done: extern fn(data: *mut c_void, callback: *mut WlCallback, callback_data: u32) -> (),
}

#[repr(C)]
struct WlOutputListener {
    geometry: extern fn(data: *mut c_void, output: *mut WlOutput, x: i32, y: i32, physical_width: i32, physical_height: i32, subpixel: i32, make: *const c_char, model: *const c_char, transform: i32) -> (),
    mode: extern fn(data: *mut c_void, output: *mut WlOutput, flags: u32, width: i32, height: i32, refresh: i32) -> (),
    done: extern fn(data: *mut c_void, output: *mut WlOutput) -> (),
    scale: extern fn(data: *mut c_void, output: *mut WlOutput, factor: i32) -> (),
}

#[repr(C)]
struct WlSeatListener {
    capabilities: extern fn(data: *mut c_void, seat: *mut WlSeat, capabilites: u32) -> (),
    name: extern fn(data: *mut c_void, seat: *mut WlSeat, name: *const c_char) -> (),
}

#[repr(C)]
struct WlKeyboardListener {
    // Keyboard mapping description.
    keymap: extern fn(data: *mut c_void, keyboard: *mut WlKeyboard, format: u32, fd: i32, size: u32) -> (),
    // Keyboard Focus Entered.
    enter: extern fn(data: *mut c_void, keyboard: *mut WlKeyboard, serial: u32, surface: *mut WlSurface, keys: *mut WlArray) -> (),
    // Keyboard Focus Exited.
    leave: extern fn(data: *mut c_void, keyboard: *mut WlKeyboard, serial: u32, surface: *mut WlSurface) -> (),
    // Key press or release.
    key: extern fn(data: *mut c_void, keyboard: *mut WlKeyboard, serial: u32, time: u32, key: u32, state: u32) -> (),
    // Modifier / Group state changed.
    modifiers: extern fn(data: *mut c_void, keyboard: *mut WlKeyboard, serial: u32, mods_depressed: u32, mods_latched: u32, mods_locked: u32, group: u32) -> (),
    // Repeat rate & delay settings changed.
    repeat_info: extern fn(data: *mut c_void, keyboard: *mut WlKeyboard, rate: i32, delay: i32) -> (),
}

#[repr(C)]
struct WlPointerListener {
    // Pointer focus enter
    enter: extern fn(data: *mut c_void, pointer: *mut WlPointer, serial: u32, surface: *mut WlSurface, surface_x: i32, surface_y: i32) -> (),
    // Pointer focus leave
    leave: extern fn(data: *mut c_void, pointer: *mut WlPointer, serial: u32, surface: *mut WlSurface) -> (),
    // Pointer motion
    motion: extern fn(data: *mut c_void, pointer: *mut WlPointer, time: u32, surface_x: i32, surface_y: i32) -> (),
    // Pointer button
    button: extern fn(data: *mut c_void, pointer: *mut WlPointer, serial: u32, time: u32, button: u32, state: u32) -> (),
    // Axis Event
    axis: extern fn(data: *mut c_void, pointer: *mut WlPointer, time: u32, axis: u32, value: i32) -> (),
    // Pointer Frame Complete (Now process events).
    frame: extern fn(data: *mut c_void, pointer: *mut WlPointer) -> (),
    // What type of device sent axis event?
    axis_source: extern fn(data: *mut c_void, pointer: *mut WlPointer, axis_source: u32) -> (),
    // Stop axis event
    axis_stop: extern fn(data: *mut c_void, pointer: *mut WlPointer, time: u32, axis: u32) -> (),
    // Discrete step axis
    axis_discrete: extern fn(data: *mut c_void, pointer: *mut WlPointer, axis: u32, discrete: i32) -> (),
}

#[repr(C)]
struct WlTouchListener {
    // Touch down event at beginning of touch sequence.
    down: extern fn(data: *mut c_void, touch: *mut WlTouch, serial: u32, time: u32, surface: *mut WlSurface, id: i32, x: i32, y: i32) -> (),
    // End of a touch event sequence.
    up: extern fn(data: *mut c_void, touch: *mut WlTouch, serial: u32, time: u32, id: i32) -> (),
    // Update of touch point coordinates.
    motion: extern fn(data: *mut c_void, touch: *mut WlTouch, time: u32, id: i32, x: i32, y: i32) -> (),
    // End of touch frame event.
    frame: extern fn(data: *mut c_void, touch: *mut WlTouch) -> (),
    // Global gesture, don't process touch stream anymore.
    cancel: extern fn(data: *mut c_void, touch: *mut WlTouch) -> (),
    // Touch event changed shape (ellipse).
    shape: extern fn(data: *mut c_void, touch: *mut WlTouch, id: i32, major: i32, minor: i32) -> (),
    // Update orientation of touch point
    orientation: extern fn(data: *mut c_void, touch: *mut WlTouch, id: i32, orientation: i32) -> (),
}

/* * From wayland-cursor.h  * */

#[repr(C)]
pub(crate) struct WlCursorImage {
    // Actual width
    width: u32,
    // Actual height
    height: u32,
    // Hot spot x (must be inside image)
    hotspot_x: u32,
    // Hot spot y (must be inside image)
    hotspot_y: u32,
    // Animation delay to next frame (ms)
    delay: u32,
}

#[repr(C)]
struct WlCursor {
    image_count: c_uint,
    images: *mut *mut WlCursorImage,
    name: *mut c_char,
}

/* * From zxdg v6 * */

#[repr(transparent)]
struct ZxdgSurface(c_void);
#[repr(transparent)]
struct ZxdgToplevel(c_void);
#[repr(transparent)]
struct ZxdgShell(c_void);

#[repr(C)]
struct ZxdgSurfaceListener {
    configure: extern fn(data: *mut c_void, surface: *mut ZxdgSurface, serial: u32) -> (),
}

#[repr(C)]
struct ZxdgToplevelListener {
    configure: extern fn(data: *mut c_void, toplevel: *mut ZxdgToplevel, width: i32, height: i32, states: *mut WlArray) -> (),
    close: extern fn(data: *mut c_void, toplevel: *mut ZxdgToplevel) -> (),
}

#[repr(C)]
struct ZxdgShellListener {
    ping: extern fn(data: *mut c_void, xdg_shell: *mut ZxdgShell, serial: u32) -> (),
}

/* From include/wayland-egl-core.h */

#[repr(transparent)]
struct WlEglWindow(c_void);

/* From include/wayland-cursor.h */

#[repr(transparent)]
struct WlCursorTheme(c_void);
#[repr(transparent)]
struct WlBuffer(c_void);
#[repr(transparent)]
struct WlShm(c_void);

/* ************************************************************************** */

// Wrapper around Wayland Library
struct WaylandClient {
    wl_proxy_marshal: extern fn(p: *mut WlProxy, opcode: u32, ...) -> (),
    wl_proxy_destroy: extern fn(proxy: *mut WlProxy) -> (),
    wl_display_connect: extern fn(name: *const c_char) -> *mut WlDisplay,
    wl_proxy_marshal_constructor: extern fn(proxy: *mut WlProxy, opcode: u32, interface: *const WlInterface, ...) -> *mut WlProxy,
    wl_proxy_add_listener: extern fn(proxy: *mut WlProxy, *const extern fn() -> (), data: *mut c_void) -> c_int,
    wl_display_dispatch: extern fn(display: *mut WlDisplay) -> c_int,
    wl_proxy_marshal_constructor_versioned: extern fn(proxy: *mut WlProxy, opcode: u32, interface: *const WlInterface, version: u32, ...) -> *mut WlProxy,
    // Static globals
    wl_registry_interface: *const WlInterface,
    wl_compositor_interface: *const WlInterface,
    wl_seat_interface: *const WlInterface,
    wl_shm_interface: *const WlInterface,
    wl_pointer_interface: *const WlInterface,
    wl_output_interface: *const WlInterface,
    wl_keyboard_interface: *const WlInterface,
    wl_touch_interface: *const WlInterface,
    wl_callback_interface: *const WlInterface,
    wl_surface_interface: *const WlInterface,
    zxdg_surface_v6_interface: *const WlInterface,
    zxdg_shell_v6_interface: *const WlInterface,
    zxdg_toplevel_v6_interface: *const WlInterface,
}

impl WaylandClient {
    fn new() -> Option<Self> {
        let so = dlopen("libwayland-client.so.0\0")?;

        Some(unsafe { WaylandClient {
            wl_proxy_marshal: transmute(dlsym(so, "wl_proxy_marshal\0")?),
            wl_proxy_destroy: transmute(dlsym(so, "wl_proxy_destroy\0")?),
            wl_display_connect: transmute(dlsym(so, "wl_display_connect\0")?),
            wl_proxy_marshal_constructor: transmute(dlsym(so, "wl_proxy_marshal_constructor\0")?),
            wl_proxy_add_listener: transmute(dlsym(so, "wl_proxy_add_listener\0")?),
            wl_display_dispatch: transmute(dlsym(so, "wl_display_dispatch\0")?),
            wl_proxy_marshal_constructor_versioned: transmute(dlsym(so, "wl_proxy_marshal_constructor_versioned\0")?),
            // Static globals
            wl_registry_interface: dlsym(so, "wl_registry_interface\0")?.cast().as_ptr(),
            wl_compositor_interface: dlsym(so, "wl_compositor_interface\0")?.cast().as_ptr(),
            wl_seat_interface: dlsym(so, "wl_seat_interface\0")?.cast().as_ptr(),
            wl_shm_interface: dlsym(so, "wl_shm_interface\0")?.cast().as_ptr(),
            wl_pointer_interface: dlsym(so, "wl_pointer_interface\0")?.cast().as_ptr(),
            wl_output_interface: dlsym(so, "wl_output_interface\0")?.cast().as_ptr(),
            wl_keyboard_interface: dlsym(so, "wl_keyboard_interface\0")?.cast().as_ptr(),
            wl_touch_interface: dlsym(so, "wl_touch_interface\0")?.cast().as_ptr(),
            wl_callback_interface: dlsym(so, "wl_callback_interface\0")?.cast().as_ptr(),
            wl_surface_interface: dlsym(so, "wl_surface_interface\0")?.cast().as_ptr(),
            zxdg_surface_v6_interface: dlsym(so, "zxdg_surface_v6_interface\0")?.cast().as_ptr(),
            zxdg_shell_v6_interface: dlsym(so, "zxdg_shell_v6_interface\0")?.cast().as_ptr(),
            zxdg_toplevel_v6_interface: dlsym(so, "zxdg_toplevel_v6_interface\0")?.cast().as_ptr(),
        } })
    }

    // Inline Functions From include/wayland-client-protocol.h
    #[inline(always)]
    unsafe fn surface_destroy(&self, surface: *mut WlSurface) {
        (self.wl_proxy_marshal)(surface.cast(), 0 /*WL_SURFACE_DESTROY*/);
        (self.wl_proxy_destroy)(surface.cast());
    }
    #[inline(always)]
    unsafe fn pointer_set_cursor(&self, pointer: *mut WlPointer, cursor_surface: *mut WlSurface, image: *mut WlCursorImage, serial: u32) {
        (self.wl_proxy_marshal)(
            pointer.cast(),
            0, /*WL_POINTER_SET_CURSOR*/
            serial,
            cursor_surface,
            (*image).hotspot_x,
            (*image).hotspot_y,
        );
    }
    #[inline(always)]
    unsafe fn surface_attach(&self, cursor_surface: *mut WlSurface, buffer: *mut WlBuffer) {
        (self.wl_proxy_marshal)(
            cursor_surface.cast(),
            1, /*WL_SURFACE_ATTACH*/
            buffer,
            0,
            0,
        );
    }
    #[inline(always)]
    unsafe fn surface_damage(&self, cursor_surface: *mut WlSurface, image: *mut WlCursorImage) {
        (self.wl_proxy_marshal)(
            cursor_surface.cast(),
            2, /*WL_SURFACE_DAMAGE*/
            0,
            0,
            (*image).width,
            (*image).height
        );
    }
    #[inline(always)]
    unsafe fn surface_commit(&self, cursor_surface: *mut WlSurface) {
        (self.wl_proxy_marshal)(cursor_surface.cast(), 6 /*WL_SURFACE_COMMIT*/);
    }
    #[inline(always)]
    unsafe fn display_get_registry(&self, display: *mut WlDisplay) -> *mut WlRegistry {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal_constructor)(
            display.cast(),
            1 /*WL_DISPLAY_GET_REGISTRY*/,
            self.wl_registry_interface,
            nil,
        ).cast()
    }
    #[inline(always)]
    unsafe fn registry_add_listener(&self, registry: *mut WlRegistry, listener: *const WlRegistryListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(registry.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn compositor_create_surface(&self, compositor: *mut WlCompositor)
        -> *mut WlSurface
    {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal_constructor)(compositor.cast(), 0 /*WL_COMPOSITOR_CREATE_SURFACE*/, self.wl_surface_interface, nil).cast()
    }
    #[inline(always)]
    unsafe fn display_sync(&self, display: *mut WlDisplay) -> *mut WlCallback {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal_constructor)(display.cast(), 0 /*WL_DISPLAY_SYNC*/, self.wl_callback_interface, nil).cast()
    }
    #[inline(always)]
    unsafe fn callback_add_listener(&self, callback: *mut WlCallback, listener: *const WlCallbackListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(callback.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn output_add_listener(&self, output: *mut WlOutput, listener: *const WlOutputListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(output.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn seat_add_listener(&self, seat: *mut WlSeat, listener: *const WlSeatListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(seat.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn pointer_add_listener(&self, pointer: *mut WlPointer, listener: *const WlPointerListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(pointer.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn keyboard_add_listener(&self, keyboard: *mut WlKeyboard, listener: *const WlKeyboardListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(keyboard.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn touch_add_listener(&self, touch: *mut WlTouch, listener: *const WlTouchListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(touch.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn registry_bind(&self, registry: *mut WlRegistry, name: u32, interface: *const WlInterface, version: u32) -> *mut c_void {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal_constructor_versioned)(registry.cast(), 0 /*WL_REGISTRY_BIND*/, interface, version, name, (*interface).name, version, nil).cast()
    }
    #[inline(always)]
    unsafe fn callback_destroy(&self, callback: *mut WlCallback) {
        (self.wl_proxy_destroy)(callback.cast());
    }
    #[inline(always)]
    unsafe fn seat_get_pointer(&self, seat: *mut WlSeat) -> *mut WlPointer {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal_constructor)(seat.cast(), 0 /*WL_SEAT_GET_POINTER*/, self.wl_pointer_interface, nil).cast()
    }
    #[inline(always)]
    unsafe fn seat_get_keyboard(&self, seat: *mut WlSeat) -> *mut WlKeyboard {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal_constructor)(seat.cast(), 1 /*WL_SEAT_GET_KEYBOARD*/, self.wl_keyboard_interface, nil).cast()
    }
    #[inline(always)]
    unsafe fn seat_get_touch(&self, seat: *mut WlSeat) -> *mut WlTouch {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal_constructor)(seat.cast(), 2 /*WL_SEAT_GET_TOUCH*/, self.wl_touch_interface, nil).cast()
    }
    // From include/protocol/xdg-shell-unstable-v6-client-protocol.h
    #[inline(always)]
    unsafe fn zxdg_shell_v6_get_xdg_surface(&self, shell: *mut ZxdgShell, surface: *mut WlSurface) -> *mut ZxdgSurface {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal_constructor)(
            shell.cast(),
            2 /*ZXDG_SHELL_V6_GET_XDG_SURFACE*/,
            self.zxdg_surface_v6_interface,
            nil,
            surface,
        ).cast()
    }
    #[inline(always)]
    unsafe fn zxdg_surface_v6_get_toplevel(&self, surface: *mut ZxdgSurface) -> *mut ZxdgToplevel {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal_constructor)(
            surface.cast(),
            1 /*ZXDG_SURFACE_V6_GET_TOPLEVEL*/,
            self.zxdg_toplevel_v6_interface,
            nil,
        ).cast()
    }
    #[inline(always)]
    unsafe fn zxdg_surface_v6_add_listener(&self, surface: *mut ZxdgSurface, listener: *const ZxdgSurfaceListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(surface.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn zxdg_toplevel_v6_add_listener(&self, toplevel: *mut ZxdgToplevel, listener: *const ZxdgToplevelListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(toplevel.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn zxdg_shell_v6_add_listener(&self, shell: *mut ZxdgShell, listener: *const ZxdgShellListener, data: *mut c_void) -> c_int {
        (self.wl_proxy_add_listener)(shell.cast(), listener.cast(), data)
    }
    #[inline(always)]
    unsafe fn zxdg_toplevel_v6_set_title(&self, toplevel: *mut ZxdgToplevel, title: *const c_char) -> () {
        (self.wl_proxy_marshal)(toplevel.cast(), 2 /*ZXDG_TOPLEVEL_V6_SET_TITLE*/, title);
    }
    #[inline(always)]
    unsafe fn zxdg_toplevel_v6_set_app_id(&self, toplevel: *mut ZxdgToplevel, title: *const c_char) -> () {
        (self.wl_proxy_marshal)(toplevel.cast(), 3 /*ZXDG_TOPLEVEL_V6_SET_APP_ID*/, title);
    }
    #[inline(always)]
    unsafe fn zxdg_toplevel_v6_set_maximized(&self, toplevel: *mut ZxdgToplevel) {
        (self.wl_proxy_marshal)(toplevel.cast(), 9 /*ZXDG_TOPLEVEL_V6_SET_MAXIMIZED*/);
    }
    #[inline(always)]
    unsafe fn zxdg_toplevel_v6_set_fullscreen(&self, toplevel: *mut ZxdgToplevel) {
        let nil: *mut c_void = null_mut();
        (self.wl_proxy_marshal)(toplevel.cast(), 11 /*ZXDG_TOPLEVEL_V6_SET_FULLSCREEN*/, nil);
    }
    #[inline(always)]
    unsafe fn zxdg_toplevel_v6_unset_fullscreen(&self, toplevel: *mut ZxdgToplevel) {
        (self.wl_proxy_marshal)(toplevel.cast(), 12 /*ZXDG_TOPLEVEL_V6_UNSET_FULLSCREEN*/);
    }
    #[inline(always)]
    unsafe fn zxdg_surface_v6_ack_configure(&self, zxdg_surface_v6: *mut ZxdgSurface, serial: u32) {
        (self.wl_proxy_marshal)(zxdg_surface_v6.cast(), 4 /* ZXDG_SURFACE_V6_ACK_CONFIGURE */, serial);
    }
    #[inline(always)]
    unsafe fn zxdg_shell_v6_pong(&self, shell: *mut ZxdgShell, serial: u32) {
        (self.wl_proxy_marshal)(shell.cast(), 3 /*ZXDG_SHELL_V6_PONG*/, serial);
    }

    #[inline(always)]
    unsafe fn connect(&self) -> Option<NonNull<WlDisplay>> {
        NonNull::new((self.wl_display_connect)(null()))
    }
}

struct WaylandEGL {
    wl_egl_window_create: extern fn(
        surface: *mut WlSurface,
        width: c_int,
        height: c_int,
    ) -> *mut WlEglWindow,
    wl_egl_window_resize: extern fn(
        egl_window: *mut WlEglWindow,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    ) -> (),
    wl_egl_window_destroy: extern fn(
        egl_window: *mut WlEglWindow
    ) -> (),
}

impl WaylandEGL {
    fn new() -> Option<Self> {
        let so = dlopen("libwayland-egl.so.1\0")?;
    
        Some(WaylandEGL {
            wl_egl_window_create: transmute(dlsym(so, "wl_egl_window_create\0")?),
            wl_egl_window_resize: transmute(dlsym(so, "wl_egl_window_resize\0")?),
            wl_egl_window_destroy: transmute(dlsym(so, "wl_egl_window_destroy\0")?),
        })
    }
}

struct WaylandCursor {
    wl_cursor_image_get_buffer: fn(image: *mut WlCursorImage) -> *mut WlBuffer,
    wl_cursor_theme_destroy: fn(theme: *mut WlCursorTheme) -> (),
    wl_cursor_theme_load: fn(
        name: *const c_char,
        size: c_int,
        shm: *mut WlShm,
    ) -> *mut WlCursorTheme,
    wl_cursor_theme_get_cursor: fn(
        theme: *mut WlCursorTheme,
        name: *const c_char,
    ) -> *mut WlCursor,
}

impl WaylandCursor {
    fn new() -> Option<Self> {
        let so = dlopen("libwayland-cursor.so.0\0")?;

        Some(WaylandCursor {
            wl_cursor_image_get_buffer: transmute(dlsym(so, "wl_cursor_image_get_buffer\0")?),
            wl_cursor_theme_destroy: transmute(dlsym(so, "wl_cursor_theme_destroy\0")?),
            wl_cursor_theme_load: transmute(dlsym(so, "wl_cursor_theme_load\0")?),
            wl_cursor_theme_get_cursor: transmute(dlsym(so, "wl_cursor_theme_get_cursor\0")?),
        })
    }
}

// Wrapper around Wayland Libraries
pub(super) struct Wayland {
    // Shared Objects
    client: WaylandClient,
    egl: WaylandEGL,
    cursor: WaylandCursor,
    
    // Client
    display: NonNull<WlDisplay>,
    registry: *mut WlRegistry,
    compositor: *mut WlCompositor,
    surface: *mut WlSurface,
    cursor_surface: *mut WlSurface,
    seat: *mut WlSeat,
    pointer: *mut WlPointer,
    keyboard: *mut WlKeyboard,
    touch: *mut WlTouch,
    shell: *mut ZxdgShell,
    shell_surface: *mut ZxdgSurface,
    toplevel: *mut ZxdgToplevel,
    restore_width: c_int,
    restore_height: c_int,
    window_width: c_int,
    window_height: c_int,
    refresh_rate: u64,
    // FIXME: Event based rather than state based.
    running: bool,
    is_restored: bool,
    fullscreen: bool,
    configured: bool,

    
    // EGL
    egl_window: *mut WlEglWindow,
    
    // Cursor
    default_cursor: *mut WlCursor,
    cursor_theme: *mut WlCursorTheme,
    shm: *mut WlShm,
}

impl Wayland {
    pub(super) fn new(name: &str) -> Option<Pin<Box<Self>>> {
        let client = WaylandClient::new()?;
        let egl = WaylandEGL::new()?;
        let cursor = WaylandCursor::new()?;

        // Create window.
        let display = client.connect()?;
        let registry = client.display_get_registry(display.as_ptr());
        let mut wayland = Box::pin(Wayland {
            client, egl, cursor, display, registry, compositor: null_mut(),
            surface: null_mut(), cursor_surface: null_mut(),
            seat: null_mut(),
            pointer: null_mut(),
            keyboard: null_mut(),
            touch: null_mut(),
            shell: null_mut(),
            shell_surface: null_mut(), toplevel: null_mut(),
            restore_width: 640,
            restore_height: 360,
            window_width: 640,
            window_height: 360,
            refresh_rate: 0,
            running: false,
            is_restored: false,
            fullscreen: false,
            configured: false,

            egl_window: null_mut(),
            
            default_cursor: null_mut(),
            cursor_theme: null_mut(),
            shm: null_mut(),
        });
        // Initialization With Callback
        let registry_listener = WlRegistryListener {
            global: registry_global,
            global_remove: registry_global_remove,
        };
        wayland.client.registry_add_listener(registry, &registry_listener,
            ((&mut *wayland) as *mut Self).cast()
        );
        (wayland.client.wl_display_dispatch)(display.as_ptr());
        // Create surfaces
        wayland.surface = wayland.client.compositor_create_surface(wayland.compositor);
        wayland.cursor_surface = wayland.client.compositor_create_surface(wayland.compositor);
        // Create shell_surface
        wayland.shell_surface = wayland.client.zxdg_shell_v6_get_xdg_surface(wayland.shell, wayland.surface);
        // Add listener to shell_surface
        let zxdg_surface_listener = ZxdgSurfaceListener {
            configure: surface_configure
        };
        wayland.client.zxdg_surface_v6_add_listener(wayland.shell_surface, &zxdg_surface_listener,
            ((&mut *wayland) as *mut Self).cast()
        );
        // Create toplevel
        wayland.toplevel = wayland.client.zxdg_surface_v6_get_toplevel(wayland.shell_surface);
        // Add toplevel listener
        let zxdg_toplevel_listener = ZxdgToplevelListener {
            configure: toplevel_configure,
            close: toplevel_close,
        };
        wayland.client.zxdg_toplevel_v6_add_listener(wayland.toplevel, &zxdg_toplevel_listener, 
            ((&mut *wayland) as *mut Self).cast()
        );
        // Set Window & App Title
        let mut window_title = CString::new(name).unwrap();
        wayland.client.zxdg_toplevel_v6_set_title(wayland.toplevel, window_title.as_ptr());
        wayland.client.zxdg_toplevel_v6_set_app_id(wayland.toplevel, window_title.as_ptr());
        // Maximize Window
        wayland.client.zxdg_toplevel_v6_set_maximized(wayland.toplevel);
        // Show Window
        let callback = wayland.client.display_sync(wayland.display.as_ptr());
        // Window Callbacks
        let callback_listener = WlCallbackListener {
            done: configure_callback,
        };
        wayland.client.callback_add_listener(callback, &callback_listener, ((&mut *wayland) as *mut Self).cast());

        Some(wayland)
    }
}

impl crate::Nwin for Wayland {
    fn handle(&self) -> crate::NwinHandle {
        crate::NwinHandle::Wayland(self.display.as_ptr().cast())
    }

    fn connect(&mut self, draw: &mut dyn crate::Draw) {
        match draw.handle() {
            crate::DrawHandle::Gl(_c) => {
                self.egl_window = unsafe {
                    (self.egl.wl_egl_window_create)(
                        self.surface,
                        self.window_width,
                        self.window_height,
                    )
                };
            }
            crate::DrawHandle::Vulkan(_c) => unimplemented!(),
        }
        draw.connect(self.egl_window.cast());
    }

    fn run(&mut self) -> bool {
        let ret = (self.client.wl_display_dispatch)(self.display.as_ptr());

        ret != -1 && self.running
    }

    fn dimensions(&self) -> (u16, u16) {
        (self.window_width as u16, self.window_height as u16)
    }

    fn key_held(&self, key: crate::Key) -> bool {
        false
        // self.keys_states & (1 << key as i8) != 0
    }
}

extern fn registry_global(
    window: *mut c_void,
    registry: *mut WlRegistry,
    name: u32,
    interface: *const c_char,
    _version: u32,
) {
    let window: *mut Wayland = window.cast();
    let interface = str::from_utf8(CStr::from_ptr(interface).to_bytes()).unwrap();

    match interface {
        "wl_compositor" => {
            (*window).compositor = (*window).client.registry_bind(registry, name, (*window).client.wl_compositor_interface, 1).cast();
        }
        "zxdg_shell_v6" => {
            (*window).shell = (*window).client.registry_bind(registry, name, (*window).client.zxdg_shell_v6_interface, 1).cast();

            let xdg_shell_listener = ZxdgShellListener {
                ping: handle_xdg_shell_ping
            };

            (*window).client.zxdg_shell_v6_add_listener((*window).shell, &xdg_shell_listener, window.cast());
        }
        "wl_seat" => {
            (*window).seat = (*window).client.registry_bind(registry, name, (*window).client.wl_seat_interface, 1).cast();

            let nil: *mut c_void = null_mut();
            let seat_listener = WlSeatListener {
                capabilities: seat_handle_capabilities,
                name: transmute(nil),
            };

            (*window).client.seat_add_listener((*window).seat, &seat_listener, window.cast());
        }
        "wl_shm" => {
            (*window).shm = (*window).client.registry_bind(registry, name, (*window).client.wl_shm_interface, 1).cast();

            (*window).cursor_theme =
                ((*window).cursor.wl_cursor_theme_load)(null_mut(), 16, (*window).shm);
            if (*window).cursor_theme.is_null() {
                eprintln!("unable to load default theme");
            }
            (*window).default_cursor = ((*window).cursor.wl_cursor_theme_get_cursor)(
                (*window).cursor_theme,
                CStr::from_bytes_with_nul("left_ptr\0".as_bytes()).unwrap().as_ptr(),
            );
            if (*window).default_cursor.is_null() {
                panic!("unable to load default left pointer");
            }
        }
        "wl_output" => {
            let output = (*window).client.registry_bind(registry, name, (*window).client.wl_output_interface, 1).cast();

            let output_listener = WlOutputListener {
                geometry: output_geometry,
                mode: output_mode,
                done: output_done,
                scale: output_scale,
            };

            (*window).client.output_add_listener(output, &output_listener, window.cast());
        }
        _ => {}
    }
}

extern fn registry_global_remove(
    _data: *mut c_void,
    _registry: *mut WlRegistry,
    _name: u32,
) {
}

extern fn surface_configure(
    window: *mut c_void,
    zxdg_surface_v6: *mut ZxdgSurface,
    serial: u32,
) {
    let window: *mut Wayland = window.cast();

    (*window).client.zxdg_surface_v6_ack_configure(zxdg_surface_v6, serial);
}

extern fn toplevel_configure(
    window: *mut c_void,
    _zxdg_toplevel_v6: *mut ZxdgToplevel,
    width: i32,
    height: i32,
    _states: *mut WlArray,
) {
    let window: *mut Wayland = window.cast();

    if !(*window).egl_window.is_null() && (*window).configured {
        ((*window).egl.wl_egl_window_resize)((*window).egl_window, width, height, 0, 0);
        (*window).configured = false;
        (*window).window_width = width;
        (*window).window_height = height;
    } else {
        if (*window).fullscreen {
        } else if width != 0 && height != 0 {
            if (*window).is_restored {
                (*window).restore_width = (*window).window_width;
                (*window).restore_height = (*window).window_height;
            }
            (*window).is_restored = false;
            if !(*window).egl_window.is_null() {
                ((*window).egl.wl_egl_window_resize)((*window).egl_window, width, height, 0, 0);
            }
            (*window).window_width = width;
            (*window).window_height = height;
        } else {
            (*window).window_width = (*window).restore_width;
            (*window).window_height = (*window).restore_height;
            (*window).is_restored = true;
            if !(*window).egl_window.is_null() {
                ((*window).egl.wl_egl_window_resize)(
                    (*window).egl_window,
                    (*window).restore_width,
                    (*window).restore_height,
                    0,
                    0,
                );
            }
        }
    }
}

extern fn toplevel_close(
    window: *mut c_void,
    _zxdg_toplevel_v6: *mut ZxdgToplevel,
) {
    let window: *mut Wayland = window.cast();

    (*window).running = false;
}

extern fn configure_callback(
    window: *mut c_void,
    callback: *mut WlCallback,
    time: u32,
) {
    let window: *mut Wayland = window.cast();

    (*window).client.callback_destroy(callback);

    // if (*window).callback.is_null() {
    //     redraw_wl(window, std::ptr::null_mut(), time);
    // }
}

extern fn output_geometry(
    _data: *mut c_void,
    _wl_output: *mut WlOutput,
    _x: i32,               // X position of window.
    _y: i32,               // Y position of window.
    _physical_width: i32,  // Width in millimeters.
    _physical_height: i32, // Height in millimeters.
    _subpixel: i32,        // subpixel orientation.
    _make: *const c_char,  // Text of make.
    _model: *const c_char, // Text of model.
    _transform: i32,
) {
}

extern fn output_mode(
    data: *mut c_void,
    _wl_output: *mut WlOutput,
    _flags: u32,
    _width: i32,  // Monitor width (in pixels)
    _height: i32, // Monitor height (in pixels)
    refresh: i32,
) {
    let window: *mut Wayland = data.cast();

    // Convert from frames per 1000 seconds to seconds per frame.
    let refresh = (f64::from(refresh) * 0.001).recip();
    // Convert seconds to nanoseconds.
    (*window).refresh_rate = (refresh * 1_000_000_000.0) as u64;
}

extern fn output_done(
    _data: *mut c_void,
    _wl_output: *mut WlOutput,
) {
}

extern fn output_scale(
    _data: *mut c_void,
    _wl_output: *mut WlOutput,
    _factor: i32, // Pixel doubling
) {
}

extern fn seat_handle_capabilities(
    window: *mut c_void,
    seat: *mut WlSeat,
    caps: u32,
) {
    let window: *mut Wayland = window.cast();

    // Allow Pointer Events
    let has_pointer = (caps & WlSeatCapability::Pointer as u32) != 0;
    if has_pointer && (*window).pointer.is_null() {
        (*window).pointer = (*window).client.seat_get_pointer(seat);

        let nil: *mut c_void = null_mut();
        let pointer_listener = WlPointerListener {
            enter: pointer_handle_enter,
            leave: pointer_handle_leave,
            motion: pointer_handle_motion,
            button: pointer_handle_button,
            axis: pointer_handle_axis,
            frame: transmute(nil),
            axis_source: transmute(nil),
            axis_stop: transmute(nil),
            axis_discrete: transmute(nil),
        };
        
        (*window).client.pointer_add_listener((*window).pointer, &pointer_listener, window.cast());
    } else if !has_pointer && !(*window).pointer.is_null() {
        ((*window).client.wl_proxy_destroy)((*window).pointer.cast());
        (*window).pointer = std::ptr::null_mut();
    }

    // Allow Keyboard Events
    let has_keyboard = (caps & WlSeatCapability::Keyboard as u32) != 0;
    if has_keyboard && (*window).keyboard.is_null() {
        (*window).keyboard = (*window).client.seat_get_keyboard(seat);
        
        let nil: *mut c_void = null_mut();
        let keyboard_listener = WlKeyboardListener {
            keymap: keyboard_handle_keymap,
            enter: keyboard_handle_enter,
            leave: keyboard_handle_leave,
            key: keyboard_handle_key,
            modifiers: keyboard_handle_modifiers,
            repeat_info: transmute(nil),
        };
        
        (*window).client.keyboard_add_listener((*window).keyboard, &keyboard_listener, window.cast());
    } else if !has_keyboard && !(*window).keyboard.is_null() {
        ((*window).client.wl_proxy_destroy)((*window).keyboard.cast());
        (*window).keyboard = std::ptr::null_mut();
    }

    let has_touch = (caps & WlSeatCapability::Touch as u32) != 0;
    if has_touch && (*window).touch.is_null() {
        (*window).touch = (*window).client.seat_get_touch(seat);
        
        // FIXME Allow Touch Events
        // (*window).client.touch_add_listener((*window).touch, &touch_listener, window.cast());
    } else if !has_touch && !(*window).touch.is_null() {
        ((*window).client.wl_proxy_destroy)((*window).touch.cast());
        (*window).touch = std::ptr::null_mut();
    }
}

extern fn handle_xdg_shell_ping(
    window: *mut c_void,
    shell: *mut ZxdgShell,
    serial: u32,
) {
    let window: *mut Wayland = window.cast();

    (*window).client.zxdg_shell_v6_pong(shell, serial);
}

extern fn keyboard_handle_keymap(
    _window: *mut c_void,
    _keyboard: *mut WlKeyboard,
    _format: u32,
    _fd: i32,
    _size: u32,
) {
}

extern fn keyboard_handle_enter(
    _window: *mut c_void,
    _keyboard: *mut WlKeyboard,
    _serial: u32,
    _surface: *mut WlSurface,
    _keys: *mut WlArray,
) {
}

extern fn keyboard_handle_leave(
    _window: *mut c_void,
    _keyboard: *mut WlKeyboard,
    _serial: u32,
    _surface: *mut WlSurface,
) {
}

extern fn keyboard_handle_key(
    window: *mut c_void,
    _keyboard: *mut WlKeyboard,
    _serial: u32,
    _time: u32,
    key: u32,
    state: u32,
) {
    let window: *mut Wayland = window.cast();

    if key == 1 /*KEY_ESC*/ && state != 0 {
        (*window).running = false;
    } else if key == 87 /*KEY_F11*/ && state != 0 {
        (*window).configured = true;

        if (*window).fullscreen {
            (*window).client.zxdg_toplevel_v6_unset_fullscreen((*window).toplevel);
            (*window).fullscreen = false;
        } else {
            (*window).client.zxdg_toplevel_v6_set_fullscreen((*window).toplevel);
            (*window).fullscreen = true;
        }

        let callback = (*window).client.display_sync((*window).display.as_ptr());

        let callback_listener = WlCallbackListener {
            done: configure_callback,
        };
        (*window).client.callback_add_listener(callback, &callback_listener, window.cast());
    } else {
        use crate::Key::*;

        let offset = match key {
            1 => Back,
            2 => Num1,
            3 => Num2,
            4 => Num3,
            5 => Num4,
            6 => Num5,
            7 => Num6,
            8 => Num7,
            9 => Num8,
            10 => Num9,
            11 => Num0,
            12 => Minus,
            13 => Equals,
            14 => Backspace,
            15 => Tab,
            16 => Q,
            17 => W,
            18 => E,
            19 => R,
            20 => T,
            21 => Y,
            22 => U,
            23 => I,
            24 => O,
            25 => P,
            26 => SquareBracketOpen,
            27 => SquareBracketClose,
            28 => Enter,
            29 => LeftCtrl,
            30 => A,
            31 => S,
            32 => D,
            33 => F,
            34 => G,
            35 => H,
            36 => J,
            37 => K,
            38 => L,
            39 => Semicolon,
            40 => Quote,
            41 => Backtick,
            42 => LeftShift,
            43 => Backslash,
            44 => Z,
            45 => X,
            46 => C,
            47 => V,
            48 => B,
            49 => N,
            50 => M,
            51 => Comma,
            52 => Period,
            53 => Slash,
            54 => RightShift,
            55 => NumpadMultiply,
            56 => LeftAlt,
            57 => Space,
            58 => CapsLock,
            59 => F1,
            60 => F2,
            61 => F3,
            62 => F4,
            63 => F5,
            64 => F6,
            65 => F7,
            66 => F8,
            67 => F9,
            68 => F10,
            69 => NumpadLock,
            70 => ScrollLock,
            71 => Numpad7,
            72 => Numpad8,
            73 => Numpad9,
            74 => NumpadSubtract,
            75 => Numpad4,
            76 => Numpad5,
            77 => Numpad6,
            78 => NumpadAdd,
            79 => Numpad1,
            80 => Numpad2,
            81 => Numpad3,
            82 => Numpad0,
            83 => NumpadDot,
            87 => F11,
            88 => F12,
            96 => NumpadEnter,
            97 => RightCtrl,
            98 => NumpadDivide,
            99 => PrintScreen,
            100 => RightAlt,
            102 => Home,
            103 => Up,
            104 => PageUp,
            105 => Left,
            106 => Right,
            107 => End,
            108 => Down,
            109 => PageDown,
            110 => Insert,
            111 => Delete,
            113 => Mute,
            114 => VolumeDown,
            115 => VolumeUp,
            119 => Break,
            125 => System,
            127 => Menu,
            143 =>
            /*Function Key should be ignored*/
            {
                ExtraClick
            }
            163 => FastForward,
            164 => PausePlay,
            165 => Rewind,
            166 => Stop,
            190 => MicrophoneToggle,
            192 => TrackpadOn,
            193 => TrackpadOff,
            212 => CameraToggle,
            224 => BrightnessDown,
            225 => BrightnessUp,
            247 => AirplaneMode,
            e => {
                eprintln!("Error: Unknown key combination: {}", e);
                ExtraClick
            }
        } as i8;

        if !offset.is_negative() {
            let bit = 1 << offset;

            if state == 0 {
                println!("Key release {:b}", bit);
            } else {
                println!("Key press {:b}", bit);
            }
        }
    }
}

extern fn keyboard_handle_modifiers(
    _window: *mut c_void,
    _keyboard: *mut WlKeyboard,
    _serial: u32,
    _mods_depressed: u32,
    _mods_latched: u32,
    _mods_locked: u32,
    _group: u32,
) {
}

extern fn pointer_handle_enter(
    window: *mut c_void,
    pointer: *mut WlPointer,
    serial: u32,
    _surface: *mut WlSurface,
    _sx: i32,
    _sy: i32,
) {
    let window: *mut Wayland = window.cast();

    let cursor = (*window).default_cursor;
    let image = *(*cursor).images;
    let buffer = ((*window).cursor.wl_cursor_image_get_buffer)(image);
    if buffer.is_null() {
        return;
    }

    (*window).client.pointer_set_cursor(pointer, (*window).cursor_surface, image, serial);
    (*window).client.surface_attach((*window).cursor_surface, buffer);
    (*window).client.surface_damage((*window).cursor_surface, image);
    (*window).client.surface_commit((*window).cursor_surface);
}

extern fn pointer_handle_leave(
    _window: *mut c_void,
    _pointer: *mut WlPointer,
    _serial: u32,
    _surface: *mut WlSurface,
) {
}

extern fn pointer_handle_motion(
    window: *mut c_void,
    _pointer: *mut WlPointer,
    _time: u32,
    x: i32,
    y: i32,
) {
    let window: *mut Wayland = window.cast();

    let x = x as f32 / 256.0;
    let y = y as f32 / 256.0;

    println!("Pointer motion: {} {}", x, y);
}

extern fn pointer_handle_button(
    window: *mut c_void,
    _pointer: *mut WlPointer,
    serial: u32,
    _time: u32,
    button: u32,
    state: u32,
) {
    let window: *mut Wayland = window.cast();

    match button {
        0x110 /*BTN_LEFT*/ => {
            // pressed.
            if state != 0 {
                /*if (*window).pointer_xy.1 < f32::from((*window).toolbar_height) {
                    wl_proxy_marshal(
                        (*c).toplevel,
                        5, /*ZXDG_TOPLEVEL_V6_MOVE*/
                        (*c).seat,
                        serial,
                    );
                } else {*/
                    println!("Press");
                //}
            } else {
                println!("Release");
            }
        }
        0x111 /*BTN_RIGHT*/ => {}
        0x112 /*BTN_MIDDLE*/ => {}
        0x113 /*BTN_SIDE*/ => {}
        _ => eprintln!("Unknown"),
    }
}

extern fn pointer_handle_axis(
    _window: *mut c_void,
    _pointer: *mut WlPointer,
    _time: u32,
    _axis: u32,
    _value: i32,
) {
}
