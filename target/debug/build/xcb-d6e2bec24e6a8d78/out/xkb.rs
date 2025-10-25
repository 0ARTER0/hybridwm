// This source file is generated automatically from xkb.xml

use crate::base::{self, BaseError, BaseEvent, GeEvent, Raw, Reply, WiredIn, WiredOut, Xid};
use crate::ext;
use crate::ffi::base::*;
use crate::ffi::ext::*;
use crate::lat1_str::{Lat1Str, Lat1String, Lat1StrF};
use crate::xproto;
use crate::xproto::PropEl;

use bitflags::bitflags;
use libc::{self, iovec};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::os::unix::io::RawFd;

/// The official identifier for the `xkb` extension.
pub const XNAME: &str = "XKEYBOARD";
/// The major version of the `xkb` extension.
pub const MAJOR_VERSION: u32 = 1;
/// The minor version of the `xkb` extension.
pub const MINOR_VERSION: u32 = 0;
/// The version string of the `xkb` extension.
pub const VERSION_STRING: &str = "1.0";

pub(crate) static mut FFI_EXT: xcb_extension_t = xcb_extension_t {
    name: "XKEYBOARD\0".as_ptr() as *const _,
    global_id: 0,
};

/// Prefetch server runtime info data of the `xkb` extension.
pub fn prefetch_extension_data(conn: &base::Connection) {
    unsafe {
        xcb_prefetch_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
    }
}

/// Fetch server runtime info data of the `xkb` extension.
///
/// Might be non-blocking if [prefetch_extension_data] was called before.
/// This function is of seldom use as the extensions are initialized by the
/// [Connection](crate::Connection) constructor.
pub fn get_extension_data(conn: &base::Connection) -> std::option::Option<ext::ExtensionData> {
    unsafe {
        let reply = xcb_get_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
        assert!(!reply.is_null(), "Could not fetch xkb extension data");
        let reply = xproto::QueryExtensionReply::from_raw(reply);
        if !reply.present() {
            std::mem::forget(reply);
            return None;
        }
        let res = ext::ExtensionData{
            ext: ext::Extension::Xkb,
            major_opcode: reply.major_opcode(),
            first_event: reply.first_event(),
            first_error: reply.first_error(),
        };
        std::mem::forget(reply);
        Some(res)
    }
}

/// The `KeyboardError` error.
pub struct KeyboardError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for KeyboardError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { KeyboardError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for KeyboardError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);

    const NUMBER: u32 = 0;
}

impl KeyboardError {
    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    fn wire_len(&self) -> usize { 32 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn error_code(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn value(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn minor_opcode(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn major_opcode(&self) -> u8 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

}

impl std::fmt::Debug for KeyboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyboardError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .field("value", &self.value())
            .field("minor_opcode", &self.minor_opcode())
            .field("major_opcode", &self.major_opcode())
            .field("pad", &21)
            .finish()
    }
}

impl Drop for KeyboardError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for KeyboardError {}
unsafe impl Sync for KeyboardError {}

/// Unified error type for the Xkb extension
#[derive(Debug)]
pub enum Error {
    Keyboard(KeyboardError),
}

impl Error {
  pub fn as_raw(&self) -> *mut xcb_generic_error_t {
    match self {
      Self::Keyboard(e) => e.as_raw(),
    }
  }
}

impl base::ResolveWireError for Error {
    unsafe fn resolve_wire_error(first_error: u8, raw: *mut xcb_generic_error_t) -> std::option::Option<Self> {
        debug_assert!(!raw.is_null());
        let error_code = (*raw).error_code;
        match error_code - first_error {
            0 => std::option::Option::Some(Error::Keyboard(KeyboardError::from_raw(raw))),
            _ => std::option::Option::None,
        }
    }
}

/// The `NewKeyboardNotifyEvent` event.
pub struct NewKeyboardNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for NewKeyboardNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { NewKeyboardNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for NewKeyboardNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 0;
}

impl NewKeyboardNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        old_device_id: u8,
        min_key_code: xproto::Keycode,
        max_key_code: xproto::Keycode,
        old_min_key_code: xproto::Keycode,
        old_max_key_code: xproto::Keycode,
        request_major: u8,
        request_minor: u8,
        changed: NknDetail,
    ) -> NewKeyboardNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += old_device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += min_key_code.serialize(&mut wire_buf[wire_off ..]);
            wire_off += max_key_code.serialize(&mut wire_buf[wire_off ..]);
            wire_off += old_min_key_code.serialize(&mut wire_buf[wire_off ..]);
            wire_off += old_max_key_code.serialize(&mut wire_buf[wire_off ..]);
            wire_off += request_major.serialize(&mut wire_buf[wire_off ..]);
            wire_off += request_minor.serialize(&mut wire_buf[wire_off ..]);
            (changed.bits() as u16).serialize(&mut wire_buf[wire_off ..]);

            NewKeyboardNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn old_device_id(&self) -> u8 {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn min_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn max_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 11usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn old_min_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn old_max_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn request_major(&self) -> u8 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn request_minor(&self) -> u8 {
        unsafe {
            let offset = 15usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn changed(&self) -> NknDetail {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, NknDetail>(val)
        }
    }

}

impl std::fmt::Debug for NewKeyboardNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewKeyboardNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("old_device_id", &self.old_device_id())
            .field("min_key_code", &self.min_key_code())
            .field("max_key_code", &self.max_key_code())
            .field("old_min_key_code", &self.old_min_key_code())
            .field("old_max_key_code", &self.old_max_key_code())
            .field("request_major", &self.request_major())
            .field("request_minor", &self.request_minor())
            .field("changed", &self.changed())
            .field("pad", &14)
            .finish()
    }
}

impl base::WiredOut for NewKeyboardNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for NewKeyboardNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        NewKeyboardNotifyEvent { raw }
    }
}

impl Drop for NewKeyboardNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for NewKeyboardNotifyEvent {}
unsafe impl Sync for NewKeyboardNotifyEvent {}

/// The `MapNotifyEvent` event.
pub struct MapNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for MapNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { MapNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for MapNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 1;
}

impl MapNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        ptr_btn_actions: u8,
        changed: MapPart,
        min_key_code: xproto::Keycode,
        max_key_code: xproto::Keycode,
        first_type: u8,
        n_types: u8,
        first_key_sym: xproto::Keycode,
        n_key_syms: u8,
        first_key_act: xproto::Keycode,
        n_key_acts: u8,
        first_key_behavior: xproto::Keycode,
        n_key_behavior: u8,
        first_key_explicit: xproto::Keycode,
        n_key_explicit: u8,
        first_mod_map_key: xproto::Keycode,
        n_mod_map_keys: u8,
        first_v_mod_map_key: xproto::Keycode,
        n_v_mod_map_keys: u8,
        virtual_mods: VMod,
    ) -> MapNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 1u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += ptr_btn_actions.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (changed.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += min_key_code.serialize(&mut wire_buf[wire_off ..]);
            wire_off += max_key_code.serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_types.serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_key_sym.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_key_syms.serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_key_act.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_key_acts.serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_key_behavior.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_key_behavior.serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_key_explicit.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_key_explicit.serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_mod_map_key.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_mod_map_keys.serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_v_mod_map_key.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_v_mod_map_keys.serialize(&mut wire_buf[wire_off ..]);
            (virtual_mods.bits() as u16).serialize(&mut wire_buf[wire_off ..]);

            MapNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn ptr_btn_actions(&self) -> u8 {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn changed(&self) -> MapPart {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, MapPart>(val)
        }
    }

    pub fn min_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn max_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn first_type(&self) -> u8 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn n_types(&self) -> u8 {
        unsafe {
            let offset = 15usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_key_sym(&self) -> xproto::Keycode {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_key_syms(&self) -> u8 {
        unsafe {
            let offset = 17usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_key_act(&self) -> xproto::Keycode {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_key_acts(&self) -> u8 {
        unsafe {
            let offset = 19usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_key_behavior(&self) -> xproto::Keycode {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_key_behavior(&self) -> u8 {
        unsafe {
            let offset = 21usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_key_explicit(&self) -> xproto::Keycode {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_key_explicit(&self) -> u8 {
        unsafe {
            let offset = 23usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_mod_map_key(&self) -> xproto::Keycode {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_mod_map_keys(&self) -> u8 {
        unsafe {
            let offset = 25usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_v_mod_map_key(&self) -> xproto::Keycode {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_v_mod_map_keys(&self) -> u8 {
        unsafe {
            let offset = 27usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn virtual_mods(&self) -> VMod {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

}

impl std::fmt::Debug for MapNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("ptr_btn_actions", &self.ptr_btn_actions())
            .field("changed", &self.changed())
            .field("min_key_code", &self.min_key_code())
            .field("max_key_code", &self.max_key_code())
            .field("first_type", &self.first_type())
            .field("n_types", &self.n_types())
            .field("first_key_sym", &self.first_key_sym())
            .field("n_key_syms", &self.n_key_syms())
            .field("first_key_act", &self.first_key_act())
            .field("n_key_acts", &self.n_key_acts())
            .field("first_key_behavior", &self.first_key_behavior())
            .field("n_key_behavior", &self.n_key_behavior())
            .field("first_key_explicit", &self.first_key_explicit())
            .field("n_key_explicit", &self.n_key_explicit())
            .field("first_mod_map_key", &self.first_mod_map_key())
            .field("n_mod_map_keys", &self.n_mod_map_keys())
            .field("first_v_mod_map_key", &self.first_v_mod_map_key())
            .field("n_v_mod_map_keys", &self.n_v_mod_map_keys())
            .field("virtual_mods", &self.virtual_mods())
            .field("pad", &2)
            .finish()
    }
}

impl base::WiredOut for MapNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for MapNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        MapNotifyEvent { raw }
    }
}

impl Drop for MapNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for MapNotifyEvent {}
unsafe impl Sync for MapNotifyEvent {}

/// The `StateNotifyEvent` event.
pub struct StateNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for StateNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { StateNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for StateNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 2;
}

impl StateNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        mods: xproto::ModMask,
        base_mods: xproto::ModMask,
        latched_mods: xproto::ModMask,
        locked_mods: xproto::ModMask,
        group: Group,
        base_group: i16,
        latched_group: i16,
        locked_group: Group,
        compat_state: xproto::ModMask,
        grab_mods: xproto::ModMask,
        compat_grab_mods: xproto::ModMask,
        lookup_mods: xproto::ModMask,
        compat_loockup_mods: xproto::ModMask,
        ptr_btn_state: xproto::KeyButMask,
        changed: StatePart,
        keycode: xproto::Keycode,
        event_type: u8,
        request_major: u8,
        request_minor: u8,
    ) -> StateNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 2u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (mods.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (base_mods.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (latched_mods.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (locked_mods.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(group) as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += base_group.serialize(&mut wire_buf[wire_off ..]);
            wire_off += latched_group.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(locked_group) as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (compat_state.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (grab_mods.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (compat_grab_mods.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (lookup_mods.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (compat_loockup_mods.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (ptr_btn_state.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (changed.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += keycode.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += request_major.serialize(&mut wire_buf[wire_off ..]);
            request_minor.serialize(&mut wire_buf[wire_off ..]);

            StateNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn base_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn latched_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 11usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn locked_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn group(&self) -> Group {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, Group>(val)
        }
    }

    pub fn base_group(&self) -> i16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            ptr.read_unaligned()
        }
    }

    pub fn latched_group(&self) -> i16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            ptr.read_unaligned()
        }
    }

    pub fn locked_group(&self) -> Group {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, Group>(val)
        }
    }

    pub fn compat_state(&self) -> xproto::ModMask {
        unsafe {
            let offset = 19usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn grab_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn compat_grab_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 21usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn lookup_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn compat_loockup_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 23usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn ptr_btn_state(&self) -> xproto::KeyButMask {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::KeyButMask>(val)
        }
    }

    pub fn changed(&self) -> StatePart {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, StatePart>(val)
        }
    }

    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn event_type(&self) -> u8 {
        unsafe {
            let offset = 29usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn request_major(&self) -> u8 {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn request_minor(&self) -> u8 {
        unsafe {
            let offset = 31usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }
}

impl std::fmt::Debug for StateNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("mods", &self.mods())
            .field("base_mods", &self.base_mods())
            .field("latched_mods", &self.latched_mods())
            .field("locked_mods", &self.locked_mods())
            .field("group", &self.group())
            .field("base_group", &self.base_group())
            .field("latched_group", &self.latched_group())
            .field("locked_group", &self.locked_group())
            .field("compat_state", &self.compat_state())
            .field("grab_mods", &self.grab_mods())
            .field("compat_grab_mods", &self.compat_grab_mods())
            .field("lookup_mods", &self.lookup_mods())
            .field("compat_loockup_mods", &self.compat_loockup_mods())
            .field("ptr_btn_state", &self.ptr_btn_state())
            .field("changed", &self.changed())
            .field("keycode", &self.keycode())
            .field("event_type", &self.event_type())
            .field("request_major", &self.request_major())
            .field("request_minor", &self.request_minor())
            .finish()
    }
}

impl base::WiredOut for StateNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for StateNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        StateNotifyEvent { raw }
    }
}

impl Drop for StateNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for StateNotifyEvent {}
unsafe impl Sync for StateNotifyEvent {}

/// The `ControlsNotifyEvent` event.
pub struct ControlsNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ControlsNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ControlsNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ControlsNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 3;
}

impl ControlsNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        num_groups: u8,
        changed_controls: Control,
        enabled_controls: BoolCtrl,
        enabled_control_changes: BoolCtrl,
        keycode: xproto::Keycode,
        event_type: u8,
        request_major: u8,
        request_minor: u8,
    ) -> ControlsNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 3u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += num_groups.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 2usize;
            wire_off += (changed_controls.bits() as u32).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (enabled_controls.bits() as u32).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (enabled_control_changes.bits() as u32).serialize(&mut wire_buf[wire_off ..]);
            wire_off += keycode.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += request_major.serialize(&mut wire_buf[wire_off ..]);
            request_minor.serialize(&mut wire_buf[wire_off ..]);

            ControlsNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn num_groups(&self) -> u8 {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn changed_controls(&self) -> Control {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, Control>(val)
        }
    }

    pub fn enabled_controls(&self) -> BoolCtrl {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BoolCtrl>(val)
        }
    }

    pub fn enabled_control_changes(&self) -> BoolCtrl {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BoolCtrl>(val)
        }
    }

    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn event_type(&self) -> u8 {
        unsafe {
            let offset = 25usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn request_major(&self) -> u8 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn request_minor(&self) -> u8 {
        unsafe {
            let offset = 27usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

}

impl std::fmt::Debug for ControlsNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ControlsNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("num_groups", &self.num_groups())
            .field("pad", &2)
            .field("changed_controls", &self.changed_controls())
            .field("enabled_controls", &self.enabled_controls())
            .field("enabled_control_changes", &self.enabled_control_changes())
            .field("keycode", &self.keycode())
            .field("event_type", &self.event_type())
            .field("request_major", &self.request_major())
            .field("request_minor", &self.request_minor())
            .field("pad", &4)
            .finish()
    }
}

impl base::WiredOut for ControlsNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for ControlsNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ControlsNotifyEvent { raw }
    }
}

impl Drop for ControlsNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ControlsNotifyEvent {}
unsafe impl Sync for ControlsNotifyEvent {}

/// The `IndicatorStateNotifyEvent` event.
pub struct IndicatorStateNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for IndicatorStateNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { IndicatorStateNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for IndicatorStateNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 4;
}

impl IndicatorStateNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        state: u32,
        state_changed: u32,
    ) -> IndicatorStateNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 4u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 3usize;
            wire_off += state.serialize(&mut wire_buf[wire_off ..]);
            state_changed.serialize(&mut wire_buf[wire_off ..]);

            IndicatorStateNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn state(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn state_changed(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

}

impl std::fmt::Debug for IndicatorStateNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndicatorStateNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("pad", &3)
            .field("state", &self.state())
            .field("state_changed", &self.state_changed())
            .field("pad", &12)
            .finish()
    }
}

impl base::WiredOut for IndicatorStateNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for IndicatorStateNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        IndicatorStateNotifyEvent { raw }
    }
}

impl Drop for IndicatorStateNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for IndicatorStateNotifyEvent {}
unsafe impl Sync for IndicatorStateNotifyEvent {}

/// The `IndicatorMapNotifyEvent` event.
pub struct IndicatorMapNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for IndicatorMapNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { IndicatorMapNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for IndicatorMapNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 5;
}

impl IndicatorMapNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        state: u32,
        map_changed: u32,
    ) -> IndicatorMapNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 5u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 3usize;
            wire_off += state.serialize(&mut wire_buf[wire_off ..]);
            map_changed.serialize(&mut wire_buf[wire_off ..]);

            IndicatorMapNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn state(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn map_changed(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

}

impl std::fmt::Debug for IndicatorMapNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndicatorMapNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("pad", &3)
            .field("state", &self.state())
            .field("map_changed", &self.map_changed())
            .field("pad", &12)
            .finish()
    }
}

impl base::WiredOut for IndicatorMapNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for IndicatorMapNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        IndicatorMapNotifyEvent { raw }
    }
}

impl Drop for IndicatorMapNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for IndicatorMapNotifyEvent {}
unsafe impl Sync for IndicatorMapNotifyEvent {}

/// The `NamesNotifyEvent` event.
pub struct NamesNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for NamesNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { NamesNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for NamesNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 6;
}

impl NamesNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        changed: NameDetail,
        first_type: u8,
        n_types: u8,
        first_level_name: u8,
        n_level_names: u8,
        n_radio_groups: u8,
        n_key_aliases: u8,
        changed_group_names: SetOfGroup,
        changed_virtual_mods: VMod,
        first_key: xproto::Keycode,
        n_keys: u8,
        changed_indicators: u32,
    ) -> NamesNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 6u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += (changed.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_types.serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_level_name.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_level_names.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += n_radio_groups.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_key_aliases.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (changed_group_names.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (changed_virtual_mods.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_key.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_keys.serialize(&mut wire_buf[wire_off ..]);
            changed_indicators.serialize(&mut wire_buf[wire_off ..]);

            NamesNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn changed(&self) -> NameDetail {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, NameDetail>(val)
        }
    }

    pub fn first_type(&self) -> u8 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn n_types(&self) -> u8 {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_level_name(&self) -> u8 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn n_level_names(&self) -> u8 {
        unsafe {
            let offset = 15usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn n_radio_groups(&self) -> u8 {
        unsafe {
            let offset = 17usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn n_key_aliases(&self) -> u8 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn changed_group_names(&self) -> SetOfGroup {
        unsafe {
            let offset = 19usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, SetOfGroup>(val)
        }
    }

    pub fn changed_virtual_mods(&self) -> VMod {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

    pub fn first_key(&self) -> xproto::Keycode {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_keys(&self) -> u8 {
        unsafe {
            let offset = 23usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn changed_indicators(&self) -> u32 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

}

impl std::fmt::Debug for NamesNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NamesNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("pad", &1)
            .field("changed", &self.changed())
            .field("first_type", &self.first_type())
            .field("n_types", &self.n_types())
            .field("first_level_name", &self.first_level_name())
            .field("n_level_names", &self.n_level_names())
            .field("pad", &1)
            .field("n_radio_groups", &self.n_radio_groups())
            .field("n_key_aliases", &self.n_key_aliases())
            .field("changed_group_names", &self.changed_group_names())
            .field("changed_virtual_mods", &self.changed_virtual_mods())
            .field("first_key", &self.first_key())
            .field("n_keys", &self.n_keys())
            .field("changed_indicators", &self.changed_indicators())
            .field("pad", &4)
            .finish()
    }
}

impl base::WiredOut for NamesNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for NamesNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        NamesNotifyEvent { raw }
    }
}

impl Drop for NamesNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for NamesNotifyEvent {}
unsafe impl Sync for NamesNotifyEvent {}

/// The `CompatMapNotifyEvent` event.
pub struct CompatMapNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for CompatMapNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { CompatMapNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for CompatMapNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 7;
}

impl CompatMapNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        changed_groups: SetOfGroup,
        first_si: u16,
        n_si: u16,
        n_total_si: u16,
    ) -> CompatMapNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 7u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (changed_groups.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_si.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_si.serialize(&mut wire_buf[wire_off ..]);
            n_total_si.serialize(&mut wire_buf[wire_off ..]);

            CompatMapNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn changed_groups(&self) -> SetOfGroup {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, SetOfGroup>(val)
        }
    }

    pub fn first_si(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn n_si(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn n_total_si(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

}

impl std::fmt::Debug for CompatMapNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompatMapNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("changed_groups", &self.changed_groups())
            .field("first_si", &self.first_si())
            .field("n_si", &self.n_si())
            .field("n_total_si", &self.n_total_si())
            .field("pad", &16)
            .finish()
    }
}

impl base::WiredOut for CompatMapNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for CompatMapNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        CompatMapNotifyEvent { raw }
    }
}

impl Drop for CompatMapNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for CompatMapNotifyEvent {}
unsafe impl Sync for CompatMapNotifyEvent {}

/// The `BellNotifyEvent` event.
pub struct BellNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for BellNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { BellNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for BellNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 8;
}

impl BellNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        bell_class: BellClassResult,
        bell_id: u8,
        percent: u8,
        pitch: u16,
        duration: u16,
        name: xproto::Atom,
        window: xproto::Window,
        event_only: bool,
    ) -> BellNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 8u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(bell_class) as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += bell_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += percent.serialize(&mut wire_buf[wire_off ..]);
            wire_off += pitch.serialize(&mut wire_buf[wire_off ..]);
            wire_off += duration.serialize(&mut wire_buf[wire_off ..]);
            wire_off += name.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            let event_only: u8 = if event_only { 1 } else { 0 };
            event_only.serialize(&mut wire_buf[wire_off ..]);

            BellNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn bell_class(&self) -> BellClassResult {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BellClassResult>(val)
        }
    }

    pub fn bell_id(&self) -> u8 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn percent(&self) -> u8 {
        unsafe {
            let offset = 11usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn pitch(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn duration(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn name(&self) -> xproto::Atom {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            ptr.read_unaligned()
        }
    }

    pub fn window(&self) -> xproto::Window {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            ptr.read_unaligned()
        }
    }

    pub fn event_only(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(24usize)) };
        val != 0
    }

}

impl std::fmt::Debug for BellNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BellNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("bell_class", &self.bell_class())
            .field("bell_id", &self.bell_id())
            .field("percent", &self.percent())
            .field("pitch", &self.pitch())
            .field("duration", &self.duration())
            .field("name", &self.name())
            .field("window", &self.window())
            .field("event_only", &self.event_only())
            .field("pad", &7)
            .finish()
    }
}

impl base::WiredOut for BellNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for BellNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        BellNotifyEvent { raw }
    }
}

impl Drop for BellNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for BellNotifyEvent {}
unsafe impl Sync for BellNotifyEvent {}

/// The `ActionMessageEvent` event.
pub struct ActionMessageEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ActionMessageEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ActionMessageEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ActionMessageEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 9;
}

impl ActionMessageEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        keycode: xproto::Keycode,
        press: bool,
        key_event_follows: bool,
        mods: xproto::ModMask,
        group: Group,
        message: [String8; 8],
    ) -> ActionMessageEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 9u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += keycode.serialize(&mut wire_buf[wire_off ..]);
            let press: u8 = if press { 1 } else { 0 };
            wire_off += press.serialize(&mut wire_buf[wire_off ..]);
            let key_event_follows: u8 = if key_event_follows { 1 } else { 0 };
            wire_off += key_event_follows.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (mods.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(group) as u8).serialize(&mut wire_buf[wire_off ..]);
            std::slice::from_raw_parts_mut(ptr.add(wire_off) as *mut String8, 8usize)
                .copy_from_slice(&message);

            ActionMessageEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn press(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(10usize)) };
        val != 0
    }

    pub fn key_event_follows(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(11usize)) };
        val != 0
    }

    pub fn mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn group(&self) -> Group {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, Group>(val)
        }
    }

    pub fn message(&self) -> &[String8; 8] {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const [String8; 8];
            &*ptr
        }
    }

}

impl std::fmt::Debug for ActionMessageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionMessageEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("keycode", &self.keycode())
            .field("press", &self.press())
            .field("key_event_follows", &self.key_event_follows())
            .field("mods", &self.mods())
            .field("group", &self.group())
            .field("message", &self.message())
            .field("pad", &10)
            .finish()
    }
}

impl base::WiredOut for ActionMessageEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for ActionMessageEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ActionMessageEvent { raw }
    }
}

impl Drop for ActionMessageEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ActionMessageEvent {}
unsafe impl Sync for ActionMessageEvent {}

/// The `AccessXNotifyEvent` event.
pub struct AccessXNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for AccessXNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { AccessXNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for AccessXNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 10;
}

impl AccessXNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        keycode: xproto::Keycode,
        detailt: AxnDetail,
        slow_keys_delay: u16,
        debounce_delay: u16,
    ) -> AccessXNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 10u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += keycode.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (detailt.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += slow_keys_delay.serialize(&mut wire_buf[wire_off ..]);
            debounce_delay.serialize(&mut wire_buf[wire_off ..]);

            AccessXNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn detailt(&self) -> AxnDetail {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, AxnDetail>(val)
        }
    }

    pub fn slow_keys_delay(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn debounce_delay(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

}

impl std::fmt::Debug for AccessXNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessXNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("keycode", &self.keycode())
            .field("detailt", &self.detailt())
            .field("slow_keys_delay", &self.slow_keys_delay())
            .field("debounce_delay", &self.debounce_delay())
            .field("pad", &16)
            .finish()
    }
}

impl base::WiredOut for AccessXNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for AccessXNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        AccessXNotifyEvent { raw }
    }
}

impl Drop for AccessXNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for AccessXNotifyEvent {}
unsafe impl Sync for AccessXNotifyEvent {}

/// The `ExtensionDeviceNotifyEvent` event.
pub struct ExtensionDeviceNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ExtensionDeviceNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ExtensionDeviceNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ExtensionDeviceNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Xkb);
    const NUMBER: u32 = 11;
}

impl ExtensionDeviceNotifyEvent {
    pub fn new(event_base: u8,
        xkb_type: u8,
        time: xproto::Timestamp,
        device_id: u8,
        reason: XiFeature,
        led_class: LedClassResult,
        led_id: u16,
        leds_defined: u32,
        led_state: u32,
        first_button: u8,
        n_buttons: u8,
        supported: XiFeature,
        unsupported: XiFeature,
    ) -> ExtensionDeviceNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 11u8 + event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += xkb_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += device_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += (reason.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(led_class) as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += led_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += leds_defined.serialize(&mut wire_buf[wire_off ..]);
            wire_off += led_state.serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_button.serialize(&mut wire_buf[wire_off ..]);
            wire_off += n_buttons.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (supported.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            (unsupported.bits() as u16).serialize(&mut wire_buf[wire_off ..]);

            ExtensionDeviceNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn xkb_type(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn time(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn reason(&self) -> XiFeature {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, XiFeature>(val)
        }
    }

    pub fn led_class(&self) -> LedClassResult {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, LedClassResult>(val)
        }
    }

    pub fn led_id(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn leds_defined(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn led_state(&self) -> u32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn first_button(&self) -> u8 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn n_buttons(&self) -> u8 {
        unsafe {
            let offset = 25usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn supported(&self) -> XiFeature {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, XiFeature>(val)
        }
    }

    pub fn unsupported(&self) -> XiFeature {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, XiFeature>(val)
        }
    }

}

impl std::fmt::Debug for ExtensionDeviceNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtensionDeviceNotifyEvent")
            .field("response_type", &self.response_type())
            .field("xkb_type", &self.xkb_type())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("device_id", &self.device_id())
            .field("pad", &1)
            .field("reason", &self.reason())
            .field("led_class", &self.led_class())
            .field("led_id", &self.led_id())
            .field("leds_defined", &self.leds_defined())
            .field("led_state", &self.led_state())
            .field("first_button", &self.first_button())
            .field("n_buttons", &self.n_buttons())
            .field("supported", &self.supported())
            .field("unsupported", &self.unsupported())
            .field("pad", &2)
            .finish()
    }
}

impl base::WiredOut for ExtensionDeviceNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for ExtensionDeviceNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ExtensionDeviceNotifyEvent { raw }
    }
}

impl Drop for ExtensionDeviceNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ExtensionDeviceNotifyEvent {}
unsafe impl Sync for ExtensionDeviceNotifyEvent {}

/// Unified event type for the Xkb extension
#[derive(Debug)]
pub enum Event {
    NewKeyboardNotify(NewKeyboardNotifyEvent),
    MapNotify(MapNotifyEvent),
    StateNotify(StateNotifyEvent),
    ControlsNotify(ControlsNotifyEvent),
    IndicatorStateNotify(IndicatorStateNotifyEvent),
    IndicatorMapNotify(IndicatorMapNotifyEvent),
    NamesNotify(NamesNotifyEvent),
    CompatMapNotify(CompatMapNotifyEvent),
    BellNotify(BellNotifyEvent),
    ActionMessage(ActionMessageEvent),
    AccessXNotify(AccessXNotifyEvent),
    ExtensionDeviceNotify(ExtensionDeviceNotifyEvent),
}

impl Event {
  pub fn as_raw(&self) -> *mut xcb_generic_event_t {
    match self {
      Self::NewKeyboardNotify(e) => e.as_raw(),
      Self::MapNotify(e) => e.as_raw(),
      Self::StateNotify(e) => e.as_raw(),
      Self::ControlsNotify(e) => e.as_raw(),
      Self::IndicatorStateNotify(e) => e.as_raw(),
      Self::IndicatorMapNotify(e) => e.as_raw(),
      Self::NamesNotify(e) => e.as_raw(),
      Self::CompatMapNotify(e) => e.as_raw(),
      Self::BellNotify(e) => e.as_raw(),
      Self::ActionMessage(e) => e.as_raw(),
      Self::AccessXNotify(e) => e.as_raw(),
      Self::ExtensionDeviceNotify(e) => e.as_raw(),
    }
  }
}

impl base::ResolveWireEvent for Event {
    unsafe fn resolve_wire_event(first_event: u8, raw: *mut xcb_generic_event_t) -> std::option::Option<Self> {
        debug_assert!(!raw.is_null());
        let response_type = (*raw).response_type & 0x7F;
        debug_assert!(response_type != 0, "This is not an event but an error!");
        debug_assert!(response_type != XCB_GE_GENERIC, "This is a GE_GENERIC event!");
        assert_eq!(response_type, first_event, "This is not an Xkb event");
        let ptr = raw as *const u8;
        let xkb_type = *(ptr.add(1));
        match xkb_type {
            0 => std::option::Option::Some(Event::NewKeyboardNotify(NewKeyboardNotifyEvent::from_raw(raw))),
            1 => std::option::Option::Some(Event::MapNotify(MapNotifyEvent::from_raw(raw))),
            2 => std::option::Option::Some(Event::StateNotify(StateNotifyEvent::from_raw(raw))),
            3 => std::option::Option::Some(Event::ControlsNotify(ControlsNotifyEvent::from_raw(raw))),
            4 => std::option::Option::Some(Event::IndicatorStateNotify(IndicatorStateNotifyEvent::from_raw(raw))),
            5 => std::option::Option::Some(Event::IndicatorMapNotify(IndicatorMapNotifyEvent::from_raw(raw))),
            6 => std::option::Option::Some(Event::NamesNotify(NamesNotifyEvent::from_raw(raw))),
            7 => std::option::Option::Some(Event::CompatMapNotify(CompatMapNotifyEvent::from_raw(raw))),
            8 => std::option::Option::Some(Event::BellNotify(BellNotifyEvent::from_raw(raw))),
            9 => std::option::Option::Some(Event::ActionMessage(ActionMessageEvent::from_raw(raw))),
            10 => std::option::Option::Some(Event::AccessXNotify(AccessXNotifyEvent::from_raw(raw))),
            11 => std::option::Option::Some(Event::ExtensionDeviceNotify(ExtensionDeviceNotifyEvent::from_raw(raw))),
            _ => std::option::Option::None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Const {
    MaxLegalKeyCode = 255,
    PerKeyBitArraySize = 32,
    KeyNameLength = 4,
}

bitflags! {
    pub struct EventType: u32 {
        const NEW_KEYBOARD_NOTIFY = 0x00000001;
        const MAP_NOTIFY = 0x00000002;
        const STATE_NOTIFY = 0x00000004;
        const CONTROLS_NOTIFY = 0x00000008;
        const INDICATOR_STATE_NOTIFY = 0x00000010;
        const INDICATOR_MAP_NOTIFY = 0x00000020;
        const NAMES_NOTIFY = 0x00000040;
        const COMPAT_MAP_NOTIFY = 0x00000080;
        const BELL_NOTIFY = 0x00000100;
        const ACTION_MESSAGE = 0x00000200;
        const ACCESS_X_NOTIFY = 0x00000400;
        const EXTENSION_DEVICE_NOTIFY = 0x00000800;
    }
}

bitflags! {
    pub struct NknDetail: u32 {
        const KEYCODES = 0x00000001;
        const GEOMETRY = 0x00000002;
        const DEVICE_ID = 0x00000004;
    }
}

bitflags! {
    pub struct AxnDetail: u32 {
        const SK_PRESS = 0x00000001;
        const SK_ACCEPT = 0x00000002;
        const SK_REJECT = 0x00000004;
        const SK_RELEASE = 0x00000008;
        const BK_ACCEPT = 0x00000010;
        const BK_REJECT = 0x00000020;
        const AXK_WARNING = 0x00000040;
    }
}

bitflags! {
    pub struct MapPart: u32 {
        const KEY_TYPES = 0x00000001;
        const KEY_SYMS = 0x00000002;
        const MODIFIER_MAP = 0x00000004;
        const EXPLICIT_COMPONENTS = 0x00000008;
        const KEY_ACTIONS = 0x00000010;
        const KEY_BEHAVIORS = 0x00000020;
        const VIRTUAL_MODS = 0x00000040;
        const VIRTUAL_MOD_MAP = 0x00000080;
    }
}

bitflags! {
    pub struct SetMapFlags: u32 {
        const RESIZE_TYPES = 0x00000001;
        const RECOMPUTE_ACTIONS = 0x00000002;
    }
}

bitflags! {
    pub struct StatePart: u32 {
        const MODIFIER_STATE = 0x00000001;
        const MODIFIER_BASE = 0x00000002;
        const MODIFIER_LATCH = 0x00000004;
        const MODIFIER_LOCK = 0x00000008;
        const GROUP_STATE = 0x00000010;
        const GROUP_BASE = 0x00000020;
        const GROUP_LATCH = 0x00000040;
        const GROUP_LOCK = 0x00000080;
        const COMPAT_STATE = 0x00000100;
        const GRAB_MODS = 0x00000200;
        const COMPAT_GRAB_MODS = 0x00000400;
        const LOOKUP_MODS = 0x00000800;
        const COMPAT_LOOKUP_MODS = 0x00001000;
        const POINTER_BUTTONS = 0x00002000;
    }
}

bitflags! {
    pub struct BoolCtrl: u32 {
        const REPEAT_KEYS = 0x00000001;
        const SLOW_KEYS = 0x00000002;
        const BOUNCE_KEYS = 0x00000004;
        const STICKY_KEYS = 0x00000008;
        const MOUSE_KEYS = 0x00000010;
        const MOUSE_KEYS_ACCEL = 0x00000020;
        const ACCESS_X_KEYS = 0x00000040;
        const ACCESS_X_TIMEOUT_MASK = 0x00000080;
        const ACCESS_X_FEEDBACK_MASK = 0x00000100;
        const AUDIBLE_BELL_MASK = 0x00000200;
        const OVERLAY1_MASK = 0x00000400;
        const OVERLAY2_MASK = 0x00000800;
        const IGNORE_GROUP_LOCK_MASK = 0x00001000;
    }
}

bitflags! {
    pub struct Control: u32 {
        const GROUPS_WRAP = 0x08000000;
        const INTERNAL_MODS = 0x10000000;
        const IGNORE_LOCK_MODS = 0x20000000;
        const PER_KEY_REPEAT = 0x40000000;
        const CONTROLS_ENABLED = 0x80000000;
    }
}

bitflags! {
    pub struct AxOption: u32 {
        const SK_PRESS_FB = 0x00000001;
        const SK_ACCEPT_FB = 0x00000002;
        const FEATURE_FB = 0x00000004;
        const SLOW_WARN_FB = 0x00000008;
        const INDICATOR_FB = 0x00000010;
        const STICKY_KEYS_FB = 0x00000020;
        const TWO_KEYS = 0x00000040;
        const LATCH_TO_LOCK = 0x00000080;
        const SK_RELEASE_FB = 0x00000100;
        const SK_REJECT_FB = 0x00000200;
        const BK_REJECT_FB = 0x00000400;
        const DUMB_BELL = 0x00000800;
    }
}

pub type DeviceSpec = u16;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LedClassResult {
    KbdFeedbackClass = 0,
    LedFeedbackClass = 4,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LedClass {
    KbdFeedbackClass = 0,
    LedFeedbackClass = 4,
    DfltXiClass = 768,
    AllXiClasses = 1280,
}

pub type LedClassSpec = u16;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum BellClassResult {
    KbdFeedbackClass = 0,
    BellFeedbackClass = 5,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum BellClass {
    KbdFeedbackClass = 0,
    BellFeedbackClass = 5,
    DfltXiClass = 768,
}

pub type BellClassSpec = u16;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Id {
    UseCoreKbd = 256,
    UseCorePtr = 512,
    DfltXiClass = 768,
    DfltXiId = 1024,
    AllXiClass = 1280,
    AllXiId = 1536,
    XiNone = 65280,
}

pub type IdSpec = u16;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Group {
    N1 = 0,
    N2 = 1,
    N3 = 2,
    N4 = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Groups {
    Any = 254,
    All = 255,
}

bitflags! {
    pub struct SetOfGroup: u32 {
        const GROUP1 = 0x00000001;
        const GROUP2 = 0x00000002;
        const GROUP3 = 0x00000004;
        const GROUP4 = 0x00000008;
    }
}

bitflags! {
    pub struct SetOfGroups: u32 {
        const ANY = 0x00000080;
    }
}

bitflags! {
    pub struct GroupsWrap: u32 {
        const WRAP_INTO_RANGE = 0x00000000;
        const CLAMP_INTO_RANGE = 0x00000040;
        const REDIRECT_INTO_RANGE = 0x00000080;
    }
}

bitflags! {
    pub struct VModsHigh: u32 {
        const N15 = 0x00000080;
        const N14 = 0x00000040;
        const N13 = 0x00000020;
        const N12 = 0x00000010;
        const N11 = 0x00000008;
        const N10 = 0x00000004;
        const N9 = 0x00000002;
        const N8 = 0x00000001;
    }
}

bitflags! {
    pub struct VModsLow: u32 {
        const N7 = 0x00000080;
        const N6 = 0x00000040;
        const N5 = 0x00000020;
        const N4 = 0x00000010;
        const N3 = 0x00000008;
        const N2 = 0x00000004;
        const N1 = 0x00000002;
        const N0 = 0x00000001;
    }
}

bitflags! {
    pub struct VMod: u32 {
        const N15 = 0x00008000;
        const N14 = 0x00004000;
        const N13 = 0x00002000;
        const N12 = 0x00001000;
        const N11 = 0x00000800;
        const N10 = 0x00000400;
        const N9 = 0x00000200;
        const N8 = 0x00000100;
        const N7 = 0x00000080;
        const N6 = 0x00000040;
        const N5 = 0x00000020;
        const N4 = 0x00000010;
        const N3 = 0x00000008;
        const N2 = 0x00000004;
        const N1 = 0x00000002;
        const N0 = 0x00000001;
    }
}

bitflags! {
    pub struct Explicit: u32 {
        const V_MOD_MAP = 0x00000080;
        const BEHAVIOR = 0x00000040;
        const AUTO_REPEAT = 0x00000020;
        const INTERPRET = 0x00000010;
        const KEY_TYPE4 = 0x00000008;
        const KEY_TYPE3 = 0x00000004;
        const KEY_TYPE2 = 0x00000002;
        const KEY_TYPE1 = 0x00000001;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SymInterpretMatch {
    NoneOf = 0,
    AnyOfOrNone = 1,
    AnyOf = 2,
    AllOf = 3,
    Exactly = 4,
}

bitflags! {
    pub struct SymInterpMatch: u32 {
        const LEVEL_ONE_ONLY = 0x00000080;
        const OP_MASK = 0x0000007f;
    }
}

bitflags! {
    pub struct ImFlag: u32 {
        const NO_EXPLICIT = 0x00000080;
        const NO_AUTOMATIC = 0x00000040;
        const LED_DRIVES_KB = 0x00000020;
    }
}

bitflags! {
    pub struct ImModsWhich: u32 {
        const USE_COMPAT = 0x00000010;
        const USE_EFFECTIVE = 0x00000008;
        const USE_LOCKED = 0x00000004;
        const USE_LATCHED = 0x00000002;
        const USE_BASE = 0x00000001;
    }
}

bitflags! {
    pub struct ImGroupsWhich: u32 {
        const USE_COMPAT = 0x00000010;
        const USE_EFFECTIVE = 0x00000008;
        const USE_LOCKED = 0x00000004;
        const USE_LATCHED = 0x00000002;
        const USE_BASE = 0x00000001;
    }
}

#[derive(Copy, Clone)]
pub struct IndicatorMap {
    data: [u8; 12],
}

#[allow(unused_parens)]
impl IndicatorMap {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &IndicatorMap {
        debug_assert_eq!(data.as_ref().len(), 12);
        &*(data.as_ref() as *const [u8] as *const IndicatorMap)
    }

    /// Construct a new [IndicatorMap].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        flags: ImFlag,
        which_groups: ImGroupsWhich,
        groups: SetOfGroup,
        which_mods: ImModsWhich,
        mods: xproto::ModMask,
        real_mods: xproto::ModMask,
        vmods: VMod,
        ctrls: BoolCtrl,
    ) -> IndicatorMap {
            unsafe {
            let mut wire_buf = [0u8; 12];
            let mut wire_off = 0usize;

            wire_off += (std::mem::transmute::<_, u32>(flags) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(which_groups) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(groups) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(which_mods) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (mods.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (real_mods.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (vmods.bits() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += ctrls.bits().serialize(&mut wire_buf[wire_off .. ]);

            IndicatorMap { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn flags(&self) -> ImFlag {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, ImFlag>(val)
        }
    }

    pub fn which_groups(&self) -> ImGroupsWhich {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, ImGroupsWhich>(val)
        }
    }

    pub fn groups(&self) -> SetOfGroup {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, SetOfGroup>(val)
        }
    }

    pub fn which_mods(&self) -> ImModsWhich {
        unsafe {
            let offset = 3usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, ImModsWhich>(val)
        }
    }

    pub fn mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn real_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 5usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn vmods(&self) -> VMod {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

    pub fn ctrls(&self) -> BoolCtrl {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BoolCtrl>(val)
        }
    }
}

#[test]
fn test_sizeof_indicator_map() {
    assert_eq!(std::mem::size_of::<IndicatorMap>(), 12);
}

impl base::WiredOut for IndicatorMap {
    fn wire_len(&self) -> usize { 12 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for IndicatorMap {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 12 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 12;
        *(ptr as *const IndicatorMap)
    }
}

impl std::fmt::Debug for IndicatorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndicatorMap")
            .field("flags", &self.flags())
            .field("which_groups", &self.which_groups())
            .field("groups", &self.groups())
            .field("which_mods", &self.which_mods())
            .field("mods", &self.mods())
            .field("real_mods", &self.real_mods())
            .field("vmods", &self.vmods())
            .field("ctrls", &self.ctrls())
            .finish()
    }
}

bitflags! {
    pub struct CmDetail: u32 {
        const SYM_INTERP = 0x00000001;
        const GROUP_COMPAT = 0x00000002;
    }
}

bitflags! {
    pub struct NameDetail: u32 {
        const KEYCODES = 0x00000001;
        const GEOMETRY = 0x00000002;
        const SYMBOLS = 0x00000004;
        const PHYS_SYMBOLS = 0x00000008;
        const TYPES = 0x00000010;
        const COMPAT = 0x00000020;
        const KEY_TYPE_NAMES = 0x00000040;
        const KT_LEVEL_NAMES = 0x00000080;
        const INDICATOR_NAMES = 0x00000100;
        const KEY_NAMES = 0x00000200;
        const KEY_ALIASES = 0x00000400;
        const VIRTUAL_MOD_NAMES = 0x00000800;
        const GROUP_NAMES = 0x00001000;
        const RG_NAMES = 0x00002000;
    }
}

bitflags! {
    pub struct GbnDetail: u32 {
        const TYPES = 0x00000001;
        const COMPAT_MAP = 0x00000002;
        const CLIENT_SYMBOLS = 0x00000004;
        const SERVER_SYMBOLS = 0x00000008;
        const INDICATOR_MAPS = 0x00000010;
        const KEY_NAMES = 0x00000020;
        const GEOMETRY = 0x00000040;
        const OTHER_NAMES = 0x00000080;
    }
}

bitflags! {
    pub struct XiFeature: u32 {
        const KEYBOARDS = 0x00000001;
        const BUTTON_ACTIONS = 0x00000002;
        const INDICATOR_NAMES = 0x00000004;
        const INDICATOR_MAPS = 0x00000008;
        const INDICATOR_STATE = 0x00000010;
    }
}

bitflags! {
    pub struct PerClientFlag: u32 {
        const DETECTABLE_AUTO_REPEAT = 0x00000001;
        const GRABS_USE_XKB_STATE = 0x00000002;
        const AUTO_RESET_CONTROLS = 0x00000004;
        const LOOKUP_STATE_WHEN_GRABBED = 0x00000008;
        const SEND_EVENT_USES_XKB_STATE = 0x00000010;
    }
}

#[derive(Copy, Clone)]
pub struct ModDef {
    data: [u8; 4],
}

#[allow(unused_parens)]
impl ModDef {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &ModDef {
        debug_assert_eq!(data.as_ref().len(), 4);
        &*(data.as_ref() as *const [u8] as *const ModDef)
    }

    /// Construct a new [ModDef].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        mask: xproto::ModMask,
        real_mods: xproto::ModMask,
        vmods: VMod,
    ) -> ModDef {
            unsafe {
            let mut wire_buf = [0u8; 4];
            let mut wire_off = 0usize;

            wire_off += (mask.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (real_mods.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (vmods.bits() as u16).serialize(&mut wire_buf[wire_off .. ]);

            ModDef { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn mask(&self) -> xproto::ModMask {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn real_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn vmods(&self) -> VMod {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }
}

#[test]
fn test_sizeof_mod_def() {
    assert_eq!(std::mem::size_of::<ModDef>(), 4);
}

impl base::WiredOut for ModDef {
    fn wire_len(&self) -> usize { 4 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for ModDef {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 4 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 4;
        *(ptr as *const ModDef)
    }
}

impl std::fmt::Debug for ModDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModDef")
            .field("mask", &self.mask())
            .field("real_mods", &self.real_mods())
            .field("vmods", &self.vmods())
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct KeyName {
    pub name: Lat1StrF<4>,
}

#[test]
fn test_sizeof_key_name() {
    assert_eq!(std::mem::size_of::<KeyName>(), 4);
}

impl base::WiredOut for KeyName {
    fn wire_len(&self) -> usize { 4 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const KeyName as _, 4)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        4
    }
}

impl base::WiredIn for KeyName {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 4 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 4;
        *(ptr as *const KeyName)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct KeyAlias {
    pub real: Lat1StrF<4>,
    pub alias: Lat1StrF<4>,
}

#[test]
fn test_sizeof_key_alias() {
    assert_eq!(std::mem::size_of::<KeyAlias>(), 8);
}

impl base::WiredOut for KeyAlias {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const KeyAlias as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for KeyAlias {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const KeyAlias)
    }
}

pub struct CountedString16 {
    data: [u8],
}

#[allow(unused_parens)]
impl CountedString16 {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &CountedString16 {
        debug_assert_eq!(data.as_ref().len(), <&CountedString16 as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const CountedString16)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn length(&self) -> u16 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }


    pub fn string(&self) -> &Lat1Str {
        unsafe {
            let offset = 2usize;
            let len = (self.length() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }

    pub fn alignment_pad(&self) -> &[u8] {
        unsafe {
            let offset = (2usize + (self.length() as usize));
            let len = ((((self.length() as usize) + 5usize) & (!3usize)) - ((self.length() as usize) + 2usize));
            let ptr = self.wire_ptr().add(offset) as *const u8;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for CountedString16 {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &CountedString16 {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // length
        let length = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // string
        sz += (length as usize);
        // alignment_pad
        sz += ((((length as usize) + 5usize) & (!3usize)) - ((length as usize) + 2usize));
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        CountedString16::from_data(data)
    }
}

#[derive(Clone)]
pub struct CountedString16Buf {
    data: Vec<u8>,
}

impl CountedString16Buf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> CountedString16Buf {
        debug_assert_eq!(<&CountedString16>::compute_wire_len(data.as_ptr(), ()), data.len());
        CountedString16Buf { data }
    }

    /// Construct a new [CountedString16Buf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        string: &[u8],
        alignment_pad: &[u8],
    ) -> CountedString16Buf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 2; // length
            wire_sz += string.len();
            wire_sz += alignment_pad.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += (string.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_buf[wire_off .. wire_off + string.len()].copy_from_slice(string);
            wire_off += string.len();
            for el in alignment_pad {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            CountedString16Buf { data: wire_buf }
        }
    }
}

impl base::WiredIn for CountedString16Buf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&CountedString16>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        CountedString16Buf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for CountedString16Buf {
    type Target = CountedString16;
    fn deref(&self) -> &Self::Target {
        unsafe { CountedString16::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<CountedString16> for CountedString16Buf {
    fn borrow(&self) -> &CountedString16 {
        unsafe { CountedString16::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for CountedString16 {
    type Owned = CountedString16Buf;
    fn to_owned(&self) -> Self::Owned {
        CountedString16Buf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for CountedString16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CountedString16")
            .field("length", &self.length())
            .field("string", &self.string())
            .field("alignment_pad", &self.alignment_pad())
            .finish()
    }
}

impl std::fmt::Debug for CountedString16Buf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CountedString16Buf")
            .field("length", &self.length())
            .field("string", &self.string())
            .field("alignment_pad", &self.alignment_pad())
            .finish()
    }
}

#[derive(Clone)]
pub struct CountedString16Iterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a CountedString16>,
}

impl<'a> Iterator for CountedString16Iterator<'a> {
    type Item = &'a CountedString16;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&CountedString16>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for CountedString16Iterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Copy, Clone)]
pub struct KtMapEntry {
    data: [u8; 8],
}

#[allow(unused_parens)]
impl KtMapEntry {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &KtMapEntry {
        debug_assert_eq!(data.as_ref().len(), 8);
        &*(data.as_ref() as *const [u8] as *const KtMapEntry)
    }

    /// Construct a new [KtMapEntry].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        active: bool,
        mods_mask: xproto::ModMask,
        level: u8,
        mods_mods: xproto::ModMask,
        mods_vmods: VMod,
    ) -> KtMapEntry {
            unsafe {
            let mut wire_buf = [0u8; 8];
            let mut wire_off = 0usize;

            wire_off += (if active { 1u8 } else { 0u8 }).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (mods_mask.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += level.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (mods_mods.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (mods_vmods.bits() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 2; // pad

            KtMapEntry { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn active(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(0usize)) };
        val != 0
    }

    pub fn mods_mask(&self) -> xproto::ModMask {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn level(&self) -> u8 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn mods_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 3usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn mods_vmods(&self) -> VMod {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

}

#[test]
fn test_sizeof_kt_map_entry() {
    assert_eq!(std::mem::size_of::<KtMapEntry>(), 8);
}

impl base::WiredOut for KtMapEntry {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for KtMapEntry {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const KtMapEntry)
    }
}

impl std::fmt::Debug for KtMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KtMapEntry")
            .field("active", &self.active())
            .field("mods_mask", &self.mods_mask())
            .field("level", &self.level())
            .field("mods_mods", &self.mods_mods())
            .field("mods_vmods", &self.mods_vmods())
            .field("pad", &2)
            .finish()
    }
}

pub struct KeyType {
    data: [u8],
}

#[allow(unused_parens)]
impl KeyType {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &KeyType {
        debug_assert_eq!(data.as_ref().len(), <&KeyType as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const KeyType)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn mods_mask(&self) -> xproto::ModMask {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn mods_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn mods_vmods(&self) -> VMod {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

    pub fn num_levels(&self) -> u8 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn n_map_entries(&self) -> u8 {
        unsafe {
            let offset = 5usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn has_preserve(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(6usize)) };
        val != 0
    }


    pub fn map(&self) -> &[KtMapEntry] {
        unsafe {
            let offset = 8usize;
            let len = (self.n_map_entries() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const KtMapEntry;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn preserve(&self) -> &[ModDef] {
        unsafe {
            let offset = (8usize + ((self.n_map_entries() as usize) * 8usize));
            let len = ((self.has_preserve() as usize) * (self.n_map_entries() as usize)) as _;
            let ptr = self.wire_ptr().add(offset) as *const ModDef;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for KeyType {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &KeyType {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // mods_mask
        sz += 1usize;
        // mods_mods
        sz += 1usize;
        // mods_vmods
        sz += 2usize;
        // num_levels
        sz += 1usize;
        // n_map_entries
        let n_map_entries = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // has_preserve
        let has_preserve = *(ptr.add(sz) as *const bool);
        sz += 1usize;
        // pad
        sz += 1usize;
        // map
        sz += ((n_map_entries as usize) * 8usize);
        // preserve
        sz += (((has_preserve as usize) * (n_map_entries as usize)) * 4usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        KeyType::from_data(data)
    }
}

#[derive(Clone)]
pub struct KeyTypeBuf {
    data: Vec<u8>,
}

impl KeyTypeBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> KeyTypeBuf {
        debug_assert_eq!(<&KeyType>::compute_wire_len(data.as_ptr(), ()), data.len());
        KeyTypeBuf { data }
    }

    /// Construct a new [KeyTypeBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        mods_mask: xproto::ModMask,
        mods_mods: xproto::ModMask,
        mods_vmods: VMod,
        num_levels: u8,
        has_preserve: bool,
        map: &[KtMapEntry],
        preserve: &[ModDef],
    ) -> KeyTypeBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // mods_mask
            wire_sz += 1; // mods_mods
            wire_sz += 2; // mods_vmods
            wire_sz += 1; // num_levels
            wire_sz += 1; // n_map_entries
            wire_sz += 1; // has_preserve
            wire_sz += 1; // pad
            wire_sz += map.iter().map(|el| el.wire_len()).sum::<usize>();
            wire_sz += preserve.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += (mods_mask.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (mods_mods.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (mods_vmods.bits() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += num_levels.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (map.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (if has_preserve { 1u8 } else { 0u8 }).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad
            for el in map {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }
            for el in preserve {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            KeyTypeBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for KeyTypeBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&KeyType>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        KeyTypeBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for KeyTypeBuf {
    type Target = KeyType;
    fn deref(&self) -> &Self::Target {
        unsafe { KeyType::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<KeyType> for KeyTypeBuf {
    fn borrow(&self) -> &KeyType {
        unsafe { KeyType::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for KeyType {
    type Owned = KeyTypeBuf;
    fn to_owned(&self) -> Self::Owned {
        KeyTypeBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyType")
            .field("mods_mask", &self.mods_mask())
            .field("mods_mods", &self.mods_mods())
            .field("mods_vmods", &self.mods_vmods())
            .field("num_levels", &self.num_levels())
            .field("n_map_entries", &self.n_map_entries())
            .field("has_preserve", &self.has_preserve())
            .field("pad", &1)
            .field("map", &self.map())
            .field("preserve", &self.preserve())
            .finish()
    }
}

impl std::fmt::Debug for KeyTypeBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyTypeBuf")
            .field("mods_mask", &self.mods_mask())
            .field("mods_mods", &self.mods_mods())
            .field("mods_vmods", &self.mods_vmods())
            .field("num_levels", &self.num_levels())
            .field("n_map_entries", &self.n_map_entries())
            .field("has_preserve", &self.has_preserve())
            .field("pad", &1)
            .field("map", &self.map())
            .field("preserve", &self.preserve())
            .finish()
    }
}

#[derive(Clone)]
pub struct KeyTypeIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a KeyType>,
}

impl<'a> Iterator for KeyTypeIterator<'a> {
    type Item = &'a KeyType;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&KeyType>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for KeyTypeIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct KeySymMap {
    data: [u8],
}

#[allow(unused_parens)]
impl KeySymMap {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &KeySymMap {
        debug_assert_eq!(data.as_ref().len(), <&KeySymMap as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const KeySymMap)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn kt_index(&self) -> &[u8; 4] {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const [u8; 4];
            &*ptr
        }
    }

    pub fn group_info(&self) -> u8 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn width(&self) -> u8 {
        unsafe {
            let offset = 5usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn n_syms(&self) -> u16 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn syms(&self) -> &[xproto::Keysym] {
        unsafe {
            let offset = 8usize;
            let len = (self.n_syms() as usize);
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keysym;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for KeySymMap {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &KeySymMap {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // kt_index
        sz += 4usize;
        // group_info
        sz += 1usize;
        // width
        sz += 1usize;
        // n_syms
        let n_syms = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // syms
        sz += ((n_syms as usize) * 4usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        KeySymMap::from_data(data)
    }
}

#[derive(Clone)]
pub struct KeySymMapBuf {
    data: Vec<u8>,
}

impl KeySymMapBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> KeySymMapBuf {
        debug_assert_eq!(<&KeySymMap>::compute_wire_len(data.as_ptr(), ()), data.len());
        KeySymMapBuf { data }
    }

    /// Construct a new [KeySymMapBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        kt_index: &[u8; 4],
        group_info: u8,
        width: u8,
        syms: &[xproto::Keysym],
    ) -> KeySymMapBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += kt_index.iter().map(|el| el.wire_len()).sum::<usize>();
            wire_sz += 1; // group_info
            wire_sz += 1; // width
            wire_sz += 2; // n_syms
            wire_sz += syms.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            for el in kt_index {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }
            wire_off += group_info.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += width.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (syms.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            for el in syms {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            KeySymMapBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for KeySymMapBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&KeySymMap>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        KeySymMapBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for KeySymMapBuf {
    type Target = KeySymMap;
    fn deref(&self) -> &Self::Target {
        unsafe { KeySymMap::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<KeySymMap> for KeySymMapBuf {
    fn borrow(&self) -> &KeySymMap {
        unsafe { KeySymMap::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for KeySymMap {
    type Owned = KeySymMapBuf;
    fn to_owned(&self) -> Self::Owned {
        KeySymMapBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for KeySymMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeySymMap")
            .field("kt_index", &self.kt_index())
            .field("group_info", &self.group_info())
            .field("width", &self.width())
            .field("n_syms", &self.n_syms())
            .field("syms", &self.syms())
            .finish()
    }
}

impl std::fmt::Debug for KeySymMapBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeySymMapBuf")
            .field("kt_index", &self.kt_index())
            .field("group_info", &self.group_info())
            .field("width", &self.width())
            .field("n_syms", &self.n_syms())
            .field("syms", &self.syms())
            .finish()
    }
}

#[derive(Clone)]
pub struct KeySymMapIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a KeySymMap>,
}

impl<'a> Iterator for KeySymMapIterator<'a> {
    type Item = &'a KeySymMap;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&KeySymMap>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for KeySymMapIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Clone, Debug)]
pub enum Behavior {
    Default{
    },
    Lock{
    },
    RadioGroup{
        group: u8,
    },
    Overlay1{
        key: xproto::Keycode,
    },
    Overlay2{
        key: xproto::Keycode,
    },
    PermamentLock{
    },
    PermamentRadioGroup{
        group: u8,
    },
    PermamentOverlay1{
        key: xproto::Keycode,
    },
    PermamentOverlay2{
        key: xproto::Keycode,
    },
}

impl Behavior {
    unsafe fn from_data(wire_data: &[u8]) -> Behavior {
        let r#type = wire_data[0] as u32;
        match r#type {
            0 => Behavior::Default{
            },
            1 => Behavior::Lock{
            },
            2 => Behavior::RadioGroup{
                group: *(wire_data.as_ptr().add(1) as *const u8),
            },
            3 => Behavior::Overlay1{
                key: *(wire_data.as_ptr().add(1) as *const xproto::Keycode),
            },
            4 => Behavior::Overlay2{
                key: *(wire_data.as_ptr().add(1) as *const xproto::Keycode),
            },
            129 => Behavior::PermamentLock{
            },
            130 => Behavior::PermamentRadioGroup{
                group: *(wire_data.as_ptr().add(1) as *const u8),
            },
            131 => Behavior::PermamentOverlay1{
                key: *(wire_data.as_ptr().add(1) as *const xproto::Keycode),
            },
            132 => Behavior::PermamentOverlay2{
                key: *(wire_data.as_ptr().add(1) as *const xproto::Keycode),
            },
            _ => unreachable!("unexpected type value for xkb::Behavior"),
        }
    }
}

impl base::WiredOut for Behavior {
    fn wire_len(&self) -> usize { 2 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        match self {
            Behavior::Default{
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(BehaviorType::Default) } as u8;
            }
            Behavior::Lock{
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(BehaviorType::Lock) } as u8;
            }
            Behavior::RadioGroup{
                group,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(BehaviorType::RadioGroup) } as u8;
                group.serialize(&mut wire_buf[1..]);
            }
            Behavior::Overlay1{
                key,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(BehaviorType::Overlay1) } as u8;
                key.serialize(&mut wire_buf[1..]);
            }
            Behavior::Overlay2{
                key,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(BehaviorType::Overlay2) } as u8;
                key.serialize(&mut wire_buf[1..]);
            }
            Behavior::PermamentLock{
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(BehaviorType::PermamentLock) } as u8;
            }
            Behavior::PermamentRadioGroup{
                group,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(BehaviorType::PermamentRadioGroup) } as u8;
                group.serialize(&mut wire_buf[1..]);
            }
            Behavior::PermamentOverlay1{
                key,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(BehaviorType::PermamentOverlay1) } as u8;
                key.serialize(&mut wire_buf[1..]);
            }
            Behavior::PermamentOverlay2{
                key,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(BehaviorType::PermamentOverlay2) } as u8;
                key.serialize(&mut wire_buf[1..]);
            }
        }
        2
    }
}

impl base::WiredIn for Behavior {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: Self::Params) -> usize { 2 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        Self::from_data(std::slice::from_raw_parts(ptr, sz))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum BehaviorType {
    Default = 0,
    Lock = 1,
    RadioGroup = 2,
    Overlay1 = 3,
    Overlay2 = 4,
    PermamentLock = 129,
    PermamentRadioGroup = 130,
    PermamentOverlay1 = 131,
    PermamentOverlay2 = 132,
}

#[derive(Copy, Clone)]
pub struct SetBehavior {
    data: [u8; 4],
}

#[allow(unused_parens)]
impl SetBehavior {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &SetBehavior {
        debug_assert_eq!(data.as_ref().len(), 4);
        &*(data.as_ref() as *const [u8] as *const SetBehavior)
    }

    /// Construct a new [SetBehavior].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        keycode: xproto::Keycode,
        behavior: Behavior,
    ) -> SetBehavior {
            unsafe {
            let mut wire_buf = [0u8; 4];
            let mut wire_off = 0usize;

            wire_off += keycode.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += behavior.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad

            SetBehavior { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn behavior(&self) -> Behavior {
        unsafe {
            let mut offset = 1usize;
            Behavior::unserialize(self.wire_ptr().add(offset), (), &mut offset)
        }
    }

}

#[test]
fn test_sizeof_set_behavior() {
    assert_eq!(std::mem::size_of::<SetBehavior>(), 4);
}

impl base::WiredOut for SetBehavior {
    fn wire_len(&self) -> usize { 4 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for SetBehavior {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 4 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 4;
        *(ptr as *const SetBehavior)
    }
}

impl std::fmt::Debug for SetBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetBehavior")
            .field("keycode", &self.keycode())
            .field("behavior", &self.behavior())
            .field("pad", &1)
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct SetExplicit {
    data: [u8; 2],
}

#[allow(unused_parens)]
impl SetExplicit {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &SetExplicit {
        debug_assert_eq!(data.as_ref().len(), 2);
        &*(data.as_ref() as *const [u8] as *const SetExplicit)
    }

    /// Construct a new [SetExplicit].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        keycode: xproto::Keycode,
        explicit: Explicit,
    ) -> SetExplicit {
            unsafe {
            let mut wire_buf = [0u8; 2];
            let mut wire_off = 0usize;

            wire_off += keycode.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (explicit.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);

            SetExplicit { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn explicit(&self) -> Explicit {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, Explicit>(val)
        }
    }
}

#[test]
fn test_sizeof_set_explicit() {
    assert_eq!(std::mem::size_of::<SetExplicit>(), 2);
}

impl base::WiredOut for SetExplicit {
    fn wire_len(&self) -> usize { 2 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for SetExplicit {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 2 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 2;
        *(ptr as *const SetExplicit)
    }
}

impl std::fmt::Debug for SetExplicit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetExplicit")
            .field("keycode", &self.keycode())
            .field("explicit", &self.explicit())
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct KeyModMap {
    data: [u8; 2],
}

#[allow(unused_parens)]
impl KeyModMap {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &KeyModMap {
        debug_assert_eq!(data.as_ref().len(), 2);
        &*(data.as_ref() as *const [u8] as *const KeyModMap)
    }

    /// Construct a new [KeyModMap].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        keycode: xproto::Keycode,
        mods: xproto::ModMask,
    ) -> KeyModMap {
            unsafe {
            let mut wire_buf = [0u8; 2];
            let mut wire_off = 0usize;

            wire_off += keycode.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (mods.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);

            KeyModMap { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }
}

#[test]
fn test_sizeof_key_mod_map() {
    assert_eq!(std::mem::size_of::<KeyModMap>(), 2);
}

impl base::WiredOut for KeyModMap {
    fn wire_len(&self) -> usize { 2 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for KeyModMap {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 2 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 2;
        *(ptr as *const KeyModMap)
    }
}

impl std::fmt::Debug for KeyModMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyModMap")
            .field("keycode", &self.keycode())
            .field("mods", &self.mods())
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct KeyVModMap {
    data: [u8; 4],
}

#[allow(unused_parens)]
impl KeyVModMap {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &KeyVModMap {
        debug_assert_eq!(data.as_ref().len(), 4);
        &*(data.as_ref() as *const [u8] as *const KeyVModMap)
    }

    /// Construct a new [KeyVModMap].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        keycode: xproto::Keycode,
        vmods: VMod,
    ) -> KeyVModMap {
            unsafe {
            let mut wire_buf = [0u8; 4];
            let mut wire_off = 0usize;

            wire_off += keycode.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad
            wire_off += (vmods.bits() as u16).serialize(&mut wire_buf[wire_off .. ]);

            KeyVModMap { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn keycode(&self) -> xproto::Keycode {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }


    pub fn vmods(&self) -> VMod {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }
}

#[test]
fn test_sizeof_key_v_mod_map() {
    assert_eq!(std::mem::size_of::<KeyVModMap>(), 4);
}

impl base::WiredOut for KeyVModMap {
    fn wire_len(&self) -> usize { 4 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for KeyVModMap {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 4 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 4;
        *(ptr as *const KeyVModMap)
    }
}

impl std::fmt::Debug for KeyVModMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyVModMap")
            .field("keycode", &self.keycode())
            .field("pad", &1)
            .field("vmods", &self.vmods())
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct KtSetMapEntry {
    data: [u8; 4],
}

#[allow(unused_parens)]
impl KtSetMapEntry {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &KtSetMapEntry {
        debug_assert_eq!(data.as_ref().len(), 4);
        &*(data.as_ref() as *const [u8] as *const KtSetMapEntry)
    }

    /// Construct a new [KtSetMapEntry].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        level: u8,
        real_mods: xproto::ModMask,
        virtual_mods: VMod,
    ) -> KtSetMapEntry {
            unsafe {
            let mut wire_buf = [0u8; 4];
            let mut wire_off = 0usize;

            wire_off += level.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (real_mods.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (virtual_mods.bits() as u16).serialize(&mut wire_buf[wire_off .. ]);

            KtSetMapEntry { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn level(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn real_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn virtual_mods(&self) -> VMod {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }
}

#[test]
fn test_sizeof_kt_set_map_entry() {
    assert_eq!(std::mem::size_of::<KtSetMapEntry>(), 4);
}

impl base::WiredOut for KtSetMapEntry {
    fn wire_len(&self) -> usize { 4 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for KtSetMapEntry {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 4 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 4;
        *(ptr as *const KtSetMapEntry)
    }
}

impl std::fmt::Debug for KtSetMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KtSetMapEntry")
            .field("level", &self.level())
            .field("real_mods", &self.real_mods())
            .field("virtual_mods", &self.virtual_mods())
            .finish()
    }
}

pub struct SetKeyType {
    data: [u8],
}

#[allow(unused_parens)]
impl SetKeyType {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &SetKeyType {
        debug_assert_eq!(data.as_ref().len(), <&SetKeyType as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const SetKeyType)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn mask(&self) -> xproto::ModMask {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn real_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn virtual_mods(&self) -> VMod {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

    pub fn num_levels(&self) -> u8 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn n_map_entries(&self) -> u8 {
        unsafe {
            let offset = 5usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn preserve(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(6usize)) };
        val != 0
    }


    pub fn entries(&self) -> &[KtSetMapEntry] {
        unsafe {
            let offset = 8usize;
            let len = (self.n_map_entries() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const KtSetMapEntry;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn preserve_entries(&self) -> &[KtSetMapEntry] {
        unsafe {
            let offset = (8usize + ((self.n_map_entries() as usize) * 4usize));
            let len = ((self.preserve() as usize) * (self.n_map_entries() as usize)) as _;
            let ptr = self.wire_ptr().add(offset) as *const KtSetMapEntry;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for SetKeyType {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &SetKeyType {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // mask
        sz += 1usize;
        // real_mods
        sz += 1usize;
        // virtual_mods
        sz += 2usize;
        // num_levels
        sz += 1usize;
        // n_map_entries
        let n_map_entries = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // preserve
        let preserve = *(ptr.add(sz) as *const bool);
        sz += 1usize;
        // pad
        sz += 1usize;
        // entries
        sz += ((n_map_entries as usize) * 4usize);
        // preserve_entries
        sz += (((preserve as usize) * (n_map_entries as usize)) * 4usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        SetKeyType::from_data(data)
    }
}

#[derive(Clone)]
pub struct SetKeyTypeBuf {
    data: Vec<u8>,
}

impl SetKeyTypeBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> SetKeyTypeBuf {
        debug_assert_eq!(<&SetKeyType>::compute_wire_len(data.as_ptr(), ()), data.len());
        SetKeyTypeBuf { data }
    }

    /// Construct a new [SetKeyTypeBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        mask: xproto::ModMask,
        real_mods: xproto::ModMask,
        virtual_mods: VMod,
        num_levels: u8,
        preserve: bool,
        entries: &[KtSetMapEntry],
        preserve_entries: &[KtSetMapEntry],
    ) -> SetKeyTypeBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // mask
            wire_sz += 1; // real_mods
            wire_sz += 2; // virtual_mods
            wire_sz += 1; // num_levels
            wire_sz += 1; // n_map_entries
            wire_sz += 1; // preserve
            wire_sz += 1; // pad
            wire_sz += entries.iter().map(|el| el.wire_len()).sum::<usize>();
            wire_sz += preserve_entries.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += (mask.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (real_mods.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (virtual_mods.bits() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += num_levels.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (entries.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (if preserve { 1u8 } else { 0u8 }).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad
            for el in entries {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }
            for el in preserve_entries {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            SetKeyTypeBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for SetKeyTypeBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&SetKeyType>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        SetKeyTypeBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for SetKeyTypeBuf {
    type Target = SetKeyType;
    fn deref(&self) -> &Self::Target {
        unsafe { SetKeyType::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<SetKeyType> for SetKeyTypeBuf {
    fn borrow(&self) -> &SetKeyType {
        unsafe { SetKeyType::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for SetKeyType {
    type Owned = SetKeyTypeBuf;
    fn to_owned(&self) -> Self::Owned {
        SetKeyTypeBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for SetKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetKeyType")
            .field("mask", &self.mask())
            .field("real_mods", &self.real_mods())
            .field("virtual_mods", &self.virtual_mods())
            .field("num_levels", &self.num_levels())
            .field("n_map_entries", &self.n_map_entries())
            .field("preserve", &self.preserve())
            .field("pad", &1)
            .field("entries", &self.entries())
            .field("preserve_entries", &self.preserve_entries())
            .finish()
    }
}

impl std::fmt::Debug for SetKeyTypeBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetKeyTypeBuf")
            .field("mask", &self.mask())
            .field("real_mods", &self.real_mods())
            .field("virtual_mods", &self.virtual_mods())
            .field("num_levels", &self.num_levels())
            .field("n_map_entries", &self.n_map_entries())
            .field("preserve", &self.preserve())
            .field("pad", &1)
            .field("entries", &self.entries())
            .field("preserve_entries", &self.preserve_entries())
            .finish()
    }
}

#[derive(Clone)]
pub struct SetKeyTypeIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a SetKeyType>,
}

impl<'a> Iterator for SetKeyTypeIterator<'a> {
    type Item = &'a SetKeyType;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&SetKeyType>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for SetKeyTypeIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub type String8 = u8;

pub struct Outline {
    data: [u8],
}

#[allow(unused_parens)]
impl Outline {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Outline {
        debug_assert_eq!(data.as_ref().len(), <&Outline as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Outline)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn n_points(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn corner_radius(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn points(&self) -> &[xproto::Point] {
        unsafe {
            let offset = 4usize;
            let len = (self.n_points() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Point;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for Outline {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Outline {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // n_points
        let n_points = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // corner_radius
        sz += 1usize;
        // pad
        sz += 2usize;
        // points
        sz += ((n_points as usize) * 4usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Outline::from_data(data)
    }
}

#[derive(Clone)]
pub struct OutlineBuf {
    data: Vec<u8>,
}

impl OutlineBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> OutlineBuf {
        debug_assert_eq!(<&Outline>::compute_wire_len(data.as_ptr(), ()), data.len());
        OutlineBuf { data }
    }

    /// Construct a new [OutlineBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        corner_radius: u8,
        points: &[xproto::Point],
    ) -> OutlineBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // n_points
            wire_sz += 1; // corner_radius
            wire_sz += 2; // pad
            wire_sz += points.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += (points.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += corner_radius.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 2; // pad
            for el in points {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            OutlineBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for OutlineBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Outline>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        OutlineBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for OutlineBuf {
    type Target = Outline;
    fn deref(&self) -> &Self::Target {
        unsafe { Outline::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Outline> for OutlineBuf {
    fn borrow(&self) -> &Outline {
        unsafe { Outline::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Outline {
    type Owned = OutlineBuf;
    fn to_owned(&self) -> Self::Owned {
        OutlineBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Outline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Outline")
            .field("n_points", &self.n_points())
            .field("corner_radius", &self.corner_radius())
            .field("pad", &2)
            .field("points", &self.points())
            .finish()
    }
}

impl std::fmt::Debug for OutlineBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutlineBuf")
            .field("n_points", &self.n_points())
            .field("corner_radius", &self.corner_radius())
            .field("pad", &2)
            .field("points", &self.points())
            .finish()
    }
}

#[derive(Clone)]
pub struct OutlineIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Outline>,
}

impl<'a> Iterator for OutlineIterator<'a> {
    type Item = &'a Outline;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Outline>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for OutlineIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct Shape {
    data: [u8],
}

#[allow(unused_parens)]
impl Shape {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Shape {
        debug_assert_eq!(data.as_ref().len(), <&Shape as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Shape)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn name(&self) -> xproto::Atom {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            ptr.read_unaligned()
        }
    }

    fn n_outlines(&self) -> u8 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn primary_ndx(&self) -> u8 {
        unsafe {
            let offset = 5usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn approx_ndx(&self) -> u8 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn outlines(&self) -> OutlineIterator<'_> {
        unsafe {
            let offset = 8usize;
            OutlineIterator {
                params: (),
                rem: (self.n_outlines() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::WiredOut for Shape {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Shape {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // name
        sz += 4usize;
        // n_outlines
        let n_outlines = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // primary_ndx
        sz += 1usize;
        // approx_ndx
        sz += 1usize;
        // pad
        sz += 1usize;
        // outlines
        for _ in 0 .. (n_outlines as usize) {
            sz += <&Outline>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Shape::from_data(data)
    }
}

#[derive(Clone)]
pub struct ShapeBuf {
    data: Vec<u8>,
}

impl ShapeBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> ShapeBuf {
        debug_assert_eq!(<&Shape>::compute_wire_len(data.as_ptr(), ()), data.len());
        ShapeBuf { data }
    }

    /// Construct a new [ShapeBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        name: xproto::Atom,
        primary_ndx: u8,
        approx_ndx: u8,
        outlines: &[OutlineBuf],
    ) -> ShapeBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 4; // name
            wire_sz += 1; // n_outlines
            wire_sz += 1; // primary_ndx
            wire_sz += 1; // approx_ndx
            wire_sz += 1; // pad
            wire_sz += outlines.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += name.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (outlines.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += primary_ndx.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += approx_ndx.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad
            for el in outlines {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            ShapeBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for ShapeBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Shape>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        ShapeBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for ShapeBuf {
    type Target = Shape;
    fn deref(&self) -> &Self::Target {
        unsafe { Shape::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Shape> for ShapeBuf {
    fn borrow(&self) -> &Shape {
        unsafe { Shape::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Shape {
    type Owned = ShapeBuf;
    fn to_owned(&self) -> Self::Owned {
        ShapeBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shape")
            .field("name", &self.name())
            .field("n_outlines", &self.n_outlines())
            .field("primary_ndx", &self.primary_ndx())
            .field("approx_ndx", &self.approx_ndx())
            .field("pad", &1)
            .field("outlines", &self.outlines())
            .finish()
    }
}

impl std::fmt::Debug for ShapeBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShapeBuf")
            .field("name", &self.name())
            .field("n_outlines", &self.n_outlines())
            .field("primary_ndx", &self.primary_ndx())
            .field("approx_ndx", &self.approx_ndx())
            .field("pad", &1)
            .field("outlines", &self.outlines())
            .finish()
    }
}

#[derive(Clone)]
pub struct ShapeIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Shape>,
}

impl<'a> Iterator for ShapeIterator<'a> {
    type Item = &'a Shape;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Shape>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for ShapeIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Key {
    pub name: [String8; 4],
    pub gap: i16,
    pub shape_ndx: u8,
    pub color_ndx: u8,
}

#[test]
fn test_sizeof_key() {
    assert_eq!(std::mem::size_of::<Key>(), 8);
}

impl base::WiredOut for Key {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Key as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for Key {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Key)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct OverlayKey {
    pub over: [String8; 4],
    pub under: [String8; 4],
}

#[test]
fn test_sizeof_overlay_key() {
    assert_eq!(std::mem::size_of::<OverlayKey>(), 8);
}

impl base::WiredOut for OverlayKey {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const OverlayKey as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for OverlayKey {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const OverlayKey)
    }
}

pub struct OverlayRow {
    data: [u8],
}

#[allow(unused_parens)]
impl OverlayRow {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &OverlayRow {
        debug_assert_eq!(data.as_ref().len(), <&OverlayRow as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const OverlayRow)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn row_under(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn n_keys(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn keys(&self) -> &[OverlayKey] {
        unsafe {
            let offset = 4usize;
            let len = (self.n_keys() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const OverlayKey;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for OverlayRow {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &OverlayRow {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // row_under
        sz += 1usize;
        // n_keys
        let n_keys = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // pad
        sz += 2usize;
        // keys
        sz += ((n_keys as usize) * 8usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        OverlayRow::from_data(data)
    }
}

#[derive(Clone)]
pub struct OverlayRowBuf {
    data: Vec<u8>,
}

impl OverlayRowBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> OverlayRowBuf {
        debug_assert_eq!(<&OverlayRow>::compute_wire_len(data.as_ptr(), ()), data.len());
        OverlayRowBuf { data }
    }

    /// Construct a new [OverlayRowBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        row_under: u8,
        keys: &[OverlayKey],
    ) -> OverlayRowBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // row_under
            wire_sz += 1; // n_keys
            wire_sz += 2; // pad
            wire_sz += keys.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += row_under.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (keys.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 2; // pad
            for el in keys {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            OverlayRowBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for OverlayRowBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&OverlayRow>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        OverlayRowBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for OverlayRowBuf {
    type Target = OverlayRow;
    fn deref(&self) -> &Self::Target {
        unsafe { OverlayRow::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<OverlayRow> for OverlayRowBuf {
    fn borrow(&self) -> &OverlayRow {
        unsafe { OverlayRow::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for OverlayRow {
    type Owned = OverlayRowBuf;
    fn to_owned(&self) -> Self::Owned {
        OverlayRowBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for OverlayRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OverlayRow")
            .field("row_under", &self.row_under())
            .field("n_keys", &self.n_keys())
            .field("pad", &2)
            .field("keys", &self.keys())
            .finish()
    }
}

impl std::fmt::Debug for OverlayRowBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OverlayRowBuf")
            .field("row_under", &self.row_under())
            .field("n_keys", &self.n_keys())
            .field("pad", &2)
            .field("keys", &self.keys())
            .finish()
    }
}

#[derive(Clone)]
pub struct OverlayRowIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a OverlayRow>,
}

impl<'a> Iterator for OverlayRowIterator<'a> {
    type Item = &'a OverlayRow;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&OverlayRow>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for OverlayRowIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct Overlay {
    data: [u8],
}

#[allow(unused_parens)]
impl Overlay {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Overlay {
        debug_assert_eq!(data.as_ref().len(), <&Overlay as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Overlay)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn name(&self) -> xproto::Atom {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            ptr.read_unaligned()
        }
    }

    fn n_rows(&self) -> u8 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn rows(&self) -> OverlayRowIterator<'_> {
        unsafe {
            let offset = 8usize;
            OverlayRowIterator {
                params: (),
                rem: (self.n_rows() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::WiredOut for Overlay {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Overlay {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // name
        sz += 4usize;
        // n_rows
        let n_rows = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // pad
        sz += 3usize;
        // rows
        for _ in 0 .. (n_rows as usize) {
            sz += <&OverlayRow>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Overlay::from_data(data)
    }
}

#[derive(Clone)]
pub struct OverlayBuf {
    data: Vec<u8>,
}

impl OverlayBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> OverlayBuf {
        debug_assert_eq!(<&Overlay>::compute_wire_len(data.as_ptr(), ()), data.len());
        OverlayBuf { data }
    }

    /// Construct a new [OverlayBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        name: xproto::Atom,
        rows: &[OverlayRowBuf],
    ) -> OverlayBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 4; // name
            wire_sz += 1; // n_rows
            wire_sz += 3; // pad
            wire_sz += rows.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += name.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (rows.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 3; // pad
            for el in rows {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            OverlayBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for OverlayBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Overlay>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        OverlayBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for OverlayBuf {
    type Target = Overlay;
    fn deref(&self) -> &Self::Target {
        unsafe { Overlay::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Overlay> for OverlayBuf {
    fn borrow(&self) -> &Overlay {
        unsafe { Overlay::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Overlay {
    type Owned = OverlayBuf;
    fn to_owned(&self) -> Self::Owned {
        OverlayBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Overlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Overlay")
            .field("name", &self.name())
            .field("n_rows", &self.n_rows())
            .field("pad", &3)
            .field("rows", &self.rows())
            .finish()
    }
}

impl std::fmt::Debug for OverlayBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OverlayBuf")
            .field("name", &self.name())
            .field("n_rows", &self.n_rows())
            .field("pad", &3)
            .field("rows", &self.rows())
            .finish()
    }
}

#[derive(Clone)]
pub struct OverlayIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Overlay>,
}

impl<'a> Iterator for OverlayIterator<'a> {
    type Item = &'a Overlay;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Overlay>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for OverlayIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct Row {
    data: [u8],
}

#[allow(unused_parens)]
impl Row {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Row {
        debug_assert_eq!(data.as_ref().len(), <&Row as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Row)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn top(&self) -> i16 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            ptr.read_unaligned()
        }
    }

    pub fn left(&self) -> i16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            ptr.read_unaligned()
        }
    }

    fn n_keys(&self) -> u8 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn vertical(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(5usize)) };
        val != 0
    }


    pub fn keys(&self) -> &[Key] {
        unsafe {
            let offset = 8usize;
            let len = (self.n_keys() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Key;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for Row {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Row {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // top
        sz += 2usize;
        // left
        sz += 2usize;
        // n_keys
        let n_keys = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // vertical
        sz += 1usize;
        // pad
        sz += 2usize;
        // keys
        sz += ((n_keys as usize) * 8usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Row::from_data(data)
    }
}

#[derive(Clone)]
pub struct RowBuf {
    data: Vec<u8>,
}

impl RowBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> RowBuf {
        debug_assert_eq!(<&Row>::compute_wire_len(data.as_ptr(), ()), data.len());
        RowBuf { data }
    }

    /// Construct a new [RowBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        top: i16,
        left: i16,
        vertical: bool,
        keys: &[Key],
    ) -> RowBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 2; // top
            wire_sz += 2; // left
            wire_sz += 1; // n_keys
            wire_sz += 1; // vertical
            wire_sz += 2; // pad
            wire_sz += keys.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += top.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += left.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (keys.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (if vertical { 1u8 } else { 0u8 }).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 2; // pad
            for el in keys {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            RowBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for RowBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Row>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        RowBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for RowBuf {
    type Target = Row;
    fn deref(&self) -> &Self::Target {
        unsafe { Row::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Row> for RowBuf {
    fn borrow(&self) -> &Row {
        unsafe { Row::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Row {
    type Owned = RowBuf;
    fn to_owned(&self) -> Self::Owned {
        RowBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Row")
            .field("top", &self.top())
            .field("left", &self.left())
            .field("n_keys", &self.n_keys())
            .field("vertical", &self.vertical())
            .field("pad", &2)
            .field("keys", &self.keys())
            .finish()
    }
}

impl std::fmt::Debug for RowBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowBuf")
            .field("top", &self.top())
            .field("left", &self.left())
            .field("n_keys", &self.n_keys())
            .field("vertical", &self.vertical())
            .field("pad", &2)
            .field("keys", &self.keys())
            .finish()
    }
}

#[derive(Clone)]
pub struct RowIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Row>,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = &'a Row;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Row>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for RowIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DoodadType {
    Outline = 1,
    Solid = 2,
    Text = 3,
    Indicator = 4,
    Logo = 5,
}

pub struct Listing {
    data: [u8],
}

#[allow(unused_parens)]
impl Listing {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Listing {
        debug_assert_eq!(data.as_ref().len(), <&Listing as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Listing)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn flags(&self) -> u16 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    fn length(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn string(&self) -> &[String8] {
        unsafe {
            let offset = 4usize;
            let len = (self.length() as usize);
            let ptr = self.wire_ptr().add(offset) as *const String8;
            std::slice::from_raw_parts(ptr, len)
        }
    }

}

impl base::WiredOut for Listing {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Listing {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // flags
        sz += 2usize;
        // length
        let length = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // string
        sz += (length as usize);
        // align pad
        sz += base::align_pad(sz, 2);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Listing::from_data(data)
    }
}

#[derive(Clone)]
pub struct ListingBuf {
    data: Vec<u8>,
}

impl ListingBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> ListingBuf {
        debug_assert_eq!(<&Listing>::compute_wire_len(data.as_ptr(), ()), data.len());
        ListingBuf { data }
    }

    /// Construct a new [ListingBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        flags: u16,
        string: &[String8],
    ) -> ListingBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 2; // flags
            wire_sz += 2; // length
            wire_sz += string.iter().map(|el| el.wire_len()).sum::<usize>();
            wire_sz += base::align_pad(wire_sz, 2);
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += flags.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (string.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            for el in string {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }
            wire_off += base::align_pad(wire_off, 2);

            ListingBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for ListingBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Listing>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        ListingBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for ListingBuf {
    type Target = Listing;
    fn deref(&self) -> &Self::Target {
        unsafe { Listing::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Listing> for ListingBuf {
    fn borrow(&self) -> &Listing {
        unsafe { Listing::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Listing {
    type Owned = ListingBuf;
    fn to_owned(&self) -> Self::Owned {
        ListingBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Listing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Listing")
            .field("flags", &self.flags())
            .field("length", &self.length())
            .field("string", &self.string())
            .field("align_pad", &2)
            .finish()
    }
}

impl std::fmt::Debug for ListingBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListingBuf")
            .field("flags", &self.flags())
            .field("length", &self.length())
            .field("string", &self.string())
            .field("align_pad", &2)
            .finish()
    }
}

#[derive(Clone)]
pub struct ListingIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Listing>,
}

impl<'a> Iterator for ListingIterator<'a> {
    type Item = &'a Listing;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Listing>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for ListingIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct DeviceLedInfo {
    data: [u8],
}

#[allow(unused_parens)]
impl DeviceLedInfo {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &DeviceLedInfo {
        debug_assert_eq!(data.as_ref().len(), <&DeviceLedInfo as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const DeviceLedInfo)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn led_class(&self) -> LedClass {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const LedClassSpec;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, LedClass>(val)
        }
    }

    pub fn led_id(&self) -> IdSpec {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const IdSpec;
            ptr.read_unaligned()
        }
    }

    fn names_present(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    fn maps_present(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn phys_indicators(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn state(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn names(&self) -> &[xproto::Atom] {
        unsafe {
            let offset = 20usize;
            let len = ((self.names_present() as usize).count_ones() as usize);
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn maps(&self) -> &[IndicatorMap] {
        unsafe {
            let offset = (20usize + (((self.names_present() as usize).count_ones() as usize) * 4usize));
            let len = ((self.maps_present() as usize).count_ones() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const IndicatorMap;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for DeviceLedInfo {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &DeviceLedInfo {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // led_class
        sz += 2usize;
        // led_id
        sz += 2usize;
        // names_present
        let names_present = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // maps_present
        let maps_present = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // phys_indicators
        sz += 4usize;
        // state
        sz += 4usize;
        // names
        sz += (((names_present as usize).count_ones() as usize) * 4usize);
        // maps
        sz += (((maps_present as usize).count_ones() as usize) * 12usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        DeviceLedInfo::from_data(data)
    }
}

#[derive(Clone)]
pub struct DeviceLedInfoBuf {
    data: Vec<u8>,
}

impl DeviceLedInfoBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> DeviceLedInfoBuf {
        debug_assert_eq!(<&DeviceLedInfo>::compute_wire_len(data.as_ptr(), ()), data.len());
        DeviceLedInfoBuf { data }
    }

    /// Construct a new [DeviceLedInfoBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        led_class: LedClass,
        led_id: IdSpec,
        names_present: u32,
        maps_present: u32,
        phys_indicators: u32,
        state: u32,
        names: &[xproto::Atom],
        maps: &[IndicatorMap],
    ) -> DeviceLedInfoBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 2; // led_class
            wire_sz += 2; // led_id
            wire_sz += 4; // names_present
            wire_sz += 4; // maps_present
            wire_sz += 4; // phys_indicators
            wire_sz += 4; // state
            wire_sz += names.iter().map(|el| el.wire_len()).sum::<usize>();
            wire_sz += maps.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += (std::mem::transmute::<_, u32>(led_class) as LedClassSpec).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += led_id.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += names_present.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += maps_present.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += phys_indicators.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += state.serialize(&mut wire_buf[wire_off .. ]);
            for el in names {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }
            for el in maps {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            DeviceLedInfoBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for DeviceLedInfoBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&DeviceLedInfo>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        DeviceLedInfoBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for DeviceLedInfoBuf {
    type Target = DeviceLedInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { DeviceLedInfo::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<DeviceLedInfo> for DeviceLedInfoBuf {
    fn borrow(&self) -> &DeviceLedInfo {
        unsafe { DeviceLedInfo::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for DeviceLedInfo {
    type Owned = DeviceLedInfoBuf;
    fn to_owned(&self) -> Self::Owned {
        DeviceLedInfoBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for DeviceLedInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceLedInfo")
            .field("led_class", &self.led_class())
            .field("led_id", &self.led_id())
            .field("names_present", &self.names_present())
            .field("maps_present", &self.maps_present())
            .field("phys_indicators", &self.phys_indicators())
            .field("state", &self.state())
            .field("names", &self.names())
            .field("maps", &self.maps())
            .finish()
    }
}

impl std::fmt::Debug for DeviceLedInfoBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceLedInfoBuf")
            .field("led_class", &self.led_class())
            .field("led_id", &self.led_id())
            .field("names_present", &self.names_present())
            .field("maps_present", &self.maps_present())
            .field("phys_indicators", &self.phys_indicators())
            .field("state", &self.state())
            .field("names", &self.names())
            .field("maps", &self.maps())
            .finish()
    }
}

#[derive(Clone)]
pub struct DeviceLedInfoIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a DeviceLedInfo>,
}

impl<'a> Iterator for DeviceLedInfoIterator<'a> {
    type Item = &'a DeviceLedInfo;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&DeviceLedInfo>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for DeviceLedInfoIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ErrorEnum {
    BadDevice = 255,
    BadClass = 254,
    BadId = 253,
}

bitflags! {
    pub struct Sa: u32 {
        const CLEAR_LOCKS = 0x00000001;
        const LATCH_TO_LOCK = 0x00000002;
        const USE_MOD_MAP_MODS = 0x00000004;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SaType {
    NoAction = 0,
    SetMods = 1,
    LatchMods = 2,
    LockMods = 3,
    SetGroup = 4,
    LatchGroup = 5,
    LockGroup = 6,
    MovePtr = 7,
    PtrBtn = 8,
    LockPtrBtn = 9,
    SetPtrDflt = 10,
    IsoLock = 11,
    Terminate = 12,
    SwitchScreen = 13,
    SetControls = 14,
    LockControls = 15,
    Message = 16,
    RedirectKey = 17,
    DeviceBtn = 18,
    LockDeviceBtn = 19,
    DeviceValuator = 20,
}

bitflags! {
    pub struct SaMovePtrFlag: u32 {
        const NO_ACCELERATION = 0x00000001;
        const MOVE_ABSOLUTE_X = 0x00000002;
        const MOVE_ABSOLUTE_Y = 0x00000004;
    }
}

bitflags! {
    pub struct SaSetPtrDfltFlag: u32 {
        const DFLT_BTN_ABSOLUTE = 0x00000004;
        const AFFECT_DFLT_BUTTON = 0x00000001;
    }
}

bitflags! {
    pub struct SaIsoLockFlag: u32 {
        const NO_LOCK = 0x00000001;
        const NO_UNLOCK = 0x00000002;
        const USE_MOD_MAP_MODS = 0x00000004;
        const ISO_DFLT_IS_GROUP = 0x00000008;
    }
}

bitflags! {
    pub struct SaIsoLockNoAffect: u32 {
        const CTRLS = 0x00000008;
        const PTR = 0x00000010;
        const GROUP = 0x00000020;
        const MODS = 0x00000040;
    }
}

bitflags! {
    pub struct SwitchScreenFlag: u32 {
        const APPLICATION = 0x00000001;
        const ABSOLUTE = 0x00000004;
    }
}

bitflags! {
    pub struct BoolCtrlsHigh: u32 {
        const ACCESS_X_FEEDBACK = 0x00000001;
        const AUDIBLE_BELL = 0x00000002;
        const OVERLAY1 = 0x00000004;
        const OVERLAY2 = 0x00000008;
        const IGNORE_GROUP_LOCK = 0x00000010;
    }
}

bitflags! {
    pub struct BoolCtrlsLow: u32 {
        const REPEAT_KEYS = 0x00000001;
        const SLOW_KEYS = 0x00000002;
        const BOUNCE_KEYS = 0x00000004;
        const STICKY_KEYS = 0x00000008;
        const MOUSE_KEYS = 0x00000010;
        const MOUSE_KEYS_ACCEL = 0x00000020;
        const ACCESS_X_KEYS = 0x00000040;
        const ACCESS_X_TIMEOUT = 0x00000080;
    }
}

bitflags! {
    pub struct ActionMessageFlag: u32 {
        const ON_PRESS = 0x00000001;
        const ON_RELEASE = 0x00000002;
        const GEN_KEY_EVENT = 0x00000004;
    }
}

bitflags! {
    pub struct LockDeviceFlags: u32 {
        const NO_LOCK = 0x00000001;
        const NO_UNLOCK = 0x00000002;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SaValWhat {
    IgnoreVal = 0,
    SetValMin = 1,
    SetValCenter = 2,
    SetValMax = 3,
    SetValRelative = 4,
    SetValAbsolute = 5,
}

#[derive(Copy, Clone)]
pub struct SiAction {
    data: [u8; 8],
}

#[allow(unused_parens)]
impl SiAction {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &SiAction {
        debug_assert_eq!(data.as_ref().len(), 8);
        &*(data.as_ref() as *const [u8] as *const SiAction)
    }

    /// Construct a new [SiAction].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        r#type: SaType,
        data: &[u8; 7],
    ) -> SiAction {
            unsafe {
            let mut wire_buf = [0u8; 8];
            let mut wire_off = 0usize;

            wire_off += (std::mem::transmute::<_, u32>(r#type) as u8).serialize(&mut wire_buf[wire_off .. ]);
            for el in data {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            SiAction { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn r#type(&self) -> SaType {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, SaType>(val)
        }
    }

    pub fn data(&self) -> &[u8; 7] {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const [u8; 7];
            &*ptr
        }
    }
}

#[test]
fn test_sizeof_si_action() {
    assert_eq!(std::mem::size_of::<SiAction>(), 8);
}

impl base::WiredOut for SiAction {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for SiAction {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const SiAction)
    }
}

impl std::fmt::Debug for SiAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SiAction")
            .field("r#type", &self.r#type())
            .field("data", &self.data())
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct SymInterpret {
    data: [u8; 16],
}

#[allow(unused_parens)]
impl SymInterpret {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &SymInterpret {
        debug_assert_eq!(data.as_ref().len(), 16);
        &*(data.as_ref() as *const [u8] as *const SymInterpret)
    }

    /// Construct a new [SymInterpret].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        sym: xproto::Keysym,
        mods: xproto::ModMask,
        r#match: u8,
        virtual_mod: VModsLow,
        flags: u8,
        action: SiAction,
    ) -> SymInterpret {
            unsafe {
            let mut wire_buf = [0u8; 16];
            let mut wire_off = 0usize;

            wire_off += sym.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (mods.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += r#match.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (virtual_mod.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += flags.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += action.serialize(&mut wire_buf[wire_off .. ]);

            SymInterpret { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn sym(&self) -> xproto::Keysym {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keysym;
            ptr.read_unaligned()
        }
    }

    pub fn mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn r#match(&self) -> u8 {
        unsafe {
            let offset = 5usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn virtual_mod(&self) -> VModsLow {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VModsLow>(val)
        }
    }

    pub fn flags(&self) -> u8 {
        unsafe {
            let offset = 7usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn action(&self) -> &SiAction {
        unsafe {
            let offset = 8usize;
            let len = 8usize;
            let data = std::slice::from_raw_parts(self.wire_ptr().add(offset), len);
            SiAction::from_data(data)
        }
    }
}

#[test]
fn test_sizeof_sym_interpret() {
    assert_eq!(std::mem::size_of::<SymInterpret>(), 16);
}

impl base::WiredOut for SymInterpret {
    fn wire_len(&self) -> usize { 16 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for SymInterpret {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 16 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 16;
        *(ptr as *const SymInterpret)
    }
}

impl std::fmt::Debug for SymInterpret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SymInterpret")
            .field("sym", &self.sym())
            .field("mods", &self.mods())
            .field("r#match", &self.r#match())
            .field("virtual_mod", &self.virtual_mod())
            .field("flags", &self.flags())
            .field("action", &self.action())
            .finish()
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    NoAction{
    },
    SetMods{
        flags: u8,
        mask: u8,
        real_mods: u8,
        vmods_high: u8,
        vmods_low: u8,
    },
    LatchMods{
        flags: u8,
        mask: u8,
        real_mods: u8,
        vmods_high: u8,
        vmods_low: u8,
    },
    LockMods{
        flags: u8,
        mask: u8,
        real_mods: u8,
        vmods_high: u8,
        vmods_low: u8,
    },
    SetGroup{
        flags: u8,
        group: i8,
    },
    LatchGroup{
        flags: u8,
        group: i8,
    },
    LockGroup{
        flags: u8,
        group: i8,
    },
    MovePtr{
        flags: u8,
        x_high: i8,
        x_low: u8,
        y_high: i8,
        y_low: u8,
    },
    PtrBtn{
        flags: u8,
        count: u8,
        button: u8,
    },
    LockPtrBtn{
        flags: u8,
        button: u8,
    },
    SetPtrDflt{
        flags: u8,
        affect: u8,
        value: i8,
    },
    IsoLock{
        flags: u8,
        mask: u8,
        real_mods: u8,
        group: i8,
        affect: u8,
        vmods_high: u8,
        vmods_low: u8,
    },
    Terminate{
    },
    SwitchScreen{
        flags: u8,
        new_screen: i8,
    },
    SetControls{
        bool_ctrls_high: u8,
        bool_ctrls_low: u8,
    },
    LockControls{
        bool_ctrls_high: u8,
        bool_ctrls_low: u8,
    },
    Message{
        flags: u8,
        message: [u8; 6],
    },
    RedirectKey{
        newkey: xproto::Keycode,
        mask: u8,
        real_modifiers: u8,
        vmods_mask_high: u8,
        vmods_mask_low: u8,
        vmods_high: u8,
        vmods_low: u8,
    },
    DeviceBtn{
        flags: u8,
        count: u8,
        button: u8,
        device: u8,
    },
    LockDeviceBtn{
        flags: u8,
        button: u8,
        device: u8,
    },
    DeviceValuator{
        device: u8,
        val1what: u8,
        val1index: u8,
        val1value: u8,
        val2what: u8,
        val2index: u8,
        val2value: u8,
    },
}

impl Action {
    unsafe fn from_data(wire_data: &[u8]) -> Action {
        let r#type = wire_data[0] as u32;
        match r#type {
            0 => Action::NoAction{
            },
            1 => Action::SetMods{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                mask: *(wire_data.as_ptr().add(2) as *const u8),
                real_mods: *(wire_data.as_ptr().add(3) as *const u8),
                vmods_high: *(wire_data.as_ptr().add(4) as *const u8),
                vmods_low: *(wire_data.as_ptr().add(5) as *const u8),
            },
            2 => Action::LatchMods{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                mask: *(wire_data.as_ptr().add(2) as *const u8),
                real_mods: *(wire_data.as_ptr().add(3) as *const u8),
                vmods_high: *(wire_data.as_ptr().add(4) as *const u8),
                vmods_low: *(wire_data.as_ptr().add(5) as *const u8),
            },
            3 => Action::LockMods{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                mask: *(wire_data.as_ptr().add(2) as *const u8),
                real_mods: *(wire_data.as_ptr().add(3) as *const u8),
                vmods_high: *(wire_data.as_ptr().add(4) as *const u8),
                vmods_low: *(wire_data.as_ptr().add(5) as *const u8),
            },
            4 => Action::SetGroup{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                group: *(wire_data.as_ptr().add(2) as *const i8),
            },
            5 => Action::LatchGroup{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                group: *(wire_data.as_ptr().add(2) as *const i8),
            },
            6 => Action::LockGroup{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                group: *(wire_data.as_ptr().add(2) as *const i8),
            },
            7 => Action::MovePtr{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                x_high: *(wire_data.as_ptr().add(2) as *const i8),
                x_low: *(wire_data.as_ptr().add(3) as *const u8),
                y_high: *(wire_data.as_ptr().add(4) as *const i8),
                y_low: *(wire_data.as_ptr().add(5) as *const u8),
            },
            8 => Action::PtrBtn{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                count: *(wire_data.as_ptr().add(2) as *const u8),
                button: *(wire_data.as_ptr().add(3) as *const u8),
            },
            9 => Action::LockPtrBtn{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                button: *(wire_data.as_ptr().add(3) as *const u8),
            },
            10 => Action::SetPtrDflt{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                affect: *(wire_data.as_ptr().add(2) as *const u8),
                value: *(wire_data.as_ptr().add(3) as *const i8),
            },
            11 => Action::IsoLock{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                mask: *(wire_data.as_ptr().add(2) as *const u8),
                real_mods: *(wire_data.as_ptr().add(3) as *const u8),
                group: *(wire_data.as_ptr().add(4) as *const i8),
                affect: *(wire_data.as_ptr().add(5) as *const u8),
                vmods_high: *(wire_data.as_ptr().add(6) as *const u8),
                vmods_low: *(wire_data.as_ptr().add(7) as *const u8),
            },
            12 => Action::Terminate{
            },
            13 => Action::SwitchScreen{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                new_screen: *(wire_data.as_ptr().add(2) as *const i8),
            },
            14 => Action::SetControls{
                bool_ctrls_high: *(wire_data.as_ptr().add(4) as *const u8),
                bool_ctrls_low: *(wire_data.as_ptr().add(5) as *const u8),
            },
            15 => Action::LockControls{
                bool_ctrls_high: *(wire_data.as_ptr().add(4) as *const u8),
                bool_ctrls_low: *(wire_data.as_ptr().add(5) as *const u8),
            },
            16 => Action::Message{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                message: wire_data[2..8].try_into().unwrap(),
            },
            17 => Action::RedirectKey{
                newkey: *(wire_data.as_ptr().add(1) as *const xproto::Keycode),
                mask: *(wire_data.as_ptr().add(2) as *const u8),
                real_modifiers: *(wire_data.as_ptr().add(3) as *const u8),
                vmods_mask_high: *(wire_data.as_ptr().add(4) as *const u8),
                vmods_mask_low: *(wire_data.as_ptr().add(5) as *const u8),
                vmods_high: *(wire_data.as_ptr().add(6) as *const u8),
                vmods_low: *(wire_data.as_ptr().add(7) as *const u8),
            },
            18 => Action::DeviceBtn{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                count: *(wire_data.as_ptr().add(2) as *const u8),
                button: *(wire_data.as_ptr().add(3) as *const u8),
                device: *(wire_data.as_ptr().add(4) as *const u8),
            },
            19 => Action::LockDeviceBtn{
                flags: *(wire_data.as_ptr().add(1) as *const u8),
                button: *(wire_data.as_ptr().add(3) as *const u8),
                device: *(wire_data.as_ptr().add(4) as *const u8),
            },
            20 => Action::DeviceValuator{
                device: *(wire_data.as_ptr().add(1) as *const u8),
                val1what: *(wire_data.as_ptr().add(2) as *const u8),
                val1index: *(wire_data.as_ptr().add(3) as *const u8),
                val1value: *(wire_data.as_ptr().add(4) as *const u8),
                val2what: *(wire_data.as_ptr().add(5) as *const u8),
                val2index: *(wire_data.as_ptr().add(6) as *const u8),
                val2value: *(wire_data.as_ptr().add(7) as *const u8),
            },
            _ => unreachable!("unexpected type value for xkb::Action"),
        }
    }
}

impl base::WiredOut for Action {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        match self {
            Action::NoAction{
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::NoAction) } as u8;
            }
            Action::SetMods{
                flags,
                mask,
                real_mods,
                vmods_high,
                vmods_low,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::SetMods) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                mask.serialize(&mut wire_buf[2..]);
                real_mods.serialize(&mut wire_buf[3..]);
                vmods_high.serialize(&mut wire_buf[4..]);
                vmods_low.serialize(&mut wire_buf[5..]);
            }
            Action::LatchMods{
                flags,
                mask,
                real_mods,
                vmods_high,
                vmods_low,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::LatchMods) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                mask.serialize(&mut wire_buf[2..]);
                real_mods.serialize(&mut wire_buf[3..]);
                vmods_high.serialize(&mut wire_buf[4..]);
                vmods_low.serialize(&mut wire_buf[5..]);
            }
            Action::LockMods{
                flags,
                mask,
                real_mods,
                vmods_high,
                vmods_low,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::LockMods) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                mask.serialize(&mut wire_buf[2..]);
                real_mods.serialize(&mut wire_buf[3..]);
                vmods_high.serialize(&mut wire_buf[4..]);
                vmods_low.serialize(&mut wire_buf[5..]);
            }
            Action::SetGroup{
                flags,
                group,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::SetGroup) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                group.serialize(&mut wire_buf[2..]);
            }
            Action::LatchGroup{
                flags,
                group,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::LatchGroup) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                group.serialize(&mut wire_buf[2..]);
            }
            Action::LockGroup{
                flags,
                group,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::LockGroup) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                group.serialize(&mut wire_buf[2..]);
            }
            Action::MovePtr{
                flags,
                x_high,
                x_low,
                y_high,
                y_low,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::MovePtr) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                x_high.serialize(&mut wire_buf[2..]);
                x_low.serialize(&mut wire_buf[3..]);
                y_high.serialize(&mut wire_buf[4..]);
                y_low.serialize(&mut wire_buf[5..]);
            }
            Action::PtrBtn{
                flags,
                count,
                button,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::PtrBtn) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                count.serialize(&mut wire_buf[2..]);
                button.serialize(&mut wire_buf[3..]);
            }
            Action::LockPtrBtn{
                flags,
                button,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::LockPtrBtn) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                button.serialize(&mut wire_buf[3..]);
            }
            Action::SetPtrDflt{
                flags,
                affect,
                value,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::SetPtrDflt) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                affect.serialize(&mut wire_buf[2..]);
                value.serialize(&mut wire_buf[3..]);
            }
            Action::IsoLock{
                flags,
                mask,
                real_mods,
                group,
                affect,
                vmods_high,
                vmods_low,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::IsoLock) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                mask.serialize(&mut wire_buf[2..]);
                real_mods.serialize(&mut wire_buf[3..]);
                group.serialize(&mut wire_buf[4..]);
                affect.serialize(&mut wire_buf[5..]);
                vmods_high.serialize(&mut wire_buf[6..]);
                vmods_low.serialize(&mut wire_buf[7..]);
            }
            Action::Terminate{
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::Terminate) } as u8;
            }
            Action::SwitchScreen{
                flags,
                new_screen,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::SwitchScreen) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                new_screen.serialize(&mut wire_buf[2..]);
            }
            Action::SetControls{
                bool_ctrls_high,
                bool_ctrls_low,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::SetControls) } as u8;
                bool_ctrls_high.serialize(&mut wire_buf[4..]);
                bool_ctrls_low.serialize(&mut wire_buf[5..]);
            }
            Action::LockControls{
                bool_ctrls_high,
                bool_ctrls_low,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::LockControls) } as u8;
                bool_ctrls_high.serialize(&mut wire_buf[4..]);
                bool_ctrls_low.serialize(&mut wire_buf[5..]);
            }
            Action::Message{
                flags,
                message,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::Message) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                wire_buf[2..8].copy_from_slice(message);
            }
            Action::RedirectKey{
                newkey,
                mask,
                real_modifiers,
                vmods_mask_high,
                vmods_mask_low,
                vmods_high,
                vmods_low,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::RedirectKey) } as u8;
                newkey.serialize(&mut wire_buf[1..]);
                mask.serialize(&mut wire_buf[2..]);
                real_modifiers.serialize(&mut wire_buf[3..]);
                vmods_mask_high.serialize(&mut wire_buf[4..]);
                vmods_mask_low.serialize(&mut wire_buf[5..]);
                vmods_high.serialize(&mut wire_buf[6..]);
                vmods_low.serialize(&mut wire_buf[7..]);
            }
            Action::DeviceBtn{
                flags,
                count,
                button,
                device,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::DeviceBtn) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                count.serialize(&mut wire_buf[2..]);
                button.serialize(&mut wire_buf[3..]);
                device.serialize(&mut wire_buf[4..]);
            }
            Action::LockDeviceBtn{
                flags,
                button,
                device,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::LockDeviceBtn) } as u8;
                flags.serialize(&mut wire_buf[1..]);
                button.serialize(&mut wire_buf[3..]);
                device.serialize(&mut wire_buf[4..]);
            }
            Action::DeviceValuator{
                device,
                val1what,
                val1index,
                val1value,
                val2what,
                val2index,
                val2value,
            } => {
                wire_buf[0] += unsafe { std::mem::transmute::<_, u32>(SaType::DeviceValuator) } as u8;
                device.serialize(&mut wire_buf[1..]);
                val1what.serialize(&mut wire_buf[2..]);
                val1index.serialize(&mut wire_buf[3..]);
                val1value.serialize(&mut wire_buf[4..]);
                val2what.serialize(&mut wire_buf[5..]);
                val2index.serialize(&mut wire_buf[6..]);
                val2value.serialize(&mut wire_buf[7..]);
            }
        }
        8
    }
}

impl base::WiredIn for Action {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: Self::Params) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        Self::from_data(std::slice::from_raw_parts(ptr, sz))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SelectEventsDetailsParams {
    pub affect_which: usize,
    pub clear: usize,
    pub select_all: usize,
}

#[derive(Clone, Debug)]
pub enum SelectEventsDetails {
    NewKeyboardNotify{
        affect_new_keyboard: NknDetail,
        new_keyboard_details: NknDetail,
    },
    StateNotify{
        affect_state: StatePart,
        state_details: StatePart,
    },
    ControlsNotify{
        affect_ctrls: Control,
        ctrl_details: Control,
    },
    IndicatorStateNotify{
        affect_indicator_state: u32,
        indicator_state_details: u32,
    },
    IndicatorMapNotify{
        affect_indicator_map: u32,
        indicator_map_details: u32,
    },
    NamesNotify{
        affect_names: NameDetail,
        names_details: NameDetail,
    },
    CompatMapNotify{
        affect_compat: CmDetail,
        compat_details: CmDetail,
    },
    BellNotify{
        affect_bell: u8,
        bell_details: u8,
    },
    ActionMessage{
        affect_msg_details: u8,
        msg_details: u8,
    },
    AccessXNotify{
        affect_access_x: AxnDetail,
        access_x_details: AxnDetail,
    },
    ExtensionDeviceNotify{
        affect_ext_dev: XiFeature,
        extdev_details: XiFeature,
    },
}

impl SelectEventsDetails {
    pub(crate) fn get_mask(slice: &[SelectEventsDetails]) -> EventType {
        let mut res = EventType::empty();
        for el in slice {
            match el {
                SelectEventsDetails::NewKeyboardNotify{..} => {
                    res |= EventType::NEW_KEYBOARD_NOTIFY;
                }
                SelectEventsDetails::StateNotify{..} => {
                    res |= EventType::STATE_NOTIFY;
                }
                SelectEventsDetails::ControlsNotify{..} => {
                    res |= EventType::CONTROLS_NOTIFY;
                }
                SelectEventsDetails::IndicatorStateNotify{..} => {
                    res |= EventType::INDICATOR_STATE_NOTIFY;
                }
                SelectEventsDetails::IndicatorMapNotify{..} => {
                    res |= EventType::INDICATOR_MAP_NOTIFY;
                }
                SelectEventsDetails::NamesNotify{..} => {
                    res |= EventType::NAMES_NOTIFY;
                }
                SelectEventsDetails::CompatMapNotify{..} => {
                    res |= EventType::COMPAT_MAP_NOTIFY;
                }
                SelectEventsDetails::BellNotify{..} => {
                    res |= EventType::BELL_NOTIFY;
                }
                SelectEventsDetails::ActionMessage{..} => {
                    res |= EventType::ACTION_MESSAGE;
                }
                SelectEventsDetails::AccessXNotify{..} => {
                    res |= EventType::ACCESS_X_NOTIFY;
                }
                SelectEventsDetails::ExtensionDeviceNotify{..} => {
                    res |= EventType::EXTENSION_DEVICE_NOTIFY;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[SelectEventsDetails]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            SelectEventsDetails::NewKeyboardNotify{..} => 0,
            SelectEventsDetails::StateNotify{..} => 1,
            SelectEventsDetails::ControlsNotify{..} => 2,
            SelectEventsDetails::IndicatorStateNotify{..} => 3,
            SelectEventsDetails::IndicatorMapNotify{..} => 4,
            SelectEventsDetails::NamesNotify{..} => 5,
            SelectEventsDetails::CompatMapNotify{..} => 6,
            SelectEventsDetails::BellNotify{..} => 7,
            SelectEventsDetails::ActionMessage{..} => 8,
            SelectEventsDetails::AccessXNotify{..} => 9,
            SelectEventsDetails::ExtensionDeviceNotify{..} => 10,
        }
    }
}

impl base::WiredOut for &[SelectEventsDetails] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                SelectEventsDetails::NewKeyboardNotify{
                    ..
                } => {
                    sz += 2;
                    sz += 2;
                }
                SelectEventsDetails::StateNotify{
                    ..
                } => {
                    sz += 2;
                    sz += 2;
                }
                SelectEventsDetails::ControlsNotify{
                    ..
                } => {
                    sz += 4;
                    sz += 4;
                }
                SelectEventsDetails::IndicatorStateNotify{
                    ..
                } => {
                    sz += 4;
                    sz += 4;
                }
                SelectEventsDetails::IndicatorMapNotify{
                    ..
                } => {
                    sz += 4;
                    sz += 4;
                }
                SelectEventsDetails::NamesNotify{
                    ..
                } => {
                    sz += 2;
                    sz += 2;
                }
                SelectEventsDetails::CompatMapNotify{
                    ..
                } => {
                    sz += 1;
                    sz += 1;
                }
                SelectEventsDetails::BellNotify{
                    ..
                } => {
                    sz += 1;
                    sz += 1;
                }
                SelectEventsDetails::ActionMessage{
                    ..
                } => {
                    sz += 1;
                    sz += 1;
                }
                SelectEventsDetails::AccessXNotify{
                    ..
                } => {
                    sz += 2;
                    sz += 2;
                }
                SelectEventsDetails::ExtensionDeviceNotify{
                    ..
                } => {
                    sz += 2;
                    sz += 2;
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                SelectEventsDetails::NewKeyboardNotify{
                    affect_new_keyboard,
                    new_keyboard_details,
                    ..
                } => {
                    offset += (affect_new_keyboard.bits() as u16).serialize(&mut wire_buf[offset..]);
                    offset += (new_keyboard_details.bits() as u16).serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::StateNotify{
                    affect_state,
                    state_details,
                    ..
                } => {
                    offset += (affect_state.bits() as u16).serialize(&mut wire_buf[offset..]);
                    offset += (state_details.bits() as u16).serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::ControlsNotify{
                    affect_ctrls,
                    ctrl_details,
                    ..
                } => {
                    offset += (affect_ctrls.bits() as u32).serialize(&mut wire_buf[offset..]);
                    offset += (ctrl_details.bits() as u32).serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::IndicatorStateNotify{
                    affect_indicator_state,
                    indicator_state_details,
                    ..
                } => {
                    offset += affect_indicator_state.serialize(&mut wire_buf[offset..]);
                    offset += indicator_state_details.serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::IndicatorMapNotify{
                    affect_indicator_map,
                    indicator_map_details,
                    ..
                } => {
                    offset += affect_indicator_map.serialize(&mut wire_buf[offset..]);
                    offset += indicator_map_details.serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::NamesNotify{
                    affect_names,
                    names_details,
                    ..
                } => {
                    offset += (affect_names.bits() as u16).serialize(&mut wire_buf[offset..]);
                    offset += (names_details.bits() as u16).serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::CompatMapNotify{
                    affect_compat,
                    compat_details,
                    ..
                } => {
                    offset += (affect_compat.bits() as u8).serialize(&mut wire_buf[offset..]);
                    offset += (compat_details.bits() as u8).serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::BellNotify{
                    affect_bell,
                    bell_details,
                    ..
                } => {
                    offset += affect_bell.serialize(&mut wire_buf[offset..]);
                    offset += bell_details.serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::ActionMessage{
                    affect_msg_details,
                    msg_details,
                    ..
                } => {
                    offset += affect_msg_details.serialize(&mut wire_buf[offset..]);
                    offset += msg_details.serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::AccessXNotify{
                    affect_access_x,
                    access_x_details,
                    ..
                } => {
                    offset += (affect_access_x.bits() as u16).serialize(&mut wire_buf[offset..]);
                    offset += (access_x_details.bits() as u16).serialize(&mut wire_buf[offset..]);
                }
                SelectEventsDetails::ExtensionDeviceNotify{
                    affect_ext_dev,
                    extdev_details,
                    ..
                } => {
                    offset += (affect_ext_dev.bits() as u16).serialize(&mut wire_buf[offset..]);
                    offset += (extdev_details.bits() as u16).serialize(&mut wire_buf[offset..]);
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<SelectEventsDetails> {
    type Params = SelectEventsDetailsParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let SelectEventsDetailsParams {
            affect_which,
            clear,
            select_all,
        } = params;
        let expr = ((affect_which as usize) & ((!(clear as usize)) & (!(select_all as usize))));
        let mut sz = 0usize;
        if expr & (EventType::NEW_KEYBOARD_NOTIFY.bits() as usize) != 0 {
            // affect_new_keyboard
            sz += 2usize;
            // new_keyboard_details
            sz += 2usize;
        }
        if expr & (EventType::STATE_NOTIFY.bits() as usize) != 0 {
            // affect_state
            sz += 2usize;
            // state_details
            sz += 2usize;
        }
        if expr & (EventType::CONTROLS_NOTIFY.bits() as usize) != 0 {
            // affect_ctrls
            sz += 4usize;
            // ctrl_details
            sz += 4usize;
        }
        if expr & (EventType::INDICATOR_STATE_NOTIFY.bits() as usize) != 0 {
            // affect_indicator_state
            sz += 4usize;
            // indicator_state_details
            sz += 4usize;
        }
        if expr & (EventType::INDICATOR_MAP_NOTIFY.bits() as usize) != 0 {
            // affect_indicator_map
            sz += 4usize;
            // indicator_map_details
            sz += 4usize;
        }
        if expr & (EventType::NAMES_NOTIFY.bits() as usize) != 0 {
            // affect_names
            sz += 2usize;
            // names_details
            sz += 2usize;
        }
        if expr & (EventType::COMPAT_MAP_NOTIFY.bits() as usize) != 0 {
            // affect_compat
            sz += 1usize;
            // compat_details
            sz += 1usize;
        }
        if expr & (EventType::BELL_NOTIFY.bits() as usize) != 0 {
            // affect_bell
            sz += 1usize;
            // bell_details
            sz += 1usize;
        }
        if expr & (EventType::ACTION_MESSAGE.bits() as usize) != 0 {
            // affect_msg_details
            sz += 1usize;
            // msg_details
            sz += 1usize;
        }
        if expr & (EventType::ACCESS_X_NOTIFY.bits() as usize) != 0 {
            // affect_access_x
            sz += 2usize;
            // access_x_details
            sz += 2usize;
        }
        if expr & (EventType::EXTENSION_DEVICE_NOTIFY.bits() as usize) != 0 {
            // affect_ext_dev
            sz += 2usize;
            // extdev_details
            sz += 2usize;
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: SelectEventsDetailsParams, out_offset: &mut usize) -> Vec<SelectEventsDetails> {
        let SelectEventsDetailsParams{
            affect_which,
            clear,
            select_all,
        } = params;
        let expr = ((affect_which as usize) & ((!(clear as usize)) & (!(select_all as usize))));
        let mut result = Vec::new();
        if expr & (EventType::NEW_KEYBOARD_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_new_keyboard = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let new_keyboard_details = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            *out_offset += offset;
            result.push(SelectEventsDetails::NewKeyboardNotify{
                affect_new_keyboard: NknDetail::from_bits(affect_new_keyboard as u32).unwrap(),
                new_keyboard_details: NknDetail::from_bits(new_keyboard_details as u32).unwrap(),
            });
        }
        if expr & (EventType::STATE_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_state = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let state_details = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            *out_offset += offset;
            result.push(SelectEventsDetails::StateNotify{
                affect_state: StatePart::from_bits(affect_state as u32).unwrap(),
                state_details: StatePart::from_bits(state_details as u32).unwrap(),
            });
        }
        if expr & (EventType::CONTROLS_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_ctrls = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let ctrl_details = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(SelectEventsDetails::ControlsNotify{
                affect_ctrls: Control::from_bits(affect_ctrls as u32).unwrap(),
                ctrl_details: Control::from_bits(ctrl_details as u32).unwrap(),
            });
        }
        if expr & (EventType::INDICATOR_STATE_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_indicator_state = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let indicator_state_details = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(SelectEventsDetails::IndicatorStateNotify{
                affect_indicator_state,
                indicator_state_details,
            });
        }
        if expr & (EventType::INDICATOR_MAP_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_indicator_map = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let indicator_map_details = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(SelectEventsDetails::IndicatorMapNotify{
                affect_indicator_map,
                indicator_map_details,
            });
        }
        if expr & (EventType::NAMES_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_names = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let names_details = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            *out_offset += offset;
            result.push(SelectEventsDetails::NamesNotify{
                affect_names: NameDetail::from_bits(affect_names as u32).unwrap(),
                names_details: NameDetail::from_bits(names_details as u32).unwrap(),
            });
        }
        if expr & (EventType::COMPAT_MAP_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_compat = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let compat_details = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            *out_offset += offset;
            result.push(SelectEventsDetails::CompatMapNotify{
                affect_compat: CmDetail::from_bits(affect_compat as u32).unwrap(),
                compat_details: CmDetail::from_bits(compat_details as u32).unwrap(),
            });
        }
        if expr & (EventType::BELL_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_bell = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let bell_details = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            *out_offset += offset;
            result.push(SelectEventsDetails::BellNotify{
                affect_bell,
                bell_details,
            });
        }
        if expr & (EventType::ACTION_MESSAGE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_msg_details = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let msg_details = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            *out_offset += offset;
            result.push(SelectEventsDetails::ActionMessage{
                affect_msg_details,
                msg_details,
            });
        }
        if expr & (EventType::ACCESS_X_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_access_x = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let access_x_details = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            *out_offset += offset;
            result.push(SelectEventsDetails::AccessXNotify{
                affect_access_x: AxnDetail::from_bits(affect_access_x as u32).unwrap(),
                access_x_details: AxnDetail::from_bits(access_x_details as u32).unwrap(),
            });
        }
        if expr & (EventType::EXTENSION_DEVICE_NOTIFY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let affect_ext_dev = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let extdev_details = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            *out_offset += offset;
            result.push(SelectEventsDetails::ExtensionDeviceNotify{
                affect_ext_dev: XiFeature::from_bits(affect_ext_dev as u32).unwrap(),
                extdev_details: XiFeature::from_bits(extdev_details as u32).unwrap(),
            });
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GetMapReplyMapParams {
    pub present: usize,
    pub n_key_actions: usize,
    pub n_key_syms: usize,
    pub n_types: usize,
    pub total_actions: usize,
    pub total_key_behaviors: usize,
    pub total_key_explicit: usize,
    pub total_mod_map_keys: usize,
    pub total_v_mod_map_keys: usize,
    pub virtual_mods: usize,
}

#[derive(Clone, Debug)]
pub enum GetMapReplyMap {
    KeyTypes(Vec<KeyTypeBuf>),
    KeySyms(Vec<KeySymMapBuf>),
    KeyActions{
        acts_rtrn_count: Vec<u8>,
        acts_rtrn_acts: Vec<Action>,
    },
    KeyBehaviors(Vec<SetBehavior>),
    VirtualMods(Vec<xproto::ModMask>),
    ExplicitComponents(Vec<SetExplicit>),
    ModifierMap(Vec<KeyModMap>),
    VirtualModMap(Vec<KeyVModMap>),
}

impl GetMapReplyMap {
    pub(crate) fn get_mask(slice: &[GetMapReplyMap]) -> MapPart {
        let mut res = MapPart::empty();
        for el in slice {
            match el {
                GetMapReplyMap::KeyTypes{..} => {
                    res |= MapPart::KEY_TYPES;
                }
                GetMapReplyMap::KeySyms{..} => {
                    res |= MapPart::KEY_SYMS;
                }
                GetMapReplyMap::KeyActions{..} => {
                    res |= MapPart::KEY_ACTIONS;
                }
                GetMapReplyMap::KeyBehaviors{..} => {
                    res |= MapPart::KEY_BEHAVIORS;
                }
                GetMapReplyMap::VirtualMods{..} => {
                    res |= MapPart::VIRTUAL_MODS;
                }
                GetMapReplyMap::ExplicitComponents{..} => {
                    res |= MapPart::EXPLICIT_COMPONENTS;
                }
                GetMapReplyMap::ModifierMap{..} => {
                    res |= MapPart::MODIFIER_MAP;
                }
                GetMapReplyMap::VirtualModMap{..} => {
                    res |= MapPart::VIRTUAL_MOD_MAP;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[GetMapReplyMap]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            GetMapReplyMap::KeyTypes{..} => 0,
            GetMapReplyMap::KeySyms{..} => 1,
            GetMapReplyMap::KeyActions{..} => 2,
            GetMapReplyMap::KeyBehaviors{..} => 3,
            GetMapReplyMap::VirtualMods{..} => 4,
            GetMapReplyMap::ExplicitComponents{..} => 5,
            GetMapReplyMap::ModifierMap{..} => 6,
            GetMapReplyMap::VirtualModMap{..} => 7,
        }
    }
}

impl base::WiredOut for &[GetMapReplyMap] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                GetMapReplyMap::KeyTypes(
                    types_rtrn,
                    ..
                ) => {
                    for el in types_rtrn {
                        sz += el.wire_len();
                    }
                }
                GetMapReplyMap::KeySyms(
                    syms_rtrn,
                    ..
                ) => {
                    for el in syms_rtrn {
                        sz += el.wire_len();
                    }
                }
                GetMapReplyMap::KeyActions{
                    acts_rtrn_count,
                    acts_rtrn_acts,
                    ..
                } => {
                    for el in acts_rtrn_count {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                    for el in acts_rtrn_acts {
                        sz += el.wire_len();
                    }
                }
                GetMapReplyMap::KeyBehaviors(
                    behaviors_rtrn,
                    ..
                ) => {
                    for el in behaviors_rtrn {
                        sz += el.wire_len();
                    }
                }
                GetMapReplyMap::VirtualMods(
                    vmods_rtrn,
                    ..
                ) => {
                    sz += vmods_rtrn.len() * std::mem::size_of::<u8>();
                    sz += base::align_pad(sz, 4);
                }
                GetMapReplyMap::ExplicitComponents(
                    explicit_rtrn,
                    ..
                ) => {
                    for el in explicit_rtrn {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                }
                GetMapReplyMap::ModifierMap(
                    modmap_rtrn,
                    ..
                ) => {
                    for el in modmap_rtrn {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                }
                GetMapReplyMap::VirtualModMap(
                    vmodmap_rtrn,
                    ..
                ) => {
                    for el in vmodmap_rtrn {
                        sz += el.wire_len();
                    }
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                GetMapReplyMap::KeyTypes(
                    types_rtrn,
                    ..
                ) => {
                    for el in types_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetMapReplyMap::KeySyms(
                    syms_rtrn,
                    ..
                ) => {
                    for el in syms_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetMapReplyMap::KeyActions{
                    acts_rtrn_count,
                    acts_rtrn_acts,
                    ..
                } => {
                    for el in acts_rtrn_count {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                    for el in acts_rtrn_acts {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetMapReplyMap::KeyBehaviors(
                    behaviors_rtrn,
                    ..
                ) => {
                    for el in behaviors_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetMapReplyMap::VirtualMods(
                    vmods_rtrn,
                    ..
                ) => {
                    for el in vmods_rtrn {
                        offset += (el.bits() as u8).serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                }
                GetMapReplyMap::ExplicitComponents(
                    explicit_rtrn,
                    ..
                ) => {
                    for el in explicit_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                }
                GetMapReplyMap::ModifierMap(
                    modmap_rtrn,
                    ..
                ) => {
                    for el in modmap_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                }
                GetMapReplyMap::VirtualModMap(
                    vmodmap_rtrn,
                    ..
                ) => {
                    for el in vmodmap_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<GetMapReplyMap> {
    type Params = GetMapReplyMapParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let GetMapReplyMapParams {
            present,
            n_key_actions,
            n_key_syms,
            n_types,
            total_actions,
            total_key_behaviors,
            total_key_explicit,
            total_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
        } = params;
        let expr = (present as usize);
        let mut sz = 0usize;
        if expr & (MapPart::KEY_TYPES.bits() as usize) != 0 {
            // types_rtrn
            for _ in 0 .. (n_types as usize) {
                sz += <&KeyType>::compute_wire_len(ptr.add(sz), ());
            }
        }
        if expr & (MapPart::KEY_SYMS.bits() as usize) != 0 {
            // syms_rtrn
            for _ in 0 .. (n_key_syms as usize) {
                sz += <&KeySymMap>::compute_wire_len(ptr.add(sz), ());
            }
        }
        if expr & (MapPart::KEY_ACTIONS.bits() as usize) != 0 {
            // acts_rtrn_count
            sz += (n_key_actions as usize);
            // align pad
            sz += base::align_pad(sz, 4);
            // acts_rtrn_acts
            sz += ((total_actions as usize) * 8usize);
        }
        if expr & (MapPart::KEY_BEHAVIORS.bits() as usize) != 0 {
            // behaviors_rtrn
            sz += ((total_key_behaviors as usize) * 4usize);
        }
        if expr & (MapPart::VIRTUAL_MODS.bits() as usize) != 0 {
            // vmods_rtrn
            sz += ((virtual_mods as usize).count_ones() as usize);
            // align pad
            sz += base::align_pad(sz, 4);
        }
        if expr & (MapPart::EXPLICIT_COMPONENTS.bits() as usize) != 0 {
            // explicit_rtrn
            sz += ((total_key_explicit as usize) * 2usize);
            // align pad
            sz += base::align_pad(sz, 4);
        }
        if expr & (MapPart::MODIFIER_MAP.bits() as usize) != 0 {
            // modmap_rtrn
            sz += ((total_mod_map_keys as usize) * 2usize);
            // align pad
            sz += base::align_pad(sz, 4);
        }
        if expr & (MapPart::VIRTUAL_MOD_MAP.bits() as usize) != 0 {
            // vmodmap_rtrn
            sz += ((total_v_mod_map_keys as usize) * 4usize);
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: GetMapReplyMapParams, out_offset: &mut usize) -> Vec<GetMapReplyMap> {
        let GetMapReplyMapParams{
            present,
            n_key_actions,
            n_key_syms,
            n_types,
            total_actions,
            total_key_behaviors,
            total_key_explicit,
            total_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
        } = params;
        let expr = (present as usize);
        let mut result = Vec::new();
        if expr & (MapPart::KEY_TYPES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let mut types_rtrn = Vec::new();
            let types_rtrn_params = ();
            for i in 0..(n_types as usize) {
                let el = KeyTypeBuf::unserialize(wire_data.add(offset), types_rtrn_params, &mut offset);
                types_rtrn.push(el);
            }
            *out_offset += offset;
            result.push(GetMapReplyMap::KeyTypes(
                types_rtrn,
            ));
        }
        if expr & (MapPart::KEY_SYMS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let mut syms_rtrn = Vec::new();
            let syms_rtrn_params = ();
            for i in 0..(n_key_syms as usize) {
                let el = KeySymMapBuf::unserialize(wire_data.add(offset), syms_rtrn_params, &mut offset);
                syms_rtrn.push(el);
            }
            *out_offset += offset;
            result.push(GetMapReplyMap::KeySyms(
                syms_rtrn,
            ));
        }
        if expr & (MapPart::KEY_ACTIONS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let acts_rtrn_count = {
                let ptr = wire_data.add(offset) as *const u8;
                let len = (n_key_actions as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<u8>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            let mut acts_rtrn_acts = Vec::new();
            let acts_rtrn_acts_params = ();
            for i in 0..(total_actions as usize) {
                let el = Action::unserialize(wire_data.add(offset), acts_rtrn_acts_params, &mut offset);
                acts_rtrn_acts.push(el);
            }
            *out_offset += offset;
            result.push(GetMapReplyMap::KeyActions{
                acts_rtrn_count,
                acts_rtrn_acts,
            });
        }
        if expr & (MapPart::KEY_BEHAVIORS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let behaviors_rtrn = {
                let ptr = wire_data.add(offset) as *const SetBehavior;
                let len = (total_key_behaviors as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<SetBehavior>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetMapReplyMap::KeyBehaviors(
                behaviors_rtrn,
            ));
        }
        if expr & (MapPart::VIRTUAL_MODS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let vmods_rtrn = {
                let ptr = wire_data.add(offset) as *const xproto::ModMask;
                let len = ((virtual_mods as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::ModMask>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            *out_offset += offset;
            result.push(GetMapReplyMap::VirtualMods(
                vmods_rtrn,
            ));
        }
        if expr & (MapPart::EXPLICIT_COMPONENTS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let explicit_rtrn = {
                let ptr = wire_data.add(offset) as *const SetExplicit;
                let len = (total_key_explicit as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<SetExplicit>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            *out_offset += offset;
            result.push(GetMapReplyMap::ExplicitComponents(
                explicit_rtrn,
            ));
        }
        if expr & (MapPart::MODIFIER_MAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let modmap_rtrn = {
                let ptr = wire_data.add(offset) as *const KeyModMap;
                let len = (total_mod_map_keys as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyModMap>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            *out_offset += offset;
            result.push(GetMapReplyMap::ModifierMap(
                modmap_rtrn,
            ));
        }
        if expr & (MapPart::VIRTUAL_MOD_MAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let vmodmap_rtrn = {
                let ptr = wire_data.add(offset) as *const KeyVModMap;
                let len = (total_v_mod_map_keys as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyVModMap>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetMapReplyMap::VirtualModMap(
                vmodmap_rtrn,
            ));
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SetMapValuesParams {
    pub present: usize,
    pub n_key_actions: usize,
    pub n_key_syms: usize,
    pub n_types: usize,
    pub total_actions: usize,
    pub total_key_behaviors: usize,
    pub total_key_explicit: usize,
    pub total_mod_map_keys: usize,
    pub total_v_mod_map_keys: usize,
    pub virtual_mods: usize,
}

#[derive(Clone, Debug)]
pub enum SetMapValues {
    KeyTypes(Vec<SetKeyTypeBuf>),
    KeySyms(Vec<KeySymMapBuf>),
    KeyActions{
        actions_count: Vec<u8>,
        actions: Vec<Action>,
    },
    KeyBehaviors(Vec<SetBehavior>),
    VirtualMods(Vec<u8>),
    ExplicitComponents(Vec<SetExplicit>),
    ModifierMap(Vec<KeyModMap>),
    VirtualModMap(Vec<KeyVModMap>),
}

impl SetMapValues {
    pub(crate) fn get_mask(slice: &[SetMapValues]) -> MapPart {
        let mut res = MapPart::empty();
        for el in slice {
            match el {
                SetMapValues::KeyTypes{..} => {
                    res |= MapPart::KEY_TYPES;
                }
                SetMapValues::KeySyms{..} => {
                    res |= MapPart::KEY_SYMS;
                }
                SetMapValues::KeyActions{..} => {
                    res |= MapPart::KEY_ACTIONS;
                }
                SetMapValues::KeyBehaviors{..} => {
                    res |= MapPart::KEY_BEHAVIORS;
                }
                SetMapValues::VirtualMods{..} => {
                    res |= MapPart::VIRTUAL_MODS;
                }
                SetMapValues::ExplicitComponents{..} => {
                    res |= MapPart::EXPLICIT_COMPONENTS;
                }
                SetMapValues::ModifierMap{..} => {
                    res |= MapPart::MODIFIER_MAP;
                }
                SetMapValues::VirtualModMap{..} => {
                    res |= MapPart::VIRTUAL_MOD_MAP;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[SetMapValues]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            SetMapValues::KeyTypes{..} => 0,
            SetMapValues::KeySyms{..} => 1,
            SetMapValues::KeyActions{..} => 2,
            SetMapValues::KeyBehaviors{..} => 3,
            SetMapValues::VirtualMods{..} => 4,
            SetMapValues::ExplicitComponents{..} => 5,
            SetMapValues::ModifierMap{..} => 6,
            SetMapValues::VirtualModMap{..} => 7,
        }
    }
}

impl base::WiredOut for &[SetMapValues] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                SetMapValues::KeyTypes(
                    types,
                    ..
                ) => {
                    for el in types {
                        sz += el.wire_len();
                    }
                }
                SetMapValues::KeySyms(
                    syms,
                    ..
                ) => {
                    for el in syms {
                        sz += el.wire_len();
                    }
                }
                SetMapValues::KeyActions{
                    actions_count,
                    actions,
                    ..
                } => {
                    for el in actions_count {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                    for el in actions {
                        sz += el.wire_len();
                    }
                }
                SetMapValues::KeyBehaviors(
                    behaviors,
                    ..
                ) => {
                    for el in behaviors {
                        sz += el.wire_len();
                    }
                }
                SetMapValues::VirtualMods(
                    vmods,
                    ..
                ) => {
                    for el in vmods {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                }
                SetMapValues::ExplicitComponents(
                    explicit,
                    ..
                ) => {
                    for el in explicit {
                        sz += el.wire_len();
                    }
                }
                SetMapValues::ModifierMap(
                    modmap,
                    ..
                ) => {
                    for el in modmap {
                        sz += el.wire_len();
                    }
                }
                SetMapValues::VirtualModMap(
                    vmodmap,
                    ..
                ) => {
                    for el in vmodmap {
                        sz += el.wire_len();
                    }
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                SetMapValues::KeyTypes(
                    types,
                    ..
                ) => {
                    for el in types {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetMapValues::KeySyms(
                    syms,
                    ..
                ) => {
                    for el in syms {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetMapValues::KeyActions{
                    actions_count,
                    actions,
                    ..
                } => {
                    for el in actions_count {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                    for el in actions {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetMapValues::KeyBehaviors(
                    behaviors,
                    ..
                ) => {
                    for el in behaviors {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetMapValues::VirtualMods(
                    vmods,
                    ..
                ) => {
                    for el in vmods {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                }
                SetMapValues::ExplicitComponents(
                    explicit,
                    ..
                ) => {
                    for el in explicit {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetMapValues::ModifierMap(
                    modmap,
                    ..
                ) => {
                    for el in modmap {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetMapValues::VirtualModMap(
                    vmodmap,
                    ..
                ) => {
                    for el in vmodmap {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<SetMapValues> {
    type Params = SetMapValuesParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let SetMapValuesParams {
            present,
            n_key_actions,
            n_key_syms,
            n_types,
            total_actions,
            total_key_behaviors,
            total_key_explicit,
            total_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
        } = params;
        let expr = (present as usize);
        let mut sz = 0usize;
        if expr & (MapPart::KEY_TYPES.bits() as usize) != 0 {
            // types
            for _ in 0 .. (n_types as usize) {
                sz += <&SetKeyType>::compute_wire_len(ptr.add(sz), ());
            }
        }
        if expr & (MapPart::KEY_SYMS.bits() as usize) != 0 {
            // syms
            for _ in 0 .. (n_key_syms as usize) {
                sz += <&KeySymMap>::compute_wire_len(ptr.add(sz), ());
            }
        }
        if expr & (MapPart::KEY_ACTIONS.bits() as usize) != 0 {
            // actions_count
            sz += (n_key_actions as usize);
            // align pad
            sz += base::align_pad(sz, 4);
            // actions
            sz += ((total_actions as usize) * 8usize);
        }
        if expr & (MapPart::KEY_BEHAVIORS.bits() as usize) != 0 {
            // behaviors
            sz += ((total_key_behaviors as usize) * 4usize);
        }
        if expr & (MapPart::VIRTUAL_MODS.bits() as usize) != 0 {
            // vmods
            sz += ((virtual_mods as usize).count_ones() as usize);
            // align pad
            sz += base::align_pad(sz, 4);
        }
        if expr & (MapPart::EXPLICIT_COMPONENTS.bits() as usize) != 0 {
            // explicit
            sz += ((total_key_explicit as usize) * 2usize);
        }
        if expr & (MapPart::MODIFIER_MAP.bits() as usize) != 0 {
            // modmap
            sz += ((total_mod_map_keys as usize) * 2usize);
        }
        if expr & (MapPart::VIRTUAL_MOD_MAP.bits() as usize) != 0 {
            // vmodmap
            sz += ((total_v_mod_map_keys as usize) * 4usize);
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: SetMapValuesParams, out_offset: &mut usize) -> Vec<SetMapValues> {
        let SetMapValuesParams{
            present,
            n_key_actions,
            n_key_syms,
            n_types,
            total_actions,
            total_key_behaviors,
            total_key_explicit,
            total_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
        } = params;
        let expr = (present as usize);
        let mut result = Vec::new();
        if expr & (MapPart::KEY_TYPES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let mut types = Vec::new();
            let types_params = ();
            for i in 0..(n_types as usize) {
                let el = SetKeyTypeBuf::unserialize(wire_data.add(offset), types_params, &mut offset);
                types.push(el);
            }
            *out_offset += offset;
            result.push(SetMapValues::KeyTypes(
                types,
            ));
        }
        if expr & (MapPart::KEY_SYMS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let mut syms = Vec::new();
            let syms_params = ();
            for i in 0..(n_key_syms as usize) {
                let el = KeySymMapBuf::unserialize(wire_data.add(offset), syms_params, &mut offset);
                syms.push(el);
            }
            *out_offset += offset;
            result.push(SetMapValues::KeySyms(
                syms,
            ));
        }
        if expr & (MapPart::KEY_ACTIONS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let actions_count = {
                let ptr = wire_data.add(offset) as *const u8;
                let len = (n_key_actions as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<u8>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            let mut actions = Vec::new();
            let actions_params = ();
            for i in 0..(total_actions as usize) {
                let el = Action::unserialize(wire_data.add(offset), actions_params, &mut offset);
                actions.push(el);
            }
            *out_offset += offset;
            result.push(SetMapValues::KeyActions{
                actions_count,
                actions,
            });
        }
        if expr & (MapPart::KEY_BEHAVIORS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let behaviors = {
                let ptr = wire_data.add(offset) as *const SetBehavior;
                let len = (total_key_behaviors as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<SetBehavior>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetMapValues::KeyBehaviors(
                behaviors,
            ));
        }
        if expr & (MapPart::VIRTUAL_MODS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let vmods = {
                let ptr = wire_data.add(offset) as *const u8;
                let len = ((virtual_mods as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<u8>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            *out_offset += offset;
            result.push(SetMapValues::VirtualMods(
                vmods,
            ));
        }
        if expr & (MapPart::EXPLICIT_COMPONENTS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let explicit = {
                let ptr = wire_data.add(offset) as *const SetExplicit;
                let len = (total_key_explicit as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<SetExplicit>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetMapValues::ExplicitComponents(
                explicit,
            ));
        }
        if expr & (MapPart::MODIFIER_MAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let modmap = {
                let ptr = wire_data.add(offset) as *const KeyModMap;
                let len = (total_mod_map_keys as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyModMap>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetMapValues::ModifierMap(
                modmap,
            ));
        }
        if expr & (MapPart::VIRTUAL_MOD_MAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let vmodmap = {
                let ptr = wire_data.add(offset) as *const KeyVModMap;
                let len = (total_v_mod_map_keys as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyVModMap>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetMapValues::VirtualModMap(
                vmodmap,
            ));
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GetNamesReplyValueListParams {
    pub which: usize,
    pub group_names: usize,
    pub indicators: usize,
    pub n_key_aliases: usize,
    pub n_keys: usize,
    pub n_radio_groups: usize,
    pub n_types: usize,
    pub virtual_mods: usize,
}

#[derive(Clone, Debug)]
pub enum GetNamesReplyValueList {
    Keycodes(xproto::Atom),
    Geometry(xproto::Atom),
    Symbols(xproto::Atom),
    PhysSymbols(xproto::Atom),
    Types(xproto::Atom),
    Compat(xproto::Atom),
    KeyTypeNames(Vec<xproto::Atom>),
    KtLevelNames{
        n_levels_per_type: Vec<u8>,
        kt_level_names: Vec<xproto::Atom>,
    },
    IndicatorNames(Vec<xproto::Atom>),
    VirtualModNames(Vec<xproto::Atom>),
    GroupNames(Vec<xproto::Atom>),
    KeyNames(Vec<KeyName>),
    KeyAliases(Vec<KeyAlias>),
    RgNames(Vec<xproto::Atom>),
}

impl GetNamesReplyValueList {
    pub(crate) fn get_mask(slice: &[GetNamesReplyValueList]) -> NameDetail {
        let mut res = NameDetail::empty();
        for el in slice {
            match el {
                GetNamesReplyValueList::Keycodes{..} => {
                    res |= NameDetail::KEYCODES;
                }
                GetNamesReplyValueList::Geometry{..} => {
                    res |= NameDetail::GEOMETRY;
                }
                GetNamesReplyValueList::Symbols{..} => {
                    res |= NameDetail::SYMBOLS;
                }
                GetNamesReplyValueList::PhysSymbols{..} => {
                    res |= NameDetail::PHYS_SYMBOLS;
                }
                GetNamesReplyValueList::Types{..} => {
                    res |= NameDetail::TYPES;
                }
                GetNamesReplyValueList::Compat{..} => {
                    res |= NameDetail::COMPAT;
                }
                GetNamesReplyValueList::KeyTypeNames{..} => {
                    res |= NameDetail::KEY_TYPE_NAMES;
                }
                GetNamesReplyValueList::KtLevelNames{..} => {
                    res |= NameDetail::KT_LEVEL_NAMES;
                }
                GetNamesReplyValueList::IndicatorNames{..} => {
                    res |= NameDetail::INDICATOR_NAMES;
                }
                GetNamesReplyValueList::VirtualModNames{..} => {
                    res |= NameDetail::VIRTUAL_MOD_NAMES;
                }
                GetNamesReplyValueList::GroupNames{..} => {
                    res |= NameDetail::GROUP_NAMES;
                }
                GetNamesReplyValueList::KeyNames{..} => {
                    res |= NameDetail::KEY_NAMES;
                }
                GetNamesReplyValueList::KeyAliases{..} => {
                    res |= NameDetail::KEY_ALIASES;
                }
                GetNamesReplyValueList::RgNames{..} => {
                    res |= NameDetail::RG_NAMES;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[GetNamesReplyValueList]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            GetNamesReplyValueList::Keycodes{..} => 0,
            GetNamesReplyValueList::Geometry{..} => 1,
            GetNamesReplyValueList::Symbols{..} => 2,
            GetNamesReplyValueList::PhysSymbols{..} => 3,
            GetNamesReplyValueList::Types{..} => 4,
            GetNamesReplyValueList::Compat{..} => 5,
            GetNamesReplyValueList::KeyTypeNames{..} => 6,
            GetNamesReplyValueList::KtLevelNames{..} => 7,
            GetNamesReplyValueList::IndicatorNames{..} => 8,
            GetNamesReplyValueList::VirtualModNames{..} => 9,
            GetNamesReplyValueList::GroupNames{..} => 10,
            GetNamesReplyValueList::KeyNames{..} => 11,
            GetNamesReplyValueList::KeyAliases{..} => 12,
            GetNamesReplyValueList::RgNames{..} => 13,
        }
    }
}

impl base::WiredOut for &[GetNamesReplyValueList] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                GetNamesReplyValueList::Keycodes(
                    ..
                ) => {
                    sz += 4;
                }
                GetNamesReplyValueList::Geometry(
                    ..
                ) => {
                    sz += 4;
                }
                GetNamesReplyValueList::Symbols(
                    ..
                ) => {
                    sz += 4;
                }
                GetNamesReplyValueList::PhysSymbols(
                    ..
                ) => {
                    sz += 4;
                }
                GetNamesReplyValueList::Types(
                    ..
                ) => {
                    sz += 4;
                }
                GetNamesReplyValueList::Compat(
                    ..
                ) => {
                    sz += 4;
                }
                GetNamesReplyValueList::KeyTypeNames(
                    type_names,
                    ..
                ) => {
                    for el in type_names {
                        sz += el.wire_len();
                    }
                }
                GetNamesReplyValueList::KtLevelNames{
                    n_levels_per_type,
                    kt_level_names,
                    ..
                } => {
                    for el in n_levels_per_type {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                    for el in kt_level_names {
                        sz += el.wire_len();
                    }
                }
                GetNamesReplyValueList::IndicatorNames(
                    indicator_names,
                    ..
                ) => {
                    for el in indicator_names {
                        sz += el.wire_len();
                    }
                }
                GetNamesReplyValueList::VirtualModNames(
                    virtual_mod_names,
                    ..
                ) => {
                    for el in virtual_mod_names {
                        sz += el.wire_len();
                    }
                }
                GetNamesReplyValueList::GroupNames(
                    groups,
                    ..
                ) => {
                    for el in groups {
                        sz += el.wire_len();
                    }
                }
                GetNamesReplyValueList::KeyNames(
                    key_names,
                    ..
                ) => {
                    for el in key_names {
                        sz += el.wire_len();
                    }
                }
                GetNamesReplyValueList::KeyAliases(
                    key_aliases,
                    ..
                ) => {
                    for el in key_aliases {
                        sz += el.wire_len();
                    }
                }
                GetNamesReplyValueList::RgNames(
                    radio_group_names,
                    ..
                ) => {
                    for el in radio_group_names {
                        sz += el.wire_len();
                    }
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                GetNamesReplyValueList::Keycodes(
                    keycodes_name,
                    ..
                ) => {
                    offset += keycodes_name.serialize(&mut wire_buf[offset..]);
                }
                GetNamesReplyValueList::Geometry(
                    geometry_name,
                    ..
                ) => {
                    offset += geometry_name.serialize(&mut wire_buf[offset..]);
                }
                GetNamesReplyValueList::Symbols(
                    symbols_name,
                    ..
                ) => {
                    offset += symbols_name.serialize(&mut wire_buf[offset..]);
                }
                GetNamesReplyValueList::PhysSymbols(
                    phys_symbols_name,
                    ..
                ) => {
                    offset += phys_symbols_name.serialize(&mut wire_buf[offset..]);
                }
                GetNamesReplyValueList::Types(
                    types_name,
                    ..
                ) => {
                    offset += types_name.serialize(&mut wire_buf[offset..]);
                }
                GetNamesReplyValueList::Compat(
                    compat_name,
                    ..
                ) => {
                    offset += compat_name.serialize(&mut wire_buf[offset..]);
                }
                GetNamesReplyValueList::KeyTypeNames(
                    type_names,
                    ..
                ) => {
                    for el in type_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetNamesReplyValueList::KtLevelNames{
                    n_levels_per_type,
                    kt_level_names,
                    ..
                } => {
                    for el in n_levels_per_type {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                    for el in kt_level_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetNamesReplyValueList::IndicatorNames(
                    indicator_names,
                    ..
                ) => {
                    for el in indicator_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetNamesReplyValueList::VirtualModNames(
                    virtual_mod_names,
                    ..
                ) => {
                    for el in virtual_mod_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetNamesReplyValueList::GroupNames(
                    groups,
                    ..
                ) => {
                    for el in groups {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetNamesReplyValueList::KeyNames(
                    key_names,
                    ..
                ) => {
                    for el in key_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetNamesReplyValueList::KeyAliases(
                    key_aliases,
                    ..
                ) => {
                    for el in key_aliases {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetNamesReplyValueList::RgNames(
                    radio_group_names,
                    ..
                ) => {
                    for el in radio_group_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<GetNamesReplyValueList> {
    type Params = GetNamesReplyValueListParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let GetNamesReplyValueListParams {
            which,
            group_names,
            indicators,
            n_key_aliases,
            n_keys,
            n_radio_groups,
            n_types,
            virtual_mods,
        } = params;
        let expr = (which as usize);
        let mut sz = 0usize;
        if expr & (NameDetail::KEYCODES.bits() as usize) != 0 {
            // keycodes_name
            sz += 4usize;
        }
        if expr & (NameDetail::GEOMETRY.bits() as usize) != 0 {
            // geometry_name
            sz += 4usize;
        }
        if expr & (NameDetail::SYMBOLS.bits() as usize) != 0 {
            // symbols_name
            sz += 4usize;
        }
        if expr & (NameDetail::PHYS_SYMBOLS.bits() as usize) != 0 {
            // phys_symbols_name
            sz += 4usize;
        }
        if expr & (NameDetail::TYPES.bits() as usize) != 0 {
            // types_name
            sz += 4usize;
        }
        if expr & (NameDetail::COMPAT.bits() as usize) != 0 {
            // compat_name
            sz += 4usize;
        }
        if expr & (NameDetail::KEY_TYPE_NAMES.bits() as usize) != 0 {
            // type_names
            sz += ((n_types as usize) * 4usize);
        }
        if expr & (NameDetail::KT_LEVEL_NAMES.bits() as usize) != 0 {
            // n_levels_per_type
            let n_levels_per_type = {
                let len = (n_types as usize);
                let data = ptr.add(sz) as *const u8;
                sz += len * std::mem::size_of::<u8>();
                std::slice::from_raw_parts(data, len)
            };
            // align pad
            sz += base::align_pad(sz, 4);
            // kt_level_names
            sz += ((n_levels_per_type.iter().sum::<u8>() as usize) * 4usize);
        }
        if expr & (NameDetail::INDICATOR_NAMES.bits() as usize) != 0 {
            // indicator_names
            sz += (((indicators as usize).count_ones() as usize) * 4usize);
        }
        if expr & (NameDetail::VIRTUAL_MOD_NAMES.bits() as usize) != 0 {
            // virtual_mod_names
            sz += (((virtual_mods as usize).count_ones() as usize) * 4usize);
        }
        if expr & (NameDetail::GROUP_NAMES.bits() as usize) != 0 {
            // groups
            sz += (((group_names as usize).count_ones() as usize) * 4usize);
        }
        if expr & (NameDetail::KEY_NAMES.bits() as usize) != 0 {
            // key_names
            sz += ((n_keys as usize) * 4usize);
        }
        if expr & (NameDetail::KEY_ALIASES.bits() as usize) != 0 {
            // key_aliases
            sz += ((n_key_aliases as usize) * 8usize);
        }
        if expr & (NameDetail::RG_NAMES.bits() as usize) != 0 {
            // radio_group_names
            sz += ((n_radio_groups as usize) * 4usize);
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: GetNamesReplyValueListParams, out_offset: &mut usize) -> Vec<GetNamesReplyValueList> {
        let GetNamesReplyValueListParams{
            which,
            group_names,
            indicators,
            n_key_aliases,
            n_keys,
            n_radio_groups,
            n_types,
            virtual_mods,
        } = params;
        let expr = (which as usize);
        let mut result = Vec::new();
        if expr & (NameDetail::KEYCODES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let keycodes_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetNamesReplyValueList::Keycodes(
                keycodes_name,
            ));
        }
        if expr & (NameDetail::GEOMETRY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let geometry_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetNamesReplyValueList::Geometry(
                geometry_name,
            ));
        }
        if expr & (NameDetail::SYMBOLS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let symbols_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetNamesReplyValueList::Symbols(
                symbols_name,
            ));
        }
        if expr & (NameDetail::PHYS_SYMBOLS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let phys_symbols_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetNamesReplyValueList::PhysSymbols(
                phys_symbols_name,
            ));
        }
        if expr & (NameDetail::TYPES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let types_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetNamesReplyValueList::Types(
                types_name,
            ));
        }
        if expr & (NameDetail::COMPAT.bits() as usize) != 0 {
            let mut offset = 0usize;
            let compat_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetNamesReplyValueList::Compat(
                compat_name,
            ));
        }
        if expr & (NameDetail::KEY_TYPE_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let type_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = (n_types as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetNamesReplyValueList::KeyTypeNames(
                type_names,
            ));
        }
        if expr & (NameDetail::KT_LEVEL_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let n_levels_per_type = {
                let ptr = wire_data.add(offset) as *const u8;
                let len = (n_types as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<u8>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            let kt_level_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = (n_levels_per_type.iter().sum::<u8>() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetNamesReplyValueList::KtLevelNames{
                n_levels_per_type,
                kt_level_names,
            });
        }
        if expr & (NameDetail::INDICATOR_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let indicator_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = ((indicators as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetNamesReplyValueList::IndicatorNames(
                indicator_names,
            ));
        }
        if expr & (NameDetail::VIRTUAL_MOD_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let virtual_mod_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = ((virtual_mods as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetNamesReplyValueList::VirtualModNames(
                virtual_mod_names,
            ));
        }
        if expr & (NameDetail::GROUP_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let groups = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = ((group_names as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetNamesReplyValueList::GroupNames(
                groups,
            ));
        }
        if expr & (NameDetail::KEY_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let key_names = {
                let ptr = wire_data.add(offset) as *const KeyName;
                let len = (n_keys as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyName>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetNamesReplyValueList::KeyNames(
                key_names,
            ));
        }
        if expr & (NameDetail::KEY_ALIASES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let key_aliases = {
                let ptr = wire_data.add(offset) as *const KeyAlias;
                let len = (n_key_aliases as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyAlias>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetNamesReplyValueList::KeyAliases(
                key_aliases,
            ));
        }
        if expr & (NameDetail::RG_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let radio_group_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = (n_radio_groups as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetNamesReplyValueList::RgNames(
                radio_group_names,
            ));
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SetNamesValuesParams {
    pub which: usize,
    pub group_names: usize,
    pub indicators: usize,
    pub n_key_aliases: usize,
    pub n_keys: usize,
    pub n_radio_groups: usize,
    pub n_types: usize,
    pub virtual_mods: usize,
}

#[derive(Clone, Debug)]
pub enum SetNamesValues {
    Keycodes(xproto::Atom),
    Geometry(xproto::Atom),
    Symbols(xproto::Atom),
    PhysSymbols(xproto::Atom),
    Types(xproto::Atom),
    Compat(xproto::Atom),
    KeyTypeNames(Vec<xproto::Atom>),
    KtLevelNames{
        n_levels_per_type: Vec<u8>,
        kt_level_names: Vec<xproto::Atom>,
    },
    IndicatorNames(Vec<xproto::Atom>),
    VirtualModNames(Vec<xproto::Atom>),
    GroupNames(Vec<xproto::Atom>),
    KeyNames(Vec<KeyName>),
    KeyAliases(Vec<KeyAlias>),
    RgNames(Vec<xproto::Atom>),
}

impl SetNamesValues {
    pub(crate) fn get_mask(slice: &[SetNamesValues]) -> NameDetail {
        let mut res = NameDetail::empty();
        for el in slice {
            match el {
                SetNamesValues::Keycodes{..} => {
                    res |= NameDetail::KEYCODES;
                }
                SetNamesValues::Geometry{..} => {
                    res |= NameDetail::GEOMETRY;
                }
                SetNamesValues::Symbols{..} => {
                    res |= NameDetail::SYMBOLS;
                }
                SetNamesValues::PhysSymbols{..} => {
                    res |= NameDetail::PHYS_SYMBOLS;
                }
                SetNamesValues::Types{..} => {
                    res |= NameDetail::TYPES;
                }
                SetNamesValues::Compat{..} => {
                    res |= NameDetail::COMPAT;
                }
                SetNamesValues::KeyTypeNames{..} => {
                    res |= NameDetail::KEY_TYPE_NAMES;
                }
                SetNamesValues::KtLevelNames{..} => {
                    res |= NameDetail::KT_LEVEL_NAMES;
                }
                SetNamesValues::IndicatorNames{..} => {
                    res |= NameDetail::INDICATOR_NAMES;
                }
                SetNamesValues::VirtualModNames{..} => {
                    res |= NameDetail::VIRTUAL_MOD_NAMES;
                }
                SetNamesValues::GroupNames{..} => {
                    res |= NameDetail::GROUP_NAMES;
                }
                SetNamesValues::KeyNames{..} => {
                    res |= NameDetail::KEY_NAMES;
                }
                SetNamesValues::KeyAliases{..} => {
                    res |= NameDetail::KEY_ALIASES;
                }
                SetNamesValues::RgNames{..} => {
                    res |= NameDetail::RG_NAMES;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[SetNamesValues]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            SetNamesValues::Keycodes{..} => 0,
            SetNamesValues::Geometry{..} => 1,
            SetNamesValues::Symbols{..} => 2,
            SetNamesValues::PhysSymbols{..} => 3,
            SetNamesValues::Types{..} => 4,
            SetNamesValues::Compat{..} => 5,
            SetNamesValues::KeyTypeNames{..} => 6,
            SetNamesValues::KtLevelNames{..} => 7,
            SetNamesValues::IndicatorNames{..} => 8,
            SetNamesValues::VirtualModNames{..} => 9,
            SetNamesValues::GroupNames{..} => 10,
            SetNamesValues::KeyNames{..} => 11,
            SetNamesValues::KeyAliases{..} => 12,
            SetNamesValues::RgNames{..} => 13,
        }
    }
}

impl base::WiredOut for &[SetNamesValues] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                SetNamesValues::Keycodes(
                    ..
                ) => {
                    sz += 4;
                }
                SetNamesValues::Geometry(
                    ..
                ) => {
                    sz += 4;
                }
                SetNamesValues::Symbols(
                    ..
                ) => {
                    sz += 4;
                }
                SetNamesValues::PhysSymbols(
                    ..
                ) => {
                    sz += 4;
                }
                SetNamesValues::Types(
                    ..
                ) => {
                    sz += 4;
                }
                SetNamesValues::Compat(
                    ..
                ) => {
                    sz += 4;
                }
                SetNamesValues::KeyTypeNames(
                    type_names,
                    ..
                ) => {
                    for el in type_names {
                        sz += el.wire_len();
                    }
                }
                SetNamesValues::KtLevelNames{
                    n_levels_per_type,
                    kt_level_names,
                    ..
                } => {
                    for el in n_levels_per_type {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                    for el in kt_level_names {
                        sz += el.wire_len();
                    }
                }
                SetNamesValues::IndicatorNames(
                    indicator_names,
                    ..
                ) => {
                    for el in indicator_names {
                        sz += el.wire_len();
                    }
                }
                SetNamesValues::VirtualModNames(
                    virtual_mod_names,
                    ..
                ) => {
                    for el in virtual_mod_names {
                        sz += el.wire_len();
                    }
                }
                SetNamesValues::GroupNames(
                    groups,
                    ..
                ) => {
                    for el in groups {
                        sz += el.wire_len();
                    }
                }
                SetNamesValues::KeyNames(
                    key_names,
                    ..
                ) => {
                    for el in key_names {
                        sz += el.wire_len();
                    }
                }
                SetNamesValues::KeyAliases(
                    key_aliases,
                    ..
                ) => {
                    for el in key_aliases {
                        sz += el.wire_len();
                    }
                }
                SetNamesValues::RgNames(
                    radio_group_names,
                    ..
                ) => {
                    for el in radio_group_names {
                        sz += el.wire_len();
                    }
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                SetNamesValues::Keycodes(
                    keycodes_name,
                    ..
                ) => {
                    offset += keycodes_name.serialize(&mut wire_buf[offset..]);
                }
                SetNamesValues::Geometry(
                    geometry_name,
                    ..
                ) => {
                    offset += geometry_name.serialize(&mut wire_buf[offset..]);
                }
                SetNamesValues::Symbols(
                    symbols_name,
                    ..
                ) => {
                    offset += symbols_name.serialize(&mut wire_buf[offset..]);
                }
                SetNamesValues::PhysSymbols(
                    phys_symbols_name,
                    ..
                ) => {
                    offset += phys_symbols_name.serialize(&mut wire_buf[offset..]);
                }
                SetNamesValues::Types(
                    types_name,
                    ..
                ) => {
                    offset += types_name.serialize(&mut wire_buf[offset..]);
                }
                SetNamesValues::Compat(
                    compat_name,
                    ..
                ) => {
                    offset += compat_name.serialize(&mut wire_buf[offset..]);
                }
                SetNamesValues::KeyTypeNames(
                    type_names,
                    ..
                ) => {
                    for el in type_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetNamesValues::KtLevelNames{
                    n_levels_per_type,
                    kt_level_names,
                    ..
                } => {
                    for el in n_levels_per_type {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                    for el in kt_level_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetNamesValues::IndicatorNames(
                    indicator_names,
                    ..
                ) => {
                    for el in indicator_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetNamesValues::VirtualModNames(
                    virtual_mod_names,
                    ..
                ) => {
                    for el in virtual_mod_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetNamesValues::GroupNames(
                    groups,
                    ..
                ) => {
                    for el in groups {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetNamesValues::KeyNames(
                    key_names,
                    ..
                ) => {
                    for el in key_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetNamesValues::KeyAliases(
                    key_aliases,
                    ..
                ) => {
                    for el in key_aliases {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                SetNamesValues::RgNames(
                    radio_group_names,
                    ..
                ) => {
                    for el in radio_group_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<SetNamesValues> {
    type Params = SetNamesValuesParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let SetNamesValuesParams {
            which,
            group_names,
            indicators,
            n_key_aliases,
            n_keys,
            n_radio_groups,
            n_types,
            virtual_mods,
        } = params;
        let expr = (which as usize);
        let mut sz = 0usize;
        if expr & (NameDetail::KEYCODES.bits() as usize) != 0 {
            // keycodes_name
            sz += 4usize;
        }
        if expr & (NameDetail::GEOMETRY.bits() as usize) != 0 {
            // geometry_name
            sz += 4usize;
        }
        if expr & (NameDetail::SYMBOLS.bits() as usize) != 0 {
            // symbols_name
            sz += 4usize;
        }
        if expr & (NameDetail::PHYS_SYMBOLS.bits() as usize) != 0 {
            // phys_symbols_name
            sz += 4usize;
        }
        if expr & (NameDetail::TYPES.bits() as usize) != 0 {
            // types_name
            sz += 4usize;
        }
        if expr & (NameDetail::COMPAT.bits() as usize) != 0 {
            // compat_name
            sz += 4usize;
        }
        if expr & (NameDetail::KEY_TYPE_NAMES.bits() as usize) != 0 {
            // type_names
            sz += ((n_types as usize) * 4usize);
        }
        if expr & (NameDetail::KT_LEVEL_NAMES.bits() as usize) != 0 {
            // n_levels_per_type
            let n_levels_per_type = {
                let len = (n_types as usize);
                let data = ptr.add(sz) as *const u8;
                sz += len * std::mem::size_of::<u8>();
                std::slice::from_raw_parts(data, len)
            };
            // align pad
            sz += base::align_pad(sz, 4);
            // kt_level_names
            sz += ((n_levels_per_type.iter().sum::<u8>() as usize) * 4usize);
        }
        if expr & (NameDetail::INDICATOR_NAMES.bits() as usize) != 0 {
            // indicator_names
            sz += (((indicators as usize).count_ones() as usize) * 4usize);
        }
        if expr & (NameDetail::VIRTUAL_MOD_NAMES.bits() as usize) != 0 {
            // virtual_mod_names
            sz += (((virtual_mods as usize).count_ones() as usize) * 4usize);
        }
        if expr & (NameDetail::GROUP_NAMES.bits() as usize) != 0 {
            // groups
            sz += (((group_names as usize).count_ones() as usize) * 4usize);
        }
        if expr & (NameDetail::KEY_NAMES.bits() as usize) != 0 {
            // key_names
            sz += ((n_keys as usize) * 4usize);
        }
        if expr & (NameDetail::KEY_ALIASES.bits() as usize) != 0 {
            // key_aliases
            sz += ((n_key_aliases as usize) * 8usize);
        }
        if expr & (NameDetail::RG_NAMES.bits() as usize) != 0 {
            // radio_group_names
            sz += ((n_radio_groups as usize) * 4usize);
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: SetNamesValuesParams, out_offset: &mut usize) -> Vec<SetNamesValues> {
        let SetNamesValuesParams{
            which,
            group_names,
            indicators,
            n_key_aliases,
            n_keys,
            n_radio_groups,
            n_types,
            virtual_mods,
        } = params;
        let expr = (which as usize);
        let mut result = Vec::new();
        if expr & (NameDetail::KEYCODES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let keycodes_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(SetNamesValues::Keycodes(
                keycodes_name,
            ));
        }
        if expr & (NameDetail::GEOMETRY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let geometry_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(SetNamesValues::Geometry(
                geometry_name,
            ));
        }
        if expr & (NameDetail::SYMBOLS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let symbols_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(SetNamesValues::Symbols(
                symbols_name,
            ));
        }
        if expr & (NameDetail::PHYS_SYMBOLS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let phys_symbols_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(SetNamesValues::PhysSymbols(
                phys_symbols_name,
            ));
        }
        if expr & (NameDetail::TYPES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let types_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(SetNamesValues::Types(
                types_name,
            ));
        }
        if expr & (NameDetail::COMPAT.bits() as usize) != 0 {
            let mut offset = 0usize;
            let compat_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(SetNamesValues::Compat(
                compat_name,
            ));
        }
        if expr & (NameDetail::KEY_TYPE_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let type_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = (n_types as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetNamesValues::KeyTypeNames(
                type_names,
            ));
        }
        if expr & (NameDetail::KT_LEVEL_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let n_levels_per_type = {
                let ptr = wire_data.add(offset) as *const u8;
                let len = (n_types as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<u8>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            let kt_level_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = (n_levels_per_type.iter().sum::<u8>() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetNamesValues::KtLevelNames{
                n_levels_per_type,
                kt_level_names,
            });
        }
        if expr & (NameDetail::INDICATOR_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let indicator_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = ((indicators as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetNamesValues::IndicatorNames(
                indicator_names,
            ));
        }
        if expr & (NameDetail::VIRTUAL_MOD_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let virtual_mod_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = ((virtual_mods as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetNamesValues::VirtualModNames(
                virtual_mod_names,
            ));
        }
        if expr & (NameDetail::GROUP_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let groups = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = ((group_names as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetNamesValues::GroupNames(
                groups,
            ));
        }
        if expr & (NameDetail::KEY_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let key_names = {
                let ptr = wire_data.add(offset) as *const KeyName;
                let len = (n_keys as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyName>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetNamesValues::KeyNames(
                key_names,
            ));
        }
        if expr & (NameDetail::KEY_ALIASES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let key_aliases = {
                let ptr = wire_data.add(offset) as *const KeyAlias;
                let len = (n_key_aliases as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyAlias>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetNamesValues::KeyAliases(
                key_aliases,
            ));
        }
        if expr & (NameDetail::RG_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let radio_group_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = (n_radio_groups as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(SetNamesValues::RgNames(
                radio_group_names,
            ));
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GetKbdByNameReplyRepliesMapParams {
    pub present: usize,
    pub n_key_actions: usize,
    pub n_key_syms: usize,
    pub n_types: usize,
    pub total_actions: usize,
    pub total_key_behaviors: usize,
    pub total_key_explicit: usize,
    pub total_mod_map_keys: usize,
    pub total_v_mod_map_keys: usize,
    pub virtual_mods: usize,
}

#[derive(Clone, Debug)]
pub enum GetKbdByNameReplyRepliesMap {
    KeyTypes(Vec<KeyTypeBuf>),
    KeySyms(Vec<KeySymMapBuf>),
    KeyActions{
        acts_rtrn_count: Vec<u8>,
        acts_rtrn_acts: Vec<Action>,
    },
    KeyBehaviors(Vec<SetBehavior>),
    VirtualMods(Vec<xproto::ModMask>),
    ExplicitComponents(Vec<SetExplicit>),
    ModifierMap(Vec<KeyModMap>),
    VirtualModMap(Vec<KeyVModMap>),
}

impl GetKbdByNameReplyRepliesMap {
    pub(crate) fn get_mask(slice: &[GetKbdByNameReplyRepliesMap]) -> MapPart {
        let mut res = MapPart::empty();
        for el in slice {
            match el {
                GetKbdByNameReplyRepliesMap::KeyTypes{..} => {
                    res |= MapPart::KEY_TYPES;
                }
                GetKbdByNameReplyRepliesMap::KeySyms{..} => {
                    res |= MapPart::KEY_SYMS;
                }
                GetKbdByNameReplyRepliesMap::KeyActions{..} => {
                    res |= MapPart::KEY_ACTIONS;
                }
                GetKbdByNameReplyRepliesMap::KeyBehaviors{..} => {
                    res |= MapPart::KEY_BEHAVIORS;
                }
                GetKbdByNameReplyRepliesMap::VirtualMods{..} => {
                    res |= MapPart::VIRTUAL_MODS;
                }
                GetKbdByNameReplyRepliesMap::ExplicitComponents{..} => {
                    res |= MapPart::EXPLICIT_COMPONENTS;
                }
                GetKbdByNameReplyRepliesMap::ModifierMap{..} => {
                    res |= MapPart::MODIFIER_MAP;
                }
                GetKbdByNameReplyRepliesMap::VirtualModMap{..} => {
                    res |= MapPart::VIRTUAL_MOD_MAP;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[GetKbdByNameReplyRepliesMap]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            GetKbdByNameReplyRepliesMap::KeyTypes{..} => 0,
            GetKbdByNameReplyRepliesMap::KeySyms{..} => 1,
            GetKbdByNameReplyRepliesMap::KeyActions{..} => 2,
            GetKbdByNameReplyRepliesMap::KeyBehaviors{..} => 3,
            GetKbdByNameReplyRepliesMap::VirtualMods{..} => 4,
            GetKbdByNameReplyRepliesMap::ExplicitComponents{..} => 5,
            GetKbdByNameReplyRepliesMap::ModifierMap{..} => 6,
            GetKbdByNameReplyRepliesMap::VirtualModMap{..} => 7,
        }
    }
}

impl base::WiredOut for &[GetKbdByNameReplyRepliesMap] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                GetKbdByNameReplyRepliesMap::KeyTypes(
                    types_rtrn,
                    ..
                ) => {
                    for el in types_rtrn {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesMap::KeySyms(
                    syms_rtrn,
                    ..
                ) => {
                    for el in syms_rtrn {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesMap::KeyActions{
                    acts_rtrn_count,
                    acts_rtrn_acts,
                    ..
                } => {
                    for el in acts_rtrn_count {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                    for el in acts_rtrn_acts {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesMap::KeyBehaviors(
                    behaviors_rtrn,
                    ..
                ) => {
                    for el in behaviors_rtrn {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesMap::VirtualMods(
                    vmods_rtrn,
                    ..
                ) => {
                    sz += vmods_rtrn.len() * std::mem::size_of::<u8>();
                    sz += base::align_pad(sz, 4);
                }
                GetKbdByNameReplyRepliesMap::ExplicitComponents(
                    explicit_rtrn,
                    ..
                ) => {
                    for el in explicit_rtrn {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                }
                GetKbdByNameReplyRepliesMap::ModifierMap(
                    modmap_rtrn,
                    ..
                ) => {
                    for el in modmap_rtrn {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                }
                GetKbdByNameReplyRepliesMap::VirtualModMap(
                    vmodmap_rtrn,
                    ..
                ) => {
                    for el in vmodmap_rtrn {
                        sz += el.wire_len();
                    }
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                GetKbdByNameReplyRepliesMap::KeyTypes(
                    types_rtrn,
                    ..
                ) => {
                    for el in types_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesMap::KeySyms(
                    syms_rtrn,
                    ..
                ) => {
                    for el in syms_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesMap::KeyActions{
                    acts_rtrn_count,
                    acts_rtrn_acts,
                    ..
                } => {
                    for el in acts_rtrn_count {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                    for el in acts_rtrn_acts {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesMap::KeyBehaviors(
                    behaviors_rtrn,
                    ..
                ) => {
                    for el in behaviors_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesMap::VirtualMods(
                    vmods_rtrn,
                    ..
                ) => {
                    for el in vmods_rtrn {
                        offset += (el.bits() as u8).serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                }
                GetKbdByNameReplyRepliesMap::ExplicitComponents(
                    explicit_rtrn,
                    ..
                ) => {
                    for el in explicit_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                }
                GetKbdByNameReplyRepliesMap::ModifierMap(
                    modmap_rtrn,
                    ..
                ) => {
                    for el in modmap_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                }
                GetKbdByNameReplyRepliesMap::VirtualModMap(
                    vmodmap_rtrn,
                    ..
                ) => {
                    for el in vmodmap_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<GetKbdByNameReplyRepliesMap> {
    type Params = GetKbdByNameReplyRepliesMapParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let GetKbdByNameReplyRepliesMapParams {
            present,
            n_key_actions,
            n_key_syms,
            n_types,
            total_actions,
            total_key_behaviors,
            total_key_explicit,
            total_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
        } = params;
        let expr = (present as usize);
        let mut sz = 0usize;
        if expr & (MapPart::KEY_TYPES.bits() as usize) != 0 {
            // types_rtrn
            for _ in 0 .. (n_types as usize) {
                sz += <&KeyType>::compute_wire_len(ptr.add(sz), ());
            }
        }
        if expr & (MapPart::KEY_SYMS.bits() as usize) != 0 {
            // syms_rtrn
            for _ in 0 .. (n_key_syms as usize) {
                sz += <&KeySymMap>::compute_wire_len(ptr.add(sz), ());
            }
        }
        if expr & (MapPart::KEY_ACTIONS.bits() as usize) != 0 {
            // acts_rtrn_count
            sz += (n_key_actions as usize);
            // align pad
            sz += base::align_pad(sz, 4);
            // acts_rtrn_acts
            sz += ((total_actions as usize) * 8usize);
        }
        if expr & (MapPart::KEY_BEHAVIORS.bits() as usize) != 0 {
            // behaviors_rtrn
            sz += ((total_key_behaviors as usize) * 4usize);
        }
        if expr & (MapPart::VIRTUAL_MODS.bits() as usize) != 0 {
            // vmods_rtrn
            sz += ((virtual_mods as usize).count_ones() as usize);
            // align pad
            sz += base::align_pad(sz, 4);
        }
        if expr & (MapPart::EXPLICIT_COMPONENTS.bits() as usize) != 0 {
            // explicit_rtrn
            sz += ((total_key_explicit as usize) * 2usize);
            // align pad
            sz += base::align_pad(sz, 4);
        }
        if expr & (MapPart::MODIFIER_MAP.bits() as usize) != 0 {
            // modmap_rtrn
            sz += ((total_mod_map_keys as usize) * 2usize);
            // align pad
            sz += base::align_pad(sz, 4);
        }
        if expr & (MapPart::VIRTUAL_MOD_MAP.bits() as usize) != 0 {
            // vmodmap_rtrn
            sz += ((total_v_mod_map_keys as usize) * 4usize);
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: GetKbdByNameReplyRepliesMapParams, out_offset: &mut usize) -> Vec<GetKbdByNameReplyRepliesMap> {
        let GetKbdByNameReplyRepliesMapParams{
            present,
            n_key_actions,
            n_key_syms,
            n_types,
            total_actions,
            total_key_behaviors,
            total_key_explicit,
            total_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
        } = params;
        let expr = (present as usize);
        let mut result = Vec::new();
        if expr & (MapPart::KEY_TYPES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let mut types_rtrn = Vec::new();
            let types_rtrn_params = ();
            for i in 0..(n_types as usize) {
                let el = KeyTypeBuf::unserialize(wire_data.add(offset), types_rtrn_params, &mut offset);
                types_rtrn.push(el);
            }
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesMap::KeyTypes(
                types_rtrn,
            ));
        }
        if expr & (MapPart::KEY_SYMS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let mut syms_rtrn = Vec::new();
            let syms_rtrn_params = ();
            for i in 0..(n_key_syms as usize) {
                let el = KeySymMapBuf::unserialize(wire_data.add(offset), syms_rtrn_params, &mut offset);
                syms_rtrn.push(el);
            }
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesMap::KeySyms(
                syms_rtrn,
            ));
        }
        if expr & (MapPart::KEY_ACTIONS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let acts_rtrn_count = {
                let ptr = wire_data.add(offset) as *const u8;
                let len = (n_key_actions as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<u8>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            let mut acts_rtrn_acts = Vec::new();
            let acts_rtrn_acts_params = ();
            for i in 0..(total_actions as usize) {
                let el = Action::unserialize(wire_data.add(offset), acts_rtrn_acts_params, &mut offset);
                acts_rtrn_acts.push(el);
            }
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesMap::KeyActions{
                acts_rtrn_count,
                acts_rtrn_acts,
            });
        }
        if expr & (MapPart::KEY_BEHAVIORS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let behaviors_rtrn = {
                let ptr = wire_data.add(offset) as *const SetBehavior;
                let len = (total_key_behaviors as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<SetBehavior>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesMap::KeyBehaviors(
                behaviors_rtrn,
            ));
        }
        if expr & (MapPart::VIRTUAL_MODS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let vmods_rtrn = {
                let ptr = wire_data.add(offset) as *const xproto::ModMask;
                let len = ((virtual_mods as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::ModMask>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesMap::VirtualMods(
                vmods_rtrn,
            ));
        }
        if expr & (MapPart::EXPLICIT_COMPONENTS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let explicit_rtrn = {
                let ptr = wire_data.add(offset) as *const SetExplicit;
                let len = (total_key_explicit as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<SetExplicit>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesMap::ExplicitComponents(
                explicit_rtrn,
            ));
        }
        if expr & (MapPart::MODIFIER_MAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let modmap_rtrn = {
                let ptr = wire_data.add(offset) as *const KeyModMap;
                let len = (total_mod_map_keys as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyModMap>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesMap::ModifierMap(
                modmap_rtrn,
            ));
        }
        if expr & (MapPart::VIRTUAL_MOD_MAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let vmodmap_rtrn = {
                let ptr = wire_data.add(offset) as *const KeyVModMap;
                let len = (total_v_mod_map_keys as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyVModMap>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesMap::VirtualModMap(
                vmodmap_rtrn,
            ));
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GetKbdByNameReplyRepliesValueListParams {
    pub which: usize,
    pub group_names: usize,
    pub indicators: usize,
    pub n_key_aliases: usize,
    pub n_keys: usize,
    pub n_radio_groups: usize,
    pub n_types: usize,
    pub virtual_mods: usize,
}

#[derive(Clone, Debug)]
pub enum GetKbdByNameReplyRepliesValueList {
    Keycodes(xproto::Atom),
    Geometry(xproto::Atom),
    Symbols(xproto::Atom),
    PhysSymbols(xproto::Atom),
    Types(xproto::Atom),
    Compat(xproto::Atom),
    KeyTypeNames(Vec<xproto::Atom>),
    KtLevelNames{
        n_levels_per_type: Vec<u8>,
        kt_level_names: Vec<xproto::Atom>,
    },
    IndicatorNames(Vec<xproto::Atom>),
    VirtualModNames(Vec<xproto::Atom>),
    GroupNames(Vec<xproto::Atom>),
    KeyNames(Vec<KeyName>),
    KeyAliases(Vec<KeyAlias>),
    RgNames(Vec<xproto::Atom>),
}

impl GetKbdByNameReplyRepliesValueList {
    pub(crate) fn get_mask(slice: &[GetKbdByNameReplyRepliesValueList]) -> NameDetail {
        let mut res = NameDetail::empty();
        for el in slice {
            match el {
                GetKbdByNameReplyRepliesValueList::Keycodes{..} => {
                    res |= NameDetail::KEYCODES;
                }
                GetKbdByNameReplyRepliesValueList::Geometry{..} => {
                    res |= NameDetail::GEOMETRY;
                }
                GetKbdByNameReplyRepliesValueList::Symbols{..} => {
                    res |= NameDetail::SYMBOLS;
                }
                GetKbdByNameReplyRepliesValueList::PhysSymbols{..} => {
                    res |= NameDetail::PHYS_SYMBOLS;
                }
                GetKbdByNameReplyRepliesValueList::Types{..} => {
                    res |= NameDetail::TYPES;
                }
                GetKbdByNameReplyRepliesValueList::Compat{..} => {
                    res |= NameDetail::COMPAT;
                }
                GetKbdByNameReplyRepliesValueList::KeyTypeNames{..} => {
                    res |= NameDetail::KEY_TYPE_NAMES;
                }
                GetKbdByNameReplyRepliesValueList::KtLevelNames{..} => {
                    res |= NameDetail::KT_LEVEL_NAMES;
                }
                GetKbdByNameReplyRepliesValueList::IndicatorNames{..} => {
                    res |= NameDetail::INDICATOR_NAMES;
                }
                GetKbdByNameReplyRepliesValueList::VirtualModNames{..} => {
                    res |= NameDetail::VIRTUAL_MOD_NAMES;
                }
                GetKbdByNameReplyRepliesValueList::GroupNames{..} => {
                    res |= NameDetail::GROUP_NAMES;
                }
                GetKbdByNameReplyRepliesValueList::KeyNames{..} => {
                    res |= NameDetail::KEY_NAMES;
                }
                GetKbdByNameReplyRepliesValueList::KeyAliases{..} => {
                    res |= NameDetail::KEY_ALIASES;
                }
                GetKbdByNameReplyRepliesValueList::RgNames{..} => {
                    res |= NameDetail::RG_NAMES;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[GetKbdByNameReplyRepliesValueList]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            GetKbdByNameReplyRepliesValueList::Keycodes{..} => 0,
            GetKbdByNameReplyRepliesValueList::Geometry{..} => 1,
            GetKbdByNameReplyRepliesValueList::Symbols{..} => 2,
            GetKbdByNameReplyRepliesValueList::PhysSymbols{..} => 3,
            GetKbdByNameReplyRepliesValueList::Types{..} => 4,
            GetKbdByNameReplyRepliesValueList::Compat{..} => 5,
            GetKbdByNameReplyRepliesValueList::KeyTypeNames{..} => 6,
            GetKbdByNameReplyRepliesValueList::KtLevelNames{..} => 7,
            GetKbdByNameReplyRepliesValueList::IndicatorNames{..} => 8,
            GetKbdByNameReplyRepliesValueList::VirtualModNames{..} => 9,
            GetKbdByNameReplyRepliesValueList::GroupNames{..} => 10,
            GetKbdByNameReplyRepliesValueList::KeyNames{..} => 11,
            GetKbdByNameReplyRepliesValueList::KeyAliases{..} => 12,
            GetKbdByNameReplyRepliesValueList::RgNames{..} => 13,
        }
    }
}

impl base::WiredOut for &[GetKbdByNameReplyRepliesValueList] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                GetKbdByNameReplyRepliesValueList::Keycodes(
                    ..
                ) => {
                    sz += 4;
                }
                GetKbdByNameReplyRepliesValueList::Geometry(
                    ..
                ) => {
                    sz += 4;
                }
                GetKbdByNameReplyRepliesValueList::Symbols(
                    ..
                ) => {
                    sz += 4;
                }
                GetKbdByNameReplyRepliesValueList::PhysSymbols(
                    ..
                ) => {
                    sz += 4;
                }
                GetKbdByNameReplyRepliesValueList::Types(
                    ..
                ) => {
                    sz += 4;
                }
                GetKbdByNameReplyRepliesValueList::Compat(
                    ..
                ) => {
                    sz += 4;
                }
                GetKbdByNameReplyRepliesValueList::KeyTypeNames(
                    type_names,
                    ..
                ) => {
                    for el in type_names {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesValueList::KtLevelNames{
                    n_levels_per_type,
                    kt_level_names,
                    ..
                } => {
                    for el in n_levels_per_type {
                        sz += el.wire_len();
                    }
                    sz += base::align_pad(sz, 4);
                    for el in kt_level_names {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesValueList::IndicatorNames(
                    indicator_names,
                    ..
                ) => {
                    for el in indicator_names {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesValueList::VirtualModNames(
                    virtual_mod_names,
                    ..
                ) => {
                    for el in virtual_mod_names {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesValueList::GroupNames(
                    groups,
                    ..
                ) => {
                    for el in groups {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesValueList::KeyNames(
                    key_names,
                    ..
                ) => {
                    for el in key_names {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesValueList::KeyAliases(
                    key_aliases,
                    ..
                ) => {
                    for el in key_aliases {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyRepliesValueList::RgNames(
                    radio_group_names,
                    ..
                ) => {
                    for el in radio_group_names {
                        sz += el.wire_len();
                    }
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                GetKbdByNameReplyRepliesValueList::Keycodes(
                    keycodes_name,
                    ..
                ) => {
                    offset += keycodes_name.serialize(&mut wire_buf[offset..]);
                }
                GetKbdByNameReplyRepliesValueList::Geometry(
                    geometry_name,
                    ..
                ) => {
                    offset += geometry_name.serialize(&mut wire_buf[offset..]);
                }
                GetKbdByNameReplyRepliesValueList::Symbols(
                    symbols_name,
                    ..
                ) => {
                    offset += symbols_name.serialize(&mut wire_buf[offset..]);
                }
                GetKbdByNameReplyRepliesValueList::PhysSymbols(
                    phys_symbols_name,
                    ..
                ) => {
                    offset += phys_symbols_name.serialize(&mut wire_buf[offset..]);
                }
                GetKbdByNameReplyRepliesValueList::Types(
                    types_name,
                    ..
                ) => {
                    offset += types_name.serialize(&mut wire_buf[offset..]);
                }
                GetKbdByNameReplyRepliesValueList::Compat(
                    compat_name,
                    ..
                ) => {
                    offset += compat_name.serialize(&mut wire_buf[offset..]);
                }
                GetKbdByNameReplyRepliesValueList::KeyTypeNames(
                    type_names,
                    ..
                ) => {
                    for el in type_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesValueList::KtLevelNames{
                    n_levels_per_type,
                    kt_level_names,
                    ..
                } => {
                    for el in n_levels_per_type {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    offset += base::align_pad(offset, 4);
                    for el in kt_level_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesValueList::IndicatorNames(
                    indicator_names,
                    ..
                ) => {
                    for el in indicator_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesValueList::VirtualModNames(
                    virtual_mod_names,
                    ..
                ) => {
                    for el in virtual_mod_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesValueList::GroupNames(
                    groups,
                    ..
                ) => {
                    for el in groups {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesValueList::KeyNames(
                    key_names,
                    ..
                ) => {
                    for el in key_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesValueList::KeyAliases(
                    key_aliases,
                    ..
                ) => {
                    for el in key_aliases {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyRepliesValueList::RgNames(
                    radio_group_names,
                    ..
                ) => {
                    for el in radio_group_names {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<GetKbdByNameReplyRepliesValueList> {
    type Params = GetKbdByNameReplyRepliesValueListParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let GetKbdByNameReplyRepliesValueListParams {
            which,
            group_names,
            indicators,
            n_key_aliases,
            n_keys,
            n_radio_groups,
            n_types,
            virtual_mods,
        } = params;
        let expr = (which as usize);
        let mut sz = 0usize;
        if expr & (NameDetail::KEYCODES.bits() as usize) != 0 {
            // keycodes_name
            sz += 4usize;
        }
        if expr & (NameDetail::GEOMETRY.bits() as usize) != 0 {
            // geometry_name
            sz += 4usize;
        }
        if expr & (NameDetail::SYMBOLS.bits() as usize) != 0 {
            // symbols_name
            sz += 4usize;
        }
        if expr & (NameDetail::PHYS_SYMBOLS.bits() as usize) != 0 {
            // phys_symbols_name
            sz += 4usize;
        }
        if expr & (NameDetail::TYPES.bits() as usize) != 0 {
            // types_name
            sz += 4usize;
        }
        if expr & (NameDetail::COMPAT.bits() as usize) != 0 {
            // compat_name
            sz += 4usize;
        }
        if expr & (NameDetail::KEY_TYPE_NAMES.bits() as usize) != 0 {
            // type_names
            sz += ((n_types as usize) * 4usize);
        }
        if expr & (NameDetail::KT_LEVEL_NAMES.bits() as usize) != 0 {
            // n_levels_per_type
            let n_levels_per_type = {
                let len = (n_types as usize);
                let data = ptr.add(sz) as *const u8;
                sz += len * std::mem::size_of::<u8>();
                std::slice::from_raw_parts(data, len)
            };
            // align pad
            sz += base::align_pad(sz, 4);
            // kt_level_names
            sz += ((n_levels_per_type.iter().sum::<u8>() as usize) * 4usize);
        }
        if expr & (NameDetail::INDICATOR_NAMES.bits() as usize) != 0 {
            // indicator_names
            sz += (((indicators as usize).count_ones() as usize) * 4usize);
        }
        if expr & (NameDetail::VIRTUAL_MOD_NAMES.bits() as usize) != 0 {
            // virtual_mod_names
            sz += (((virtual_mods as usize).count_ones() as usize) * 4usize);
        }
        if expr & (NameDetail::GROUP_NAMES.bits() as usize) != 0 {
            // groups
            sz += (((group_names as usize).count_ones() as usize) * 4usize);
        }
        if expr & (NameDetail::KEY_NAMES.bits() as usize) != 0 {
            // key_names
            sz += ((n_keys as usize) * 4usize);
        }
        if expr & (NameDetail::KEY_ALIASES.bits() as usize) != 0 {
            // key_aliases
            sz += ((n_key_aliases as usize) * 8usize);
        }
        if expr & (NameDetail::RG_NAMES.bits() as usize) != 0 {
            // radio_group_names
            sz += ((n_radio_groups as usize) * 4usize);
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: GetKbdByNameReplyRepliesValueListParams, out_offset: &mut usize) -> Vec<GetKbdByNameReplyRepliesValueList> {
        let GetKbdByNameReplyRepliesValueListParams{
            which,
            group_names,
            indicators,
            n_key_aliases,
            n_keys,
            n_radio_groups,
            n_types,
            virtual_mods,
        } = params;
        let expr = (which as usize);
        let mut result = Vec::new();
        if expr & (NameDetail::KEYCODES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let keycodes_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::Keycodes(
                keycodes_name,
            ));
        }
        if expr & (NameDetail::GEOMETRY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let geometry_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::Geometry(
                geometry_name,
            ));
        }
        if expr & (NameDetail::SYMBOLS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let symbols_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::Symbols(
                symbols_name,
            ));
        }
        if expr & (NameDetail::PHYS_SYMBOLS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let phys_symbols_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::PhysSymbols(
                phys_symbols_name,
            ));
        }
        if expr & (NameDetail::TYPES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let types_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::Types(
                types_name,
            ));
        }
        if expr & (NameDetail::COMPAT.bits() as usize) != 0 {
            let mut offset = 0usize;
            let compat_name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::Compat(
                compat_name,
            ));
        }
        if expr & (NameDetail::KEY_TYPE_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let type_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = (n_types as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::KeyTypeNames(
                type_names,
            ));
        }
        if expr & (NameDetail::KT_LEVEL_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let n_levels_per_type = {
                let ptr = wire_data.add(offset) as *const u8;
                let len = (n_types as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<u8>();
                data.to_vec()
            };
            offset += base::align_pad(offset, 4);
            let kt_level_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = (n_levels_per_type.iter().sum::<u8>() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::KtLevelNames{
                n_levels_per_type,
                kt_level_names,
            });
        }
        if expr & (NameDetail::INDICATOR_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let indicator_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = ((indicators as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::IndicatorNames(
                indicator_names,
            ));
        }
        if expr & (NameDetail::VIRTUAL_MOD_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let virtual_mod_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = ((virtual_mods as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::VirtualModNames(
                virtual_mod_names,
            ));
        }
        if expr & (NameDetail::GROUP_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let groups = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = ((group_names as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::GroupNames(
                groups,
            ));
        }
        if expr & (NameDetail::KEY_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let key_names = {
                let ptr = wire_data.add(offset) as *const KeyName;
                let len = (n_keys as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyName>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::KeyNames(
                key_names,
            ));
        }
        if expr & (NameDetail::KEY_ALIASES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let key_aliases = {
                let ptr = wire_data.add(offset) as *const KeyAlias;
                let len = (n_key_aliases as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<KeyAlias>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::KeyAliases(
                key_aliases,
            ));
        }
        if expr & (NameDetail::RG_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let radio_group_names = {
                let ptr = wire_data.add(offset) as *const xproto::Atom;
                let len = (n_radio_groups as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<xproto::Atom>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyRepliesValueList::RgNames(
                radio_group_names,
            ));
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GetKbdByNameReplyRepliesParams {
    pub reported: usize,
}

#[derive(Clone, Debug)]
pub enum GetKbdByNameReplyReplies {
    Types{
        getmap_type: u8,
        type_device_id: u8,
        getmap_sequence: u16,
        getmap_length: u32,
        type_min_key_code: xproto::Keycode,
        type_max_key_code: xproto::Keycode,
        first_type: u8,
        n_types: u8,
        total_types: u8,
        first_key_sym: xproto::Keycode,
        total_syms: u16,
        n_key_syms: u8,
        first_key_action: xproto::Keycode,
        total_actions: u16,
        n_key_actions: u8,
        first_key_behavior: xproto::Keycode,
        n_key_behaviors: u8,
        total_key_behaviors: u8,
        first_key_explicit: xproto::Keycode,
        n_key_explicit: u8,
        total_key_explicit: u8,
        first_mod_map_key: xproto::Keycode,
        n_mod_map_keys: u8,
        total_mod_map_keys: u8,
        first_v_mod_map_key: xproto::Keycode,
        n_v_mod_map_keys: u8,
        total_v_mod_map_keys: u8,
        virtual_mods: VMod,
        map: Vec<GetKbdByNameReplyRepliesMap>,
    },
    CompatMap{
        compatmap_type: u8,
        compat_device_id: u8,
        compatmap_sequence: u16,
        compatmap_length: u32,
        groups_rtrn: SetOfGroup,
        first_si_rtrn: u16,
        n_total_si: u16,
        si_rtrn: Vec<SymInterpret>,
        group_rtrn: Vec<ModDef>,
    },
    IndicatorMaps{
        indicatormap_type: u8,
        indicator_device_id: u8,
        indicatormap_sequence: u16,
        indicatormap_length: u32,
        which: u32,
        real_indicators: u32,
        maps: Vec<IndicatorMap>,
    },
    KeyNames{
        keyname_type: u8,
        key_device_id: u8,
        keyname_sequence: u16,
        keyname_length: u32,
        key_min_key_code: xproto::Keycode,
        key_max_key_code: xproto::Keycode,
        n_types: u8,
        group_names: SetOfGroup,
        virtual_mods: VMod,
        first_key: xproto::Keycode,
        n_keys: u8,
        indicators: u32,
        n_radio_groups: u8,
        n_key_aliases: u8,
        n_kt_levels: u16,
        value_list: Vec<GetKbdByNameReplyRepliesValueList>,
    },
    Geometry{
        geometry_type: u8,
        geometry_device_id: u8,
        geometry_sequence: u16,
        geometry_length: u32,
        name: xproto::Atom,
        geometry_found: bool,
        width_mm: u16,
        height_mm: u16,
        n_properties: u16,
        n_colors: u16,
        n_shapes: u16,
        n_sections: u16,
        n_doodads: u16,
        n_key_aliases: u16,
        base_color_ndx: u8,
        label_color_ndx: u8,
        label_font: CountedString16Buf,
    },
}

impl GetKbdByNameReplyReplies {
    pub(crate) fn get_mask(slice: &[GetKbdByNameReplyReplies]) -> GbnDetail {
        let mut res = GbnDetail::empty();
        for el in slice {
            match el {
                GetKbdByNameReplyReplies::Types{..} => {
                    res |= GbnDetail::TYPES;
                    res |= GbnDetail::CLIENT_SYMBOLS;
                    res |= GbnDetail::SERVER_SYMBOLS;
                }
                GetKbdByNameReplyReplies::CompatMap{..} => {
                    res |= GbnDetail::COMPAT_MAP;
                }
                GetKbdByNameReplyReplies::IndicatorMaps{..} => {
                    res |= GbnDetail::INDICATOR_MAPS;
                }
                GetKbdByNameReplyReplies::KeyNames{..} => {
                    res |= GbnDetail::KEY_NAMES;
                    res |= GbnDetail::OTHER_NAMES;
                }
                GetKbdByNameReplyReplies::Geometry{..} => {
                    res |= GbnDetail::GEOMETRY;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[GetKbdByNameReplyReplies]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            GetKbdByNameReplyReplies::Types{..} => 0,
            GetKbdByNameReplyReplies::CompatMap{..} => 1,
            GetKbdByNameReplyReplies::IndicatorMaps{..} => 2,
            GetKbdByNameReplyReplies::KeyNames{..} => 3,
            GetKbdByNameReplyReplies::Geometry{..} => 4,
        }
    }
}

impl base::WiredOut for &[GetKbdByNameReplyReplies] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                GetKbdByNameReplyReplies::Types{
                    n_types,
                    n_key_syms,
                    total_actions,
                    n_key_actions,
                    total_key_behaviors,
                    total_key_explicit,
                    total_mod_map_keys,
                    total_v_mod_map_keys,
                    virtual_mods,
                    map,
                    ..
                } => {
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 4;
                    sz += 2;
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += map.as_slice().wire_len();
                }
                GetKbdByNameReplyReplies::CompatMap{
                    groups_rtrn,
                    si_rtrn,
                    group_rtrn,
                    ..
                } => {
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 4;
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 2;
                    sz += 2;
                    sz += 16;
                    for el in si_rtrn {
                        sz += el.wire_len();
                    }
                    for el in group_rtrn {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyReplies::IndicatorMaps{
                    maps,
                    ..
                } => {
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 4;
                    sz += 4;
                    sz += 4;
                    sz += 1;
                    sz += 15;
                    for el in maps {
                        sz += el.wire_len();
                    }
                }
                GetKbdByNameReplyReplies::KeyNames{
                    n_types,
                    group_names,
                    virtual_mods,
                    n_keys,
                    indicators,
                    n_radio_groups,
                    n_key_aliases,
                    value_list,
                    ..
                } => {
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 4;
                    sz += 4;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 1;
                    sz += 1;
                    sz += 4;
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 4;
                    sz += value_list.as_slice().wire_len();
                }
                GetKbdByNameReplyReplies::Geometry{
                    label_font,
                    ..
                } => {
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 4;
                    sz += 4;
                    sz += 1;
                    sz += 1;
                    sz += 2;
                    sz += 2;
                    sz += 2;
                    sz += 2;
                    sz += 2;
                    sz += 2;
                    sz += 2;
                    sz += 2;
                    sz += 1;
                    sz += 1;
                    sz += label_font.wire_len();
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                GetKbdByNameReplyReplies::Types{
                    getmap_type,
                    type_device_id,
                    getmap_sequence,
                    getmap_length,
                    type_min_key_code,
                    type_max_key_code,
                    first_type,
                    n_types,
                    total_types,
                    first_key_sym,
                    total_syms,
                    n_key_syms,
                    first_key_action,
                    total_actions,
                    n_key_actions,
                    first_key_behavior,
                    n_key_behaviors,
                    total_key_behaviors,
                    first_key_explicit,
                    n_key_explicit,
                    total_key_explicit,
                    first_mod_map_key,
                    n_mod_map_keys,
                    total_mod_map_keys,
                    first_v_mod_map_key,
                    n_v_mod_map_keys,
                    total_v_mod_map_keys,
                    virtual_mods,
                    map,
                    ..
                } => {
                    offset += getmap_type.serialize(&mut wire_buf[offset..]);
                    offset += type_device_id.serialize(&mut wire_buf[offset..]);
                    offset += getmap_sequence.serialize(&mut wire_buf[offset..]);
                    offset += getmap_length.serialize(&mut wire_buf[offset..]);
                    offset += 2;
                    offset += type_min_key_code.serialize(&mut wire_buf[offset..]);
                    offset += type_max_key_code.serialize(&mut wire_buf[offset..]);
                    offset += (GetKbdByNameReplyRepliesMap::get_mask(map).bits() as u16).serialize(&mut wire_buf[offset..]);
                    offset += first_type.serialize(&mut wire_buf[offset..]);
                    offset += n_types.serialize(&mut wire_buf[offset..]);
                    offset += total_types.serialize(&mut wire_buf[offset..]);
                    offset += first_key_sym.serialize(&mut wire_buf[offset..]);
                    offset += total_syms.serialize(&mut wire_buf[offset..]);
                    offset += n_key_syms.serialize(&mut wire_buf[offset..]);
                    offset += first_key_action.serialize(&mut wire_buf[offset..]);
                    offset += total_actions.serialize(&mut wire_buf[offset..]);
                    offset += n_key_actions.serialize(&mut wire_buf[offset..]);
                    offset += first_key_behavior.serialize(&mut wire_buf[offset..]);
                    offset += n_key_behaviors.serialize(&mut wire_buf[offset..]);
                    offset += total_key_behaviors.serialize(&mut wire_buf[offset..]);
                    offset += first_key_explicit.serialize(&mut wire_buf[offset..]);
                    offset += n_key_explicit.serialize(&mut wire_buf[offset..]);
                    offset += total_key_explicit.serialize(&mut wire_buf[offset..]);
                    offset += first_mod_map_key.serialize(&mut wire_buf[offset..]);
                    offset += n_mod_map_keys.serialize(&mut wire_buf[offset..]);
                    offset += total_mod_map_keys.serialize(&mut wire_buf[offset..]);
                    offset += first_v_mod_map_key.serialize(&mut wire_buf[offset..]);
                    offset += n_v_mod_map_keys.serialize(&mut wire_buf[offset..]);
                    offset += total_v_mod_map_keys.serialize(&mut wire_buf[offset..]);
                    offset += 1;
                    offset += (virtual_mods.bits() as u16).serialize(&mut wire_buf[offset..]);
                    offset += map.as_slice().serialize(&mut wire_buf[offset..]);
                }
                GetKbdByNameReplyReplies::CompatMap{
                    compatmap_type,
                    compat_device_id,
                    compatmap_sequence,
                    compatmap_length,
                    groups_rtrn,
                    first_si_rtrn,
                    n_total_si,
                    si_rtrn,
                    group_rtrn,
                    ..
                } => {
                    offset += compatmap_type.serialize(&mut wire_buf[offset..]);
                    offset += compat_device_id.serialize(&mut wire_buf[offset..]);
                    offset += compatmap_sequence.serialize(&mut wire_buf[offset..]);
                    offset += compatmap_length.serialize(&mut wire_buf[offset..]);
                    offset += (groups_rtrn.bits() as u8).serialize(&mut wire_buf[offset..]);
                    offset += 1;
                    offset += first_si_rtrn.serialize(&mut wire_buf[offset..]);
                    offset += (si_rtrn.len() as u16).serialize(&mut wire_buf[offset..]);
                    offset += n_total_si.serialize(&mut wire_buf[offset..]);
                    offset += 16;
                    for el in si_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                    for el in group_rtrn {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyReplies::IndicatorMaps{
                    indicatormap_type,
                    indicator_device_id,
                    indicatormap_sequence,
                    indicatormap_length,
                    which,
                    real_indicators,
                    maps,
                    ..
                } => {
                    offset += indicatormap_type.serialize(&mut wire_buf[offset..]);
                    offset += indicator_device_id.serialize(&mut wire_buf[offset..]);
                    offset += indicatormap_sequence.serialize(&mut wire_buf[offset..]);
                    offset += indicatormap_length.serialize(&mut wire_buf[offset..]);
                    offset += which.serialize(&mut wire_buf[offset..]);
                    offset += real_indicators.serialize(&mut wire_buf[offset..]);
                    offset += (maps.len() as u8).serialize(&mut wire_buf[offset..]);
                    offset += 15;
                    for el in maps {
                        offset += el.serialize(&mut wire_buf[offset..]);
                    }
                }
                GetKbdByNameReplyReplies::KeyNames{
                    keyname_type,
                    key_device_id,
                    keyname_sequence,
                    keyname_length,
                    key_min_key_code,
                    key_max_key_code,
                    n_types,
                    group_names,
                    virtual_mods,
                    first_key,
                    n_keys,
                    indicators,
                    n_radio_groups,
                    n_key_aliases,
                    n_kt_levels,
                    value_list,
                    ..
                } => {
                    offset += keyname_type.serialize(&mut wire_buf[offset..]);
                    offset += key_device_id.serialize(&mut wire_buf[offset..]);
                    offset += keyname_sequence.serialize(&mut wire_buf[offset..]);
                    offset += keyname_length.serialize(&mut wire_buf[offset..]);
                    offset += (GetKbdByNameReplyRepliesValueList::get_mask(value_list).bits() as u32).serialize(&mut wire_buf[offset..]);
                    offset += key_min_key_code.serialize(&mut wire_buf[offset..]);
                    offset += key_max_key_code.serialize(&mut wire_buf[offset..]);
                    offset += n_types.serialize(&mut wire_buf[offset..]);
                    offset += (group_names.bits() as u8).serialize(&mut wire_buf[offset..]);
                    offset += (virtual_mods.bits() as u16).serialize(&mut wire_buf[offset..]);
                    offset += first_key.serialize(&mut wire_buf[offset..]);
                    offset += n_keys.serialize(&mut wire_buf[offset..]);
                    offset += indicators.serialize(&mut wire_buf[offset..]);
                    offset += n_radio_groups.serialize(&mut wire_buf[offset..]);
                    offset += n_key_aliases.serialize(&mut wire_buf[offset..]);
                    offset += n_kt_levels.serialize(&mut wire_buf[offset..]);
                    offset += 4;
                    offset += value_list.as_slice().serialize(&mut wire_buf[offset..]);
                }
                GetKbdByNameReplyReplies::Geometry{
                    geometry_type,
                    geometry_device_id,
                    geometry_sequence,
                    geometry_length,
                    name,
                    geometry_found,
                    width_mm,
                    height_mm,
                    n_properties,
                    n_colors,
                    n_shapes,
                    n_sections,
                    n_doodads,
                    n_key_aliases,
                    base_color_ndx,
                    label_color_ndx,
                    label_font,
                    ..
                } => {
                    offset += geometry_type.serialize(&mut wire_buf[offset..]);
                    offset += geometry_device_id.serialize(&mut wire_buf[offset..]);
                    offset += geometry_sequence.serialize(&mut wire_buf[offset..]);
                    offset += geometry_length.serialize(&mut wire_buf[offset..]);
                    offset += name.serialize(&mut wire_buf[offset..]);
                    let geometry_found: u8 = if *geometry_found { 1 } else { 0 };
                    offset += geometry_found.serialize(&mut wire_buf[offset..]);
                    offset += 1;
                    offset += width_mm.serialize(&mut wire_buf[offset..]);
                    offset += height_mm.serialize(&mut wire_buf[offset..]);
                    offset += n_properties.serialize(&mut wire_buf[offset..]);
                    offset += n_colors.serialize(&mut wire_buf[offset..]);
                    offset += n_shapes.serialize(&mut wire_buf[offset..]);
                    offset += n_sections.serialize(&mut wire_buf[offset..]);
                    offset += n_doodads.serialize(&mut wire_buf[offset..]);
                    offset += n_key_aliases.serialize(&mut wire_buf[offset..]);
                    offset += base_color_ndx.serialize(&mut wire_buf[offset..]);
                    offset += label_color_ndx.serialize(&mut wire_buf[offset..]);
                    offset += label_font.serialize(&mut wire_buf[offset..]);
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<GetKbdByNameReplyReplies> {
    type Params = GetKbdByNameReplyRepliesParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let GetKbdByNameReplyRepliesParams {
            reported,
        } = params;
        let expr = (reported as usize);
        let mut sz = 0usize;
        if expr & (GbnDetail::TYPES.bits() as usize) | (GbnDetail::CLIENT_SYMBOLS.bits() as usize) | (GbnDetail::SERVER_SYMBOLS.bits() as usize) != 0 {
            // getmap_type
            sz += 1usize;
            // type_device_id
            sz += 1usize;
            // getmap_sequence
            sz += 2usize;
            // getmap_length
            sz += 4usize;
            // pad
            sz += 2usize;
            // type_min_key_code
            sz += 1usize;
            // type_max_key_code
            sz += 1usize;
            // present
            let present = *(ptr.add(sz) as *const u16);
            sz += 2usize;
            // first_type
            sz += 1usize;
            // n_types
            let n_types = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // total_types
            sz += 1usize;
            // first_key_sym
            sz += 1usize;
            // total_syms
            sz += 2usize;
            // n_key_syms
            let n_key_syms = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // first_key_action
            sz += 1usize;
            // total_actions
            let total_actions = *(ptr.add(sz) as *const u16);
            sz += 2usize;
            // n_key_actions
            let n_key_actions = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // first_key_behavior
            sz += 1usize;
            // n_key_behaviors
            sz += 1usize;
            // total_key_behaviors
            let total_key_behaviors = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // first_key_explicit
            sz += 1usize;
            // n_key_explicit
            sz += 1usize;
            // total_key_explicit
            let total_key_explicit = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // first_mod_map_key
            sz += 1usize;
            // n_mod_map_keys
            sz += 1usize;
            // total_mod_map_keys
            let total_mod_map_keys = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // first_v_mod_map_key
            sz += 1usize;
            // n_v_mod_map_keys
            sz += 1usize;
            // total_v_mod_map_keys
            let total_v_mod_map_keys = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // pad
            sz += 1usize;
            // virtual_mods
            let virtual_mods = *(ptr.add(sz) as *const u16);
            sz += 2usize;
            // map
            sz += <Vec<GetKbdByNameReplyRepliesMap>>::compute_wire_len(ptr.add(sz), GetKbdByNameReplyRepliesMapParams {present: present as usize, n_key_actions: n_key_actions as usize, n_key_syms: n_key_syms as usize, n_types: n_types as usize, total_actions: total_actions as usize, total_key_behaviors: total_key_behaviors as usize, total_key_explicit: total_key_explicit as usize, total_mod_map_keys: total_mod_map_keys as usize, total_v_mod_map_keys: total_v_mod_map_keys as usize, virtual_mods: virtual_mods as usize});
        }
        if expr & (GbnDetail::COMPAT_MAP.bits() as usize) != 0 {
            // compatmap_type
            sz += 1usize;
            // compat_device_id
            sz += 1usize;
            // compatmap_sequence
            sz += 2usize;
            // compatmap_length
            sz += 4usize;
            // groups_rtrn
            let groups_rtrn = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // pad
            sz += 1usize;
            // first_si_rtrn
            sz += 2usize;
            // n_si_rtrn
            let n_si_rtrn = *(ptr.add(sz) as *const u16);
            sz += 2usize;
            // n_total_si
            sz += 2usize;
            // pad
            sz += 16usize;
            // si_rtrn
            sz += ((n_si_rtrn as usize) * 16usize);
            // group_rtrn
            sz += (((groups_rtrn as usize).count_ones() as usize) * 4usize);
        }
        if expr & (GbnDetail::INDICATOR_MAPS.bits() as usize) != 0 {
            // indicatormap_type
            sz += 1usize;
            // indicator_device_id
            sz += 1usize;
            // indicatormap_sequence
            sz += 2usize;
            // indicatormap_length
            sz += 4usize;
            // which
            sz += 4usize;
            // real_indicators
            sz += 4usize;
            // n_indicators
            let n_indicators = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // pad
            sz += 15usize;
            // maps
            sz += ((n_indicators as usize) * 12usize);
        }
        if expr & (GbnDetail::KEY_NAMES.bits() as usize) | (GbnDetail::OTHER_NAMES.bits() as usize) != 0 {
            // keyname_type
            sz += 1usize;
            // key_device_id
            sz += 1usize;
            // keyname_sequence
            sz += 2usize;
            // keyname_length
            sz += 4usize;
            // which
            let which = *(ptr.add(sz) as *const u32);
            sz += 4usize;
            // key_min_key_code
            sz += 1usize;
            // key_max_key_code
            sz += 1usize;
            // n_types
            let n_types = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // group_names
            let group_names = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // virtual_mods
            let virtual_mods = *(ptr.add(sz) as *const u16);
            sz += 2usize;
            // first_key
            sz += 1usize;
            // n_keys
            let n_keys = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // indicators
            let indicators = *(ptr.add(sz) as *const u32);
            sz += 4usize;
            // n_radio_groups
            let n_radio_groups = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // n_key_aliases
            let n_key_aliases = *(ptr.add(sz) as *const u8);
            sz += 1usize;
            // n_kt_levels
            sz += 2usize;
            // pad
            sz += 4usize;
            // value_list
            sz += <Vec<GetKbdByNameReplyRepliesValueList>>::compute_wire_len(ptr.add(sz), GetKbdByNameReplyRepliesValueListParams {which: which as usize, group_names: group_names as usize, indicators: indicators as usize, n_key_aliases: n_key_aliases as usize, n_keys: n_keys as usize, n_radio_groups: n_radio_groups as usize, n_types: n_types as usize, virtual_mods: virtual_mods as usize});
        }
        if expr & (GbnDetail::GEOMETRY.bits() as usize) != 0 {
            // geometry_type
            sz += 1usize;
            // geometry_device_id
            sz += 1usize;
            // geometry_sequence
            sz += 2usize;
            // geometry_length
            sz += 4usize;
            // name
            sz += 4usize;
            // geometry_found
            sz += 1usize;
            // pad
            sz += 1usize;
            // width_mm
            sz += 2usize;
            // height_mm
            sz += 2usize;
            // n_properties
            sz += 2usize;
            // n_colors
            sz += 2usize;
            // n_shapes
            sz += 2usize;
            // n_sections
            sz += 2usize;
            // n_doodads
            sz += 2usize;
            // n_key_aliases
            sz += 2usize;
            // base_color_ndx
            sz += 1usize;
            // label_color_ndx
            sz += 1usize;
            // label_font
            sz += <&CountedString16>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: GetKbdByNameReplyRepliesParams, out_offset: &mut usize) -> Vec<GetKbdByNameReplyReplies> {
        let GetKbdByNameReplyRepliesParams{
            reported,
        } = params;
        let expr = (reported as usize);
        let mut result = Vec::new();
        if expr & (GbnDetail::TYPES.bits() as usize) & (GbnDetail::CLIENT_SYMBOLS.bits() as usize) & (GbnDetail::SERVER_SYMBOLS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let getmap_type = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let type_device_id = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let getmap_sequence = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let getmap_length = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            offset += 2; // pad
            let type_min_key_code = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let type_max_key_code = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let present = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let first_type = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let n_types = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let total_types = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let first_key_sym = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let total_syms = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_key_syms = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let first_key_action = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let total_actions = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_key_actions = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let first_key_behavior = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let n_key_behaviors = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let total_key_behaviors = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let first_key_explicit = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let n_key_explicit = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let total_key_explicit = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let first_mod_map_key = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let n_mod_map_keys = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let total_mod_map_keys = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let first_v_mod_map_key = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let n_v_mod_map_keys = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let total_v_mod_map_keys = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            offset += 1; // pad
            let virtual_mods = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let map_params = GetKbdByNameReplyRepliesMapParams {present: present as usize, n_key_actions: n_key_actions as usize, n_key_syms: n_key_syms as usize, n_types: n_types as usize, total_actions: total_actions as usize, total_key_behaviors: total_key_behaviors as usize, total_key_explicit: total_key_explicit as usize, total_mod_map_keys: total_mod_map_keys as usize, total_v_mod_map_keys: total_v_mod_map_keys as usize, virtual_mods: virtual_mods as usize};
            let map = <Vec<GetKbdByNameReplyRepliesMap>>::unserialize(wire_data.add(offset), map_params, &mut offset);
            *out_offset += offset;
            result.push(GetKbdByNameReplyReplies::Types{
                getmap_type,
                type_device_id,
                getmap_sequence,
                getmap_length,
                type_min_key_code,
                type_max_key_code,
                first_type,
                n_types,
                total_types,
                first_key_sym,
                total_syms,
                n_key_syms,
                first_key_action,
                total_actions,
                n_key_actions,
                first_key_behavior,
                n_key_behaviors,
                total_key_behaviors,
                first_key_explicit,
                n_key_explicit,
                total_key_explicit,
                first_mod_map_key,
                n_mod_map_keys,
                total_mod_map_keys,
                first_v_mod_map_key,
                n_v_mod_map_keys,
                total_v_mod_map_keys,
                virtual_mods: VMod::from_bits(virtual_mods as u32).unwrap(),
                map,
            });
        }
        if expr & (GbnDetail::COMPAT_MAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let compatmap_type = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let compat_device_id = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let compatmap_sequence = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let compatmap_length = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let groups_rtrn = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            offset += 1; // pad
            let first_si_rtrn = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_si_rtrn = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_total_si = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            offset += 16; // pad
            let si_rtrn = {
                let ptr = wire_data.add(offset) as *const SymInterpret;
                let len = (n_si_rtrn as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<SymInterpret>();
                data.to_vec()
            };
            let group_rtrn = {
                let ptr = wire_data.add(offset) as *const ModDef;
                let len = ((groups_rtrn as usize).count_ones() as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<ModDef>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyReplies::CompatMap{
                compatmap_type,
                compat_device_id,
                compatmap_sequence,
                compatmap_length,
                groups_rtrn: SetOfGroup::from_bits(groups_rtrn as u32).unwrap(),
                first_si_rtrn,
                n_total_si,
                si_rtrn,
                group_rtrn,
            });
        }
        if expr & (GbnDetail::INDICATOR_MAPS.bits() as usize) != 0 {
            let mut offset = 0usize;
            let indicatormap_type = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let indicator_device_id = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let indicatormap_sequence = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let indicatormap_length = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let which = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let real_indicators = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let n_indicators = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            offset += 15; // pad
            let maps = {
                let ptr = wire_data.add(offset) as *const IndicatorMap;
                let len = (n_indicators as usize);
                let data = std::slice::from_raw_parts(ptr, len);
                offset += len * std::mem::size_of::<IndicatorMap>();
                data.to_vec()
            };
            *out_offset += offset;
            result.push(GetKbdByNameReplyReplies::IndicatorMaps{
                indicatormap_type,
                indicator_device_id,
                indicatormap_sequence,
                indicatormap_length,
                which,
                real_indicators,
                maps,
            });
        }
        if expr & (GbnDetail::KEY_NAMES.bits() as usize) & (GbnDetail::OTHER_NAMES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let keyname_type = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let key_device_id = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let keyname_sequence = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let keyname_length = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let which = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let key_min_key_code = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let key_max_key_code = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let n_types = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let group_names = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let virtual_mods = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let first_key = *(wire_data.add(offset) as *const xproto::Keycode);
            offset += std::mem::size_of::<xproto::Keycode>();
            let n_keys = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let indicators = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let n_radio_groups = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let n_key_aliases = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let n_kt_levels = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            offset += 4; // pad
            let value_list_params = GetKbdByNameReplyRepliesValueListParams {which: which as usize, group_names: group_names as usize, indicators: indicators as usize, n_key_aliases: n_key_aliases as usize, n_keys: n_keys as usize, n_radio_groups: n_radio_groups as usize, n_types: n_types as usize, virtual_mods: virtual_mods as usize};
            let value_list = <Vec<GetKbdByNameReplyRepliesValueList>>::unserialize(wire_data.add(offset), value_list_params, &mut offset);
            *out_offset += offset;
            result.push(GetKbdByNameReplyReplies::KeyNames{
                keyname_type,
                key_device_id,
                keyname_sequence,
                keyname_length,
                key_min_key_code,
                key_max_key_code,
                n_types,
                group_names: SetOfGroup::from_bits(group_names as u32).unwrap(),
                virtual_mods: VMod::from_bits(virtual_mods as u32).unwrap(),
                first_key,
                n_keys,
                indicators,
                n_radio_groups,
                n_key_aliases,
                n_kt_levels,
                value_list,
            });
        }
        if expr & (GbnDetail::GEOMETRY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let geometry_type = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let geometry_device_id = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let geometry_sequence = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let geometry_length = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            let name = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            let geometry_found = *(wire_data.add(offset) as *const u8) != 0;
            offset += 1;
            offset += 1; // pad
            let width_mm = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let height_mm = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_properties = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_colors = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_shapes = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_sections = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_doodads = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let n_key_aliases = *(wire_data.add(offset) as *const u16);
            offset += std::mem::size_of::<u16>();
            let base_color_ndx = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let label_color_ndx = *(wire_data.add(offset) as *const u8);
            offset += std::mem::size_of::<u8>();
            let label_font = CountedString16Buf::unserialize(wire_data.add(offset), (), &mut offset);
            *out_offset += offset;
            result.push(GetKbdByNameReplyReplies::Geometry{
                geometry_type,
                geometry_device_id,
                geometry_sequence,
                geometry_length,
                name,
                geometry_found,
                width_mm,
                height_mm,
                n_properties,
                n_colors,
                n_shapes,
                n_sections,
                n_doodads,
                n_key_aliases,
                base_color_ndx,
                label_color_ndx,
                label_font,
            });
        }
        result
    }
}

pub(crate) fn request_name(opcode: u16) -> std::option::Option<&'static str> {
    match opcode {
        0 => Some("xkb::UseExtension"),
        1 => Some("xkb::SelectEvents"),
        3 => Some("xkb::Bell"),
        4 => Some("xkb::GetState"),
        5 => Some("xkb::LatchLockState"),
        6 => Some("xkb::GetControls"),
        7 => Some("xkb::SetControls"),
        8 => Some("xkb::GetMap"),
        9 => Some("xkb::SetMap"),
        10 => Some("xkb::GetCompatMap"),
        11 => Some("xkb::SetCompatMap"),
        12 => Some("xkb::GetIndicatorState"),
        13 => Some("xkb::GetIndicatorMap"),
        14 => Some("xkb::SetIndicatorMap"),
        15 => Some("xkb::GetNamedIndicator"),
        16 => Some("xkb::SetNamedIndicator"),
        17 => Some("xkb::GetNames"),
        18 => Some("xkb::SetNames"),
        21 => Some("xkb::PerClientFlags"),
        22 => Some("xkb::ListComponents"),
        23 => Some("xkb::GetKbdByName"),
        24 => Some("xkb::GetDeviceInfo"),
        25 => Some("xkb::SetDeviceInfo"),
        101 => Some("xkb::SetDebuggingFlags"),
        _ => None,
    }
}

/// Reply type for [UseExtension].
///
/// Can be obtained from a [UseExtensionCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [UseExtensionCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct UseExtensionReply {
    raw: *const u8,
}

impl UseExtensionReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn supported(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(1usize)) };
        val != 0
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn server_major(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn server_minor(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

}

impl base::Reply for UseExtensionReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for UseExtensionReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseExtensionReply")
            .field("response_type", &self.response_type())
            .field("supported", &self.supported())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("server_major", &self.server_major())
            .field("server_minor", &self.server_minor())
            .field("pad", &20)
            .finish()
    }
}

impl Drop for UseExtensionReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for UseExtensionReply {}
unsafe impl std::marker::Sync for UseExtensionReply {}

/// Cookie type for [UseExtension].
///
/// This cookie can be used to get a [UseExtensionReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct UseExtensionCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [UseExtension].
///
/// This cookie can be used to get a [UseExtensionReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct UseExtensionCookieUnchecked {
    seq: u64,
}

impl base::Cookie for UseExtensionCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        UseExtensionCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for UseExtensionCookie {
}

unsafe impl base::CookieWithReplyChecked for UseExtensionCookie {
    type Reply = UseExtensionReply;
}

impl base::Cookie for UseExtensionCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        UseExtensionCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for UseExtensionCookieUnchecked {
    type Reply = UseExtensionReply;
}

/// The `UseExtension` request.
///
/// This request replies [UseExtensionReply].
///
/// Associated cookie types are [UseExtensionCookie] and [UseExtensionCookieUnchecked].
#[derive(Clone, Debug)]
pub struct UseExtension {
    pub wanted_major: u16,
    pub wanted_minor: u16,
}

unsafe impl base::RawRequest for UseExtension {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 0,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.wanted_major.serialize(&mut buf0[4 .. ]);
        self.wanted_minor.serialize(&mut buf0[6 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for UseExtension {
    type Cookie = UseExtensionCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for UseExtension {
    type Reply = UseExtensionReply;
    type Cookie = UseExtensionCookie;
    type CookieUnchecked = UseExtensionCookieUnchecked;
}

/// The `SelectEvents` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SelectEvents<'a> {
    pub device_spec: DeviceSpec,
    pub affect_which: EventType,
    pub clear: EventType,
    pub select_all: EventType,
    pub affect_map: MapPart,
    pub map: MapPart,
    pub details: &'a [SelectEventsDetails],
}

unsafe impl<'a> base::RawRequest for SelectEvents<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(SelectEventsDetails::is_sorted_distinct(self.details), "SelectEvents::details must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 1,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (self.affect_which.bits() as u16).serialize(&mut buf0[6 .. ]);
        (self.clear.bits() as u16).serialize(&mut buf0[8 .. ]);
        (self.select_all.bits() as u16).serialize(&mut buf0[10 .. ]);
        (self.affect_map.bits() as u16).serialize(&mut buf0[12 .. ]);
        (self.map.bits() as u16).serialize(&mut buf0[14 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.details.wire_len();
        let mut buf1 = vec![0u8; len1];
        self.details.serialize(&mut buf1);
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SelectEvents<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SelectEvents<'a> {
}

/// The `Bell` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct Bell {
    pub device_spec: DeviceSpec,
    pub bell_class: BellClassSpec,
    pub bell_id: IdSpec,
    pub percent: i8,
    pub force_sound: bool,
    pub event_only: bool,
    pub pitch: i16,
    pub duration: i16,
    pub name: xproto::Atom,
    pub window: xproto::Window,
}

unsafe impl base::RawRequest for Bell {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 3,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 28];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        self.bell_class.serialize(&mut buf0[6 .. ]);
        self.bell_id.serialize(&mut buf0[8 .. ]);
        self.percent.serialize(&mut buf0[10 .. ]);
        (if self.force_sound { 1u8 } else { 0u8 }).serialize(&mut buf0[11 .. ]);
        (if self.event_only { 1u8 } else { 0u8 }).serialize(&mut buf0[12 .. ]);
        self.pitch.serialize(&mut buf0[14 .. ]);
        self.duration.serialize(&mut buf0[16 .. ]);
        self.name.serialize(&mut buf0[20 .. ]);
        self.window.serialize(&mut buf0[24 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for Bell {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for Bell {
}

/// Reply type for [GetState].
///
/// Can be obtained from a [GetStateCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetStateCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetStateReply {
    raw: *const u8,
}

impl GetStateReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn base_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn latched_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn locked_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 11usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn group(&self) -> Group {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, Group>(val)
        }
    }

    pub fn locked_group(&self) -> Group {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, Group>(val)
        }
    }

    pub fn base_group(&self) -> i16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            ptr.read_unaligned()
        }
    }

    pub fn latched_group(&self) -> i16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            ptr.read_unaligned()
        }
    }

    pub fn compat_state(&self) -> xproto::ModMask {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn grab_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 19usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn compat_grab_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn lookup_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 21usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn compat_lookup_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }


    pub fn ptr_btn_state(&self) -> xproto::KeyButMask {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::KeyButMask>(val)
        }
    }

}

impl base::Reply for GetStateReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetStateReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetStateReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("mods", &self.mods())
            .field("base_mods", &self.base_mods())
            .field("latched_mods", &self.latched_mods())
            .field("locked_mods", &self.locked_mods())
            .field("group", &self.group())
            .field("locked_group", &self.locked_group())
            .field("base_group", &self.base_group())
            .field("latched_group", &self.latched_group())
            .field("compat_state", &self.compat_state())
            .field("grab_mods", &self.grab_mods())
            .field("compat_grab_mods", &self.compat_grab_mods())
            .field("lookup_mods", &self.lookup_mods())
            .field("compat_lookup_mods", &self.compat_lookup_mods())
            .field("pad", &1)
            .field("ptr_btn_state", &self.ptr_btn_state())
            .field("pad", &6)
            .finish()
    }
}

impl Drop for GetStateReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetStateReply {}
unsafe impl std::marker::Sync for GetStateReply {}

/// Cookie type for [GetState].
///
/// This cookie can be used to get a [GetStateReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetStateCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetState].
///
/// This cookie can be used to get a [GetStateReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetStateCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetStateCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetStateCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetStateCookie {
}

unsafe impl base::CookieWithReplyChecked for GetStateCookie {
    type Reply = GetStateReply;
}

impl base::Cookie for GetStateCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetStateCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetStateCookieUnchecked {
    type Reply = GetStateReply;
}

/// The `GetState` request.
///
/// This request replies [GetStateReply].
///
/// Associated cookie types are [GetStateCookie] and [GetStateCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetState {
    pub device_spec: DeviceSpec,
}

unsafe impl base::RawRequest for GetState {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 4,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetState {
    type Cookie = GetStateCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetState {
    type Reply = GetStateReply;
    type Cookie = GetStateCookie;
    type CookieUnchecked = GetStateCookieUnchecked;
}

/// The `LatchLockState` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct LatchLockState {
    pub device_spec: DeviceSpec,
    pub affect_mod_locks: xproto::ModMask,
    pub mod_locks: xproto::ModMask,
    pub lock_group: bool,
    pub group_lock: Group,
    pub affect_mod_latches: xproto::ModMask,
    pub latch_group: bool,
    pub group_latch: u16,
}

unsafe impl base::RawRequest for LatchLockState {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 5,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (self.affect_mod_locks.bits() as u8).serialize(&mut buf0[6 .. ]);
        (self.mod_locks.bits() as u8).serialize(&mut buf0[7 .. ]);
        (if self.lock_group { 1u8 } else { 0u8 }).serialize(&mut buf0[8 .. ]);
        (std::mem::transmute::<_, u32>(self.group_lock) as u8).serialize(&mut buf0[9 .. ]);
        (self.affect_mod_latches.bits() as u8).serialize(&mut buf0[10 .. ]);
        (if self.latch_group { 1u8 } else { 0u8 }).serialize(&mut buf0[13 .. ]);
        self.group_latch.serialize(&mut buf0[14 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for LatchLockState {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for LatchLockState {
}

/// Reply type for [GetControls].
///
/// Can be obtained from a [GetControlsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetControlsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetControlsReply {
    raw: *const u8,
}

impl GetControlsReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    unsafe fn compute_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // mouse_keys_dflt_btn
        sz += 1usize;
        // num_groups
        sz += 1usize;
        // groups_wrap
        sz += 1usize;
        // internal_mods_mask
        sz += 1usize;
        // ignore_lock_mods_mask
        sz += 1usize;
        // internal_mods_real_mods
        sz += 1usize;
        // ignore_lock_mods_real_mods
        sz += 1usize;
        // pad
        sz += 1usize;
        // internal_mods_vmods
        sz += 2usize;
        // ignore_lock_mods_vmods
        sz += 2usize;
        // repeat_delay
        sz += 2usize;
        // repeat_interval
        sz += 2usize;
        // slow_keys_delay
        sz += 2usize;
        // debounce_delay
        sz += 2usize;
        // mouse_keys_delay
        sz += 2usize;
        // mouse_keys_interval
        sz += 2usize;
        // mouse_keys_time_to_max
        sz += 2usize;
        // mouse_keys_max_speed
        sz += 2usize;
        // mouse_keys_curve
        sz += 2usize;
        // access_x_option
        sz += 2usize;
        // access_x_timeout
        sz += 2usize;
        // access_x_timeout_options_mask
        sz += 2usize;
        // access_x_timeout_options_values
        sz += 2usize;
        // pad
        sz += 2usize;
        // access_x_timeout_mask
        sz += 4usize;
        // access_x_timeout_values
        sz += 4usize;
        // enabled_controls
        sz += 4usize;
        // per_key_repeat
        sz += 32usize;
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn mouse_keys_dflt_btn(&self) -> u8 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn num_groups(&self) -> u8 {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn groups_wrap(&self) -> u8 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn internal_mods_mask(&self) -> xproto::ModMask {
        unsafe {
            let offset = 11usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn ignore_lock_mods_mask(&self) -> xproto::ModMask {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn internal_mods_real_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn ignore_lock_mods_real_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }


    pub fn internal_mods_vmods(&self) -> VMod {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

    pub fn ignore_lock_mods_vmods(&self) -> VMod {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

    pub fn repeat_delay(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn repeat_interval(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn slow_keys_delay(&self) -> u16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn debounce_delay(&self) -> u16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn mouse_keys_delay(&self) -> u16 {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn mouse_keys_interval(&self) -> u16 {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn mouse_keys_time_to_max(&self) -> u16 {
        unsafe {
            let offset = 32usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn mouse_keys_max_speed(&self) -> u16 {
        unsafe {
            let offset = 34usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn mouse_keys_curve(&self) -> i16 {
        unsafe {
            let offset = 36usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            ptr.read_unaligned()
        }
    }

    pub fn access_x_option(&self) -> AxOption {
        unsafe {
            let offset = 38usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, AxOption>(val)
        }
    }

    pub fn access_x_timeout(&self) -> u16 {
        unsafe {
            let offset = 40usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn access_x_timeout_options_mask(&self) -> AxOption {
        unsafe {
            let offset = 42usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, AxOption>(val)
        }
    }

    pub fn access_x_timeout_options_values(&self) -> AxOption {
        unsafe {
            let offset = 44usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, AxOption>(val)
        }
    }


    pub fn access_x_timeout_mask(&self) -> BoolCtrl {
        unsafe {
            let offset = 48usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BoolCtrl>(val)
        }
    }

    pub fn access_x_timeout_values(&self) -> BoolCtrl {
        unsafe {
            let offset = 52usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BoolCtrl>(val)
        }
    }

    pub fn enabled_controls(&self) -> BoolCtrl {
        unsafe {
            let offset = 56usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BoolCtrl>(val)
        }
    }

    pub fn per_key_repeat(&self) -> &[u8; 32] {
        unsafe {
            let offset = 60usize;
            let ptr = self.wire_ptr().add(offset) as *const [u8; 32];
            &*ptr
        }
    }
}

impl base::Reply for GetControlsReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetControlsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetControlsReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("mouse_keys_dflt_btn", &self.mouse_keys_dflt_btn())
            .field("num_groups", &self.num_groups())
            .field("groups_wrap", &self.groups_wrap())
            .field("internal_mods_mask", &self.internal_mods_mask())
            .field("ignore_lock_mods_mask", &self.ignore_lock_mods_mask())
            .field("internal_mods_real_mods", &self.internal_mods_real_mods())
            .field("ignore_lock_mods_real_mods", &self.ignore_lock_mods_real_mods())
            .field("pad", &1)
            .field("internal_mods_vmods", &self.internal_mods_vmods())
            .field("ignore_lock_mods_vmods", &self.ignore_lock_mods_vmods())
            .field("repeat_delay", &self.repeat_delay())
            .field("repeat_interval", &self.repeat_interval())
            .field("slow_keys_delay", &self.slow_keys_delay())
            .field("debounce_delay", &self.debounce_delay())
            .field("mouse_keys_delay", &self.mouse_keys_delay())
            .field("mouse_keys_interval", &self.mouse_keys_interval())
            .field("mouse_keys_time_to_max", &self.mouse_keys_time_to_max())
            .field("mouse_keys_max_speed", &self.mouse_keys_max_speed())
            .field("mouse_keys_curve", &self.mouse_keys_curve())
            .field("access_x_option", &self.access_x_option())
            .field("access_x_timeout", &self.access_x_timeout())
            .field("access_x_timeout_options_mask", &self.access_x_timeout_options_mask())
            .field("access_x_timeout_options_values", &self.access_x_timeout_options_values())
            .field("pad", &2)
            .field("access_x_timeout_mask", &self.access_x_timeout_mask())
            .field("access_x_timeout_values", &self.access_x_timeout_values())
            .field("enabled_controls", &self.enabled_controls())
            .field("per_key_repeat", &self.per_key_repeat())
            .finish()
    }
}

impl Drop for GetControlsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetControlsReply {}
unsafe impl std::marker::Sync for GetControlsReply {}

/// Cookie type for [GetControls].
///
/// This cookie can be used to get a [GetControlsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetControlsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetControls].
///
/// This cookie can be used to get a [GetControlsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetControlsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetControlsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetControlsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetControlsCookie {
}

unsafe impl base::CookieWithReplyChecked for GetControlsCookie {
    type Reply = GetControlsReply;
}

impl base::Cookie for GetControlsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetControlsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetControlsCookieUnchecked {
    type Reply = GetControlsReply;
}

/// The `GetControls` request.
///
/// This request replies [GetControlsReply].
///
/// Associated cookie types are [GetControlsCookie] and [GetControlsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetControls {
    pub device_spec: DeviceSpec,
}

unsafe impl base::RawRequest for GetControls {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 6,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetControls {
    type Cookie = GetControlsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetControls {
    type Reply = GetControlsReply;
    type Cookie = GetControlsCookie;
    type CookieUnchecked = GetControlsCookieUnchecked;
}

/// The `SetControls` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetControls {
    pub device_spec: DeviceSpec,
    pub affect_internal_real_mods: xproto::ModMask,
    pub internal_real_mods: xproto::ModMask,
    pub affect_ignore_lock_real_mods: xproto::ModMask,
    pub ignore_lock_real_mods: xproto::ModMask,
    pub affect_internal_virtual_mods: VMod,
    pub internal_virtual_mods: VMod,
    pub affect_ignore_lock_virtual_mods: VMod,
    pub ignore_lock_virtual_mods: VMod,
    pub mouse_keys_dflt_btn: u8,
    pub groups_wrap: u8,
    pub access_x_options: AxOption,
    pub affect_enabled_controls: BoolCtrl,
    pub enabled_controls: BoolCtrl,
    pub change_controls: Control,
    pub repeat_delay: u16,
    pub repeat_interval: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub mouse_keys_delay: u16,
    pub mouse_keys_interval: u16,
    pub mouse_keys_time_to_max: u16,
    pub mouse_keys_max_speed: u16,
    pub mouse_keys_curve: i16,
    pub access_x_timeout: u16,
    pub access_x_timeout_mask: BoolCtrl,
    pub access_x_timeout_values: BoolCtrl,
    pub access_x_timeout_options_mask: AxOption,
    pub access_x_timeout_options_values: AxOption,
    pub per_key_repeat: [u8; 32],
}

unsafe impl base::RawRequest for SetControls {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 7,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 100];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (self.affect_internal_real_mods.bits() as u8).serialize(&mut buf0[6 .. ]);
        (self.internal_real_mods.bits() as u8).serialize(&mut buf0[7 .. ]);
        (self.affect_ignore_lock_real_mods.bits() as u8).serialize(&mut buf0[8 .. ]);
        (self.ignore_lock_real_mods.bits() as u8).serialize(&mut buf0[9 .. ]);
        (self.affect_internal_virtual_mods.bits() as u16).serialize(&mut buf0[10 .. ]);
        (self.internal_virtual_mods.bits() as u16).serialize(&mut buf0[12 .. ]);
        (self.affect_ignore_lock_virtual_mods.bits() as u16).serialize(&mut buf0[14 .. ]);
        (self.ignore_lock_virtual_mods.bits() as u16).serialize(&mut buf0[16 .. ]);
        self.mouse_keys_dflt_btn.serialize(&mut buf0[18 .. ]);
        self.groups_wrap.serialize(&mut buf0[19 .. ]);
        (self.access_x_options.bits() as u16).serialize(&mut buf0[20 .. ]);
        self.affect_enabled_controls.bits().serialize(&mut buf0[24 .. ]);
        self.enabled_controls.bits().serialize(&mut buf0[28 .. ]);
        self.change_controls.bits().serialize(&mut buf0[32 .. ]);
        self.repeat_delay.serialize(&mut buf0[36 .. ]);
        self.repeat_interval.serialize(&mut buf0[38 .. ]);
        self.slow_keys_delay.serialize(&mut buf0[40 .. ]);
        self.debounce_delay.serialize(&mut buf0[42 .. ]);
        self.mouse_keys_delay.serialize(&mut buf0[44 .. ]);
        self.mouse_keys_interval.serialize(&mut buf0[46 .. ]);
        self.mouse_keys_time_to_max.serialize(&mut buf0[48 .. ]);
        self.mouse_keys_max_speed.serialize(&mut buf0[50 .. ]);
        self.mouse_keys_curve.serialize(&mut buf0[52 .. ]);
        self.access_x_timeout.serialize(&mut buf0[54 .. ]);
        self.access_x_timeout_mask.bits().serialize(&mut buf0[56 .. ]);
        self.access_x_timeout_values.bits().serialize(&mut buf0[60 .. ]);
        (self.access_x_timeout_options_mask.bits() as u16).serialize(&mut buf0[64 .. ]);
        (self.access_x_timeout_options_values.bits() as u16).serialize(&mut buf0[66 .. ]);
        std::slice::from_raw_parts_mut(
            buf0.as_mut_ptr().add(68) as *mut u8, 32
        ).copy_from_slice(&self.per_key_repeat);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 100;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for SetControls {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetControls {
}

/// Reply type for [GetMap].
///
/// Can be obtained from a [GetMapCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetMapCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetMapReply {
    raw: *const u8,
}

impl GetMapReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }


    pub fn min_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn max_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 11usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn present(&self) -> MapPart {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, MapPart>(val)
        }
    }

    pub fn first_type(&self) -> u8 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn n_types(&self) -> u8 {
        unsafe {
            let offset = 15usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn total_types(&self) -> u8 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_key_sym(&self) -> xproto::Keycode {
        unsafe {
            let offset = 17usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn total_syms(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    fn n_key_syms(&self) -> u8 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_key_action(&self) -> xproto::Keycode {
        unsafe {
            let offset = 21usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    fn total_actions(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    fn n_key_actions(&self) -> u8 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_key_behavior(&self) -> xproto::Keycode {
        unsafe {
            let offset = 25usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_key_behaviors(&self) -> u8 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn total_key_behaviors(&self) -> u8 {
        unsafe {
            let offset = 27usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_key_explicit(&self) -> xproto::Keycode {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_key_explicit(&self) -> u8 {
        unsafe {
            let offset = 29usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn total_key_explicit(&self) -> u8 {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_mod_map_key(&self) -> xproto::Keycode {
        unsafe {
            let offset = 31usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_mod_map_keys(&self) -> u8 {
        unsafe {
            let offset = 32usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn total_mod_map_keys(&self) -> u8 {
        unsafe {
            let offset = 33usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_v_mod_map_key(&self) -> xproto::Keycode {
        unsafe {
            let offset = 34usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn n_v_mod_map_keys(&self) -> u8 {
        unsafe {
            let offset = 35usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn total_v_mod_map_keys(&self) -> u8 {
        unsafe {
            let offset = 36usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn virtual_mods(&self) -> VMod {
        unsafe {
            let offset = 38usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

    pub fn map(&self) -> Vec<GetMapReplyMap> {
        let present = self.present().bits();
        let n_key_actions = self.n_key_actions();
        let n_key_syms = self.n_key_syms();
        let n_types = self.n_types();
        let total_actions = self.total_actions();
        let total_key_behaviors = self.total_key_behaviors();
        let total_key_explicit = self.total_key_explicit();
        let total_mod_map_keys = self.total_mod_map_keys();
        let total_v_mod_map_keys = self.total_v_mod_map_keys();
        let virtual_mods = self.virtual_mods().bits();
        let params = GetMapReplyMapParams {present: present as usize, n_key_actions: n_key_actions as usize, n_key_syms: n_key_syms as usize, n_types: n_types as usize, total_actions: total_actions as usize, total_key_behaviors: total_key_behaviors as usize, total_key_explicit: total_key_explicit as usize, total_mod_map_keys: total_mod_map_keys as usize, total_v_mod_map_keys: total_v_mod_map_keys as usize, virtual_mods: virtual_mods as usize};
        let mut offset = 40usize;
        unsafe {
            <Vec<GetMapReplyMap>>::unserialize(
                self.wire_ptr().add(offset), params, &mut offset
            )
        }
    }
}

impl base::Reply for GetMapReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetMapReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetMapReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pad", &2)
            .field("min_key_code", &self.min_key_code())
            .field("max_key_code", &self.max_key_code())
            .field("present", &self.present())
            .field("first_type", &self.first_type())
            .field("n_types", &self.n_types())
            .field("total_types", &self.total_types())
            .field("first_key_sym", &self.first_key_sym())
            .field("total_syms", &self.total_syms())
            .field("n_key_syms", &self.n_key_syms())
            .field("first_key_action", &self.first_key_action())
            .field("total_actions", &self.total_actions())
            .field("n_key_actions", &self.n_key_actions())
            .field("first_key_behavior", &self.first_key_behavior())
            .field("n_key_behaviors", &self.n_key_behaviors())
            .field("total_key_behaviors", &self.total_key_behaviors())
            .field("first_key_explicit", &self.first_key_explicit())
            .field("n_key_explicit", &self.n_key_explicit())
            .field("total_key_explicit", &self.total_key_explicit())
            .field("first_mod_map_key", &self.first_mod_map_key())
            .field("n_mod_map_keys", &self.n_mod_map_keys())
            .field("total_mod_map_keys", &self.total_mod_map_keys())
            .field("first_v_mod_map_key", &self.first_v_mod_map_key())
            .field("n_v_mod_map_keys", &self.n_v_mod_map_keys())
            .field("total_v_mod_map_keys", &self.total_v_mod_map_keys())
            .field("pad", &1)
            .field("virtual_mods", &self.virtual_mods())
            .field("map", &self.map())
            .finish()
    }
}

impl Drop for GetMapReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetMapReply {}
unsafe impl std::marker::Sync for GetMapReply {}

/// Cookie type for [GetMap].
///
/// This cookie can be used to get a [GetMapReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetMapCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetMap].
///
/// This cookie can be used to get a [GetMapReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetMapCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetMapCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetMapCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetMapCookie {
}

unsafe impl base::CookieWithReplyChecked for GetMapCookie {
    type Reply = GetMapReply;
}

impl base::Cookie for GetMapCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetMapCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetMapCookieUnchecked {
    type Reply = GetMapReply;
}

/// The `GetMap` request.
///
/// This request replies [GetMapReply].
///
/// Associated cookie types are [GetMapCookie] and [GetMapCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetMap {
    pub device_spec: DeviceSpec,
    pub full: MapPart,
    pub partial: MapPart,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xproto::Keycode,
    pub n_key_syms: u8,
    pub first_key_action: xproto::Keycode,
    pub n_key_actions: u8,
    pub first_key_behavior: xproto::Keycode,
    pub n_key_behaviors: u8,
    pub virtual_mods: VMod,
    pub first_key_explicit: xproto::Keycode,
    pub n_key_explicit: u8,
    pub first_mod_map_key: xproto::Keycode,
    pub n_mod_map_keys: u8,
    pub first_v_mod_map_key: xproto::Keycode,
    pub n_v_mod_map_keys: u8,
}

unsafe impl base::RawRequest for GetMap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 8,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 28];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (self.full.bits() as u16).serialize(&mut buf0[6 .. ]);
        (self.partial.bits() as u16).serialize(&mut buf0[8 .. ]);
        self.first_type.serialize(&mut buf0[10 .. ]);
        self.n_types.serialize(&mut buf0[11 .. ]);
        self.first_key_sym.serialize(&mut buf0[12 .. ]);
        self.n_key_syms.serialize(&mut buf0[13 .. ]);
        self.first_key_action.serialize(&mut buf0[14 .. ]);
        self.n_key_actions.serialize(&mut buf0[15 .. ]);
        self.first_key_behavior.serialize(&mut buf0[16 .. ]);
        self.n_key_behaviors.serialize(&mut buf0[17 .. ]);
        (self.virtual_mods.bits() as u16).serialize(&mut buf0[18 .. ]);
        self.first_key_explicit.serialize(&mut buf0[20 .. ]);
        self.n_key_explicit.serialize(&mut buf0[21 .. ]);
        self.first_mod_map_key.serialize(&mut buf0[22 .. ]);
        self.n_mod_map_keys.serialize(&mut buf0[23 .. ]);
        self.first_v_mod_map_key.serialize(&mut buf0[24 .. ]);
        self.n_v_mod_map_keys.serialize(&mut buf0[25 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetMap {
    type Cookie = GetMapCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetMap {
    type Reply = GetMapReply;
    type Cookie = GetMapCookie;
    type CookieUnchecked = GetMapCookieUnchecked;
}

/// The `SetMap` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetMap<'a> {
    pub device_spec: DeviceSpec,
    pub flags: SetMapFlags,
    pub min_key_code: xproto::Keycode,
    pub max_key_code: xproto::Keycode,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xproto::Keycode,
    pub n_key_syms: u8,
    pub total_syms: u16,
    pub first_key_action: xproto::Keycode,
    pub n_key_actions: u8,
    pub total_actions: u16,
    pub first_key_behavior: xproto::Keycode,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: xproto::Keycode,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: xproto::Keycode,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_v_mod_map_key: xproto::Keycode,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub virtual_mods: VMod,
    pub values: &'a [SetMapValues],
}

unsafe impl<'a> base::RawRequest for SetMap<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(SetMapValues::is_sorted_distinct(self.values), "SetMap::values must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 9,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 36];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (SetMapValues::get_mask(self.values).bits() as u16).serialize(&mut buf0[6 .. ]);
        (self.flags.bits() as u16).serialize(&mut buf0[8 .. ]);
        self.min_key_code.serialize(&mut buf0[10 .. ]);
        self.max_key_code.serialize(&mut buf0[11 .. ]);
        self.first_type.serialize(&mut buf0[12 .. ]);
        self.n_types.serialize(&mut buf0[13 .. ]);
        self.first_key_sym.serialize(&mut buf0[14 .. ]);
        self.n_key_syms.serialize(&mut buf0[15 .. ]);
        self.total_syms.serialize(&mut buf0[16 .. ]);
        self.first_key_action.serialize(&mut buf0[18 .. ]);
        self.n_key_actions.serialize(&mut buf0[19 .. ]);
        self.total_actions.serialize(&mut buf0[20 .. ]);
        self.first_key_behavior.serialize(&mut buf0[22 .. ]);
        self.n_key_behaviors.serialize(&mut buf0[23 .. ]);
        self.total_key_behaviors.serialize(&mut buf0[24 .. ]);
        self.first_key_explicit.serialize(&mut buf0[25 .. ]);
        self.n_key_explicit.serialize(&mut buf0[26 .. ]);
        self.total_key_explicit.serialize(&mut buf0[27 .. ]);
        self.first_mod_map_key.serialize(&mut buf0[28 .. ]);
        self.n_mod_map_keys.serialize(&mut buf0[29 .. ]);
        self.total_mod_map_keys.serialize(&mut buf0[30 .. ]);
        self.first_v_mod_map_key.serialize(&mut buf0[31 .. ]);
        self.n_v_mod_map_keys.serialize(&mut buf0[32 .. ]);
        self.total_v_mod_map_keys.serialize(&mut buf0[33 .. ]);
        (self.virtual_mods.bits() as u16).serialize(&mut buf0[34 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 36;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.values.wire_len();
        let mut buf1 = vec![0u8; len1];
        self.values.serialize(&mut buf1);
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetMap<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetMap<'a> {
}

/// Reply type for [GetCompatMap].
///
/// Can be obtained from a [GetCompatMapCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetCompatMapCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCompatMapReply {
    raw: *const u8,
}

impl GetCompatMapReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    unsafe fn compute_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // groups_rtrn
        let groups_rtrn = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // pad
        sz += 1usize;
        // first_si_rtrn
        sz += 2usize;
        // n_si_rtrn
        let n_si_rtrn = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_total_si
        sz += 2usize;
        // pad
        sz += 16usize;
        // si_rtrn
        sz += ((n_si_rtrn as usize) * 16usize);
        // group_rtrn
        sz += (((groups_rtrn as usize).count_ones() as usize) * 4usize);
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn groups_rtrn(&self) -> SetOfGroup {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, SetOfGroup>(val)
        }
    }


    pub fn first_si_rtrn(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    fn n_si_rtrn(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn n_total_si(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }


    pub fn si_rtrn(&self) -> &[SymInterpret] {
        unsafe {
            let offset = 32usize;
            let len = (self.n_si_rtrn() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const SymInterpret;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn group_rtrn(&self) -> &[ModDef] {
        unsafe {
            let offset = (32usize + ((self.n_si_rtrn() as usize) * 16usize));
            let len = ((self.groups_rtrn().bits() as usize).count_ones() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const ModDef;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetCompatMapReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetCompatMapReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetCompatMapReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("groups_rtrn", &self.groups_rtrn())
            .field("pad", &1)
            .field("first_si_rtrn", &self.first_si_rtrn())
            .field("n_si_rtrn", &self.n_si_rtrn())
            .field("n_total_si", &self.n_total_si())
            .field("pad", &16)
            .field("si_rtrn", &self.si_rtrn())
            .field("group_rtrn", &self.group_rtrn())
            .finish()
    }
}

impl Drop for GetCompatMapReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetCompatMapReply {}
unsafe impl std::marker::Sync for GetCompatMapReply {}

/// Cookie type for [GetCompatMap].
///
/// This cookie can be used to get a [GetCompatMapReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetCompatMapCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetCompatMap].
///
/// This cookie can be used to get a [GetCompatMapReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCompatMapCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetCompatMapCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCompatMapCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetCompatMapCookie {
}

unsafe impl base::CookieWithReplyChecked for GetCompatMapCookie {
    type Reply = GetCompatMapReply;
}

impl base::Cookie for GetCompatMapCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCompatMapCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetCompatMapCookieUnchecked {
    type Reply = GetCompatMapReply;
}

/// The `GetCompatMap` request.
///
/// This request replies [GetCompatMapReply].
///
/// Associated cookie types are [GetCompatMapCookie] and [GetCompatMapCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetCompatMap {
    pub device_spec: DeviceSpec,
    pub groups: SetOfGroup,
    pub get_all_si: bool,
    pub first_si: u16,
    pub n_si: u16,
}

unsafe impl base::RawRequest for GetCompatMap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 10,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (self.groups.bits() as u8).serialize(&mut buf0[6 .. ]);
        (if self.get_all_si { 1u8 } else { 0u8 }).serialize(&mut buf0[7 .. ]);
        self.first_si.serialize(&mut buf0[8 .. ]);
        self.n_si.serialize(&mut buf0[10 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetCompatMap {
    type Cookie = GetCompatMapCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetCompatMap {
    type Reply = GetCompatMapReply;
    type Cookie = GetCompatMapCookie;
    type CookieUnchecked = GetCompatMapCookieUnchecked;
}

/// The `SetCompatMap` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetCompatMap<'a> {
    pub device_spec: DeviceSpec,
    pub recompute_actions: bool,
    pub truncate_si: bool,
    pub groups: SetOfGroup,
    pub first_si: u16,
    pub si: &'a [SymInterpret],
    pub group_maps: &'a [ModDef],
}

unsafe impl<'a> base::RawRequest for SetCompatMap<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 6,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 11,
            isvoid: 1,
        };

        let mut sections: [iovec; 8] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 8];

        let buf0: &mut [u8] = &mut [0; 16];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (if self.recompute_actions { 1u8 } else { 0u8 }).serialize(&mut buf0[7 .. ]);
        (if self.truncate_si { 1u8 } else { 0u8 }).serialize(&mut buf0[8 .. ]);
        (self.groups.bits() as u8).serialize(&mut buf0[9 .. ]);
        self.first_si.serialize(&mut buf0[10 .. ]);
        (self.si.len() as u16).serialize(&mut buf0[12 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.si.as_ptr() as *mut _;
        sections[4].iov_len = self.si.len() * std::mem::size_of::<SymInterpret>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        sections[6].iov_base = self.group_maps.as_ptr() as *mut _;
        sections[6].iov_len = self.group_maps.len() * std::mem::size_of::<ModDef>();
        sections[7].iov_len = base::align_pad(sections[6].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetCompatMap<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetCompatMap<'a> {
}

/// Reply type for [GetIndicatorState].
///
/// Can be obtained from a [GetIndicatorStateCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetIndicatorStateCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetIndicatorStateReply {
    raw: *const u8,
}

impl GetIndicatorStateReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn state(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

}

impl base::Reply for GetIndicatorStateReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetIndicatorStateReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetIndicatorStateReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("state", &self.state())
            .field("pad", &20)
            .finish()
    }
}

impl Drop for GetIndicatorStateReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetIndicatorStateReply {}
unsafe impl std::marker::Sync for GetIndicatorStateReply {}

/// Cookie type for [GetIndicatorState].
///
/// This cookie can be used to get a [GetIndicatorStateReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetIndicatorStateCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetIndicatorState].
///
/// This cookie can be used to get a [GetIndicatorStateReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetIndicatorStateCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetIndicatorStateCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetIndicatorStateCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetIndicatorStateCookie {
}

unsafe impl base::CookieWithReplyChecked for GetIndicatorStateCookie {
    type Reply = GetIndicatorStateReply;
}

impl base::Cookie for GetIndicatorStateCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetIndicatorStateCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetIndicatorStateCookieUnchecked {
    type Reply = GetIndicatorStateReply;
}

/// The `GetIndicatorState` request.
///
/// This request replies [GetIndicatorStateReply].
///
/// Associated cookie types are [GetIndicatorStateCookie] and [GetIndicatorStateCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetIndicatorState {
    pub device_spec: DeviceSpec,
}

unsafe impl base::RawRequest for GetIndicatorState {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 12,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetIndicatorState {
    type Cookie = GetIndicatorStateCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetIndicatorState {
    type Reply = GetIndicatorStateReply;
    type Cookie = GetIndicatorStateCookie;
    type CookieUnchecked = GetIndicatorStateCookieUnchecked;
}

/// Reply type for [GetIndicatorMap].
///
/// Can be obtained from a [GetIndicatorMapCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetIndicatorMapCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetIndicatorMapReply {
    raw: *const u8,
}

impl GetIndicatorMapReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    unsafe fn compute_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // which
        let which = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // real_indicators
        sz += 4usize;
        // n_indicators
        sz += 1usize;
        // pad
        sz += 15usize;
        // maps
        sz += (((which as usize).count_ones() as usize) * 12usize);
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    fn which(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn real_indicators(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn n_indicators(&self) -> u8 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn maps(&self) -> &[IndicatorMap] {
        unsafe {
            let offset = 32usize;
            let len = ((self.which() as usize).count_ones() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const IndicatorMap;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetIndicatorMapReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetIndicatorMapReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetIndicatorMapReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("which", &self.which())
            .field("real_indicators", &self.real_indicators())
            .field("n_indicators", &self.n_indicators())
            .field("pad", &15)
            .field("maps", &self.maps())
            .finish()
    }
}

impl Drop for GetIndicatorMapReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetIndicatorMapReply {}
unsafe impl std::marker::Sync for GetIndicatorMapReply {}

/// Cookie type for [GetIndicatorMap].
///
/// This cookie can be used to get a [GetIndicatorMapReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetIndicatorMapCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetIndicatorMap].
///
/// This cookie can be used to get a [GetIndicatorMapReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetIndicatorMapCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetIndicatorMapCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetIndicatorMapCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetIndicatorMapCookie {
}

unsafe impl base::CookieWithReplyChecked for GetIndicatorMapCookie {
    type Reply = GetIndicatorMapReply;
}

impl base::Cookie for GetIndicatorMapCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetIndicatorMapCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetIndicatorMapCookieUnchecked {
    type Reply = GetIndicatorMapReply;
}

/// The `GetIndicatorMap` request.
///
/// This request replies [GetIndicatorMapReply].
///
/// Associated cookie types are [GetIndicatorMapCookie] and [GetIndicatorMapCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetIndicatorMap {
    pub device_spec: DeviceSpec,
    pub which: u32,
}

unsafe impl base::RawRequest for GetIndicatorMap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 13,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        self.which.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetIndicatorMap {
    type Cookie = GetIndicatorMapCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetIndicatorMap {
    type Reply = GetIndicatorMapReply;
    type Cookie = GetIndicatorMapCookie;
    type CookieUnchecked = GetIndicatorMapCookieUnchecked;
}

/// The `SetIndicatorMap` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetIndicatorMap<'a> {
    pub device_spec: DeviceSpec,
    pub which: u32,
    pub maps: &'a [IndicatorMap],
}

unsafe impl<'a> base::RawRequest for SetIndicatorMap<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 14,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        self.which.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.maps.as_ptr() as *mut _;
        sections[4].iov_len = self.maps.len() * std::mem::size_of::<IndicatorMap>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetIndicatorMap<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetIndicatorMap<'a> {
}

/// Reply type for [GetNamedIndicator].
///
/// Can be obtained from a [GetNamedIndicatorCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetNamedIndicatorCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetNamedIndicatorReply {
    raw: *const u8,
}

impl GetNamedIndicatorReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn indicator(&self) -> xproto::Atom {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            ptr.read_unaligned()
        }
    }

    pub fn found(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(12usize)) };
        val != 0
    }

    pub fn on(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(13usize)) };
        val != 0
    }

    pub fn real_indicator(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(14usize)) };
        val != 0
    }

    pub fn ndx(&self) -> u8 {
        unsafe {
            let offset = 15usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn map_flags(&self) -> ImFlag {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, ImFlag>(val)
        }
    }

    pub fn map_which_groups(&self) -> ImGroupsWhich {
        unsafe {
            let offset = 17usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, ImGroupsWhich>(val)
        }
    }

    pub fn map_groups(&self) -> SetOfGroups {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, SetOfGroups>(val)
        }
    }

    pub fn map_which_mods(&self) -> ImModsWhich {
        unsafe {
            let offset = 19usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, ImModsWhich>(val)
        }
    }

    pub fn map_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn map_real_mods(&self) -> xproto::ModMask {
        unsafe {
            let offset = 21usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, xproto::ModMask>(val)
        }
    }

    pub fn map_vmod(&self) -> VMod {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

    pub fn map_ctrls(&self) -> BoolCtrl {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BoolCtrl>(val)
        }
    }

    pub fn supported(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(28usize)) };
        val != 0
    }

}

impl base::Reply for GetNamedIndicatorReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetNamedIndicatorReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetNamedIndicatorReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("indicator", &self.indicator())
            .field("found", &self.found())
            .field("on", &self.on())
            .field("real_indicator", &self.real_indicator())
            .field("ndx", &self.ndx())
            .field("map_flags", &self.map_flags())
            .field("map_which_groups", &self.map_which_groups())
            .field("map_groups", &self.map_groups())
            .field("map_which_mods", &self.map_which_mods())
            .field("map_mods", &self.map_mods())
            .field("map_real_mods", &self.map_real_mods())
            .field("map_vmod", &self.map_vmod())
            .field("map_ctrls", &self.map_ctrls())
            .field("supported", &self.supported())
            .field("pad", &3)
            .finish()
    }
}

impl Drop for GetNamedIndicatorReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetNamedIndicatorReply {}
unsafe impl std::marker::Sync for GetNamedIndicatorReply {}

/// Cookie type for [GetNamedIndicator].
///
/// This cookie can be used to get a [GetNamedIndicatorReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetNamedIndicatorCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetNamedIndicator].
///
/// This cookie can be used to get a [GetNamedIndicatorReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetNamedIndicatorCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetNamedIndicatorCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetNamedIndicatorCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetNamedIndicatorCookie {
}

unsafe impl base::CookieWithReplyChecked for GetNamedIndicatorCookie {
    type Reply = GetNamedIndicatorReply;
}

impl base::Cookie for GetNamedIndicatorCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetNamedIndicatorCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetNamedIndicatorCookieUnchecked {
    type Reply = GetNamedIndicatorReply;
}

/// The `GetNamedIndicator` request.
///
/// This request replies [GetNamedIndicatorReply].
///
/// Associated cookie types are [GetNamedIndicatorCookie] and [GetNamedIndicatorCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetNamedIndicator {
    pub device_spec: DeviceSpec,
    pub led_class: LedClass,
    pub led_id: IdSpec,
    pub indicator: xproto::Atom,
}

unsafe impl base::RawRequest for GetNamedIndicator {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 15,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (std::mem::transmute::<_, u32>(self.led_class) as LedClassSpec).serialize(&mut buf0[6 .. ]);
        self.led_id.serialize(&mut buf0[8 .. ]);
        self.indicator.serialize(&mut buf0[12 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetNamedIndicator {
    type Cookie = GetNamedIndicatorCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetNamedIndicator {
    type Reply = GetNamedIndicatorReply;
    type Cookie = GetNamedIndicatorCookie;
    type CookieUnchecked = GetNamedIndicatorCookieUnchecked;
}

/// The `SetNamedIndicator` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetNamedIndicator {
    pub device_spec: DeviceSpec,
    pub led_class: LedClass,
    pub led_id: IdSpec,
    pub indicator: xproto::Atom,
    pub set_state: bool,
    pub on: bool,
    pub set_map: bool,
    pub create_map: bool,
    pub map_flags: ImFlag,
    pub map_which_groups: ImGroupsWhich,
    pub map_groups: SetOfGroups,
    pub map_which_mods: ImModsWhich,
    pub map_real_mods: xproto::ModMask,
    pub map_vmods: VMod,
    pub map_ctrls: BoolCtrl,
}

unsafe impl base::RawRequest for SetNamedIndicator {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 16,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 32];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (std::mem::transmute::<_, u32>(self.led_class) as LedClassSpec).serialize(&mut buf0[6 .. ]);
        self.led_id.serialize(&mut buf0[8 .. ]);
        self.indicator.serialize(&mut buf0[12 .. ]);
        (if self.set_state { 1u8 } else { 0u8 }).serialize(&mut buf0[16 .. ]);
        (if self.on { 1u8 } else { 0u8 }).serialize(&mut buf0[17 .. ]);
        (if self.set_map { 1u8 } else { 0u8 }).serialize(&mut buf0[18 .. ]);
        (if self.create_map { 1u8 } else { 0u8 }).serialize(&mut buf0[19 .. ]);
        (self.map_flags.bits() as u8).serialize(&mut buf0[21 .. ]);
        (self.map_which_groups.bits() as u8).serialize(&mut buf0[22 .. ]);
        (self.map_groups.bits() as u8).serialize(&mut buf0[23 .. ]);
        (self.map_which_mods.bits() as u8).serialize(&mut buf0[24 .. ]);
        (self.map_real_mods.bits() as u8).serialize(&mut buf0[25 .. ]);
        (self.map_vmods.bits() as u16).serialize(&mut buf0[26 .. ]);
        self.map_ctrls.bits().serialize(&mut buf0[28 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 32;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for SetNamedIndicator {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetNamedIndicator {
}

/// Reply type for [GetNames].
///
/// Can be obtained from a [GetNamesCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetNamesCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetNamesReply {
    raw: *const u8,
}

impl GetNamesReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn which(&self) -> NameDetail {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, NameDetail>(val)
        }
    }

    pub fn min_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn max_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    fn n_types(&self) -> u8 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn group_names(&self) -> SetOfGroup {
        unsafe {
            let offset = 15usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, SetOfGroup>(val)
        }
    }

    pub fn virtual_mods(&self) -> VMod {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, VMod>(val)
        }
    }

    pub fn first_key(&self) -> xproto::Keycode {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    fn n_keys(&self) -> u8 {
        unsafe {
            let offset = 19usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn indicators(&self) -> u32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    fn n_radio_groups(&self) -> u8 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn n_key_aliases(&self) -> u8 {
        unsafe {
            let offset = 25usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn n_kt_levels(&self) -> u16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }


    pub fn value_list(&self) -> Vec<GetNamesReplyValueList> {
        let which = self.which().bits();
        let group_names = self.group_names().bits();
        let indicators = self.indicators();
        let n_key_aliases = self.n_key_aliases();
        let n_keys = self.n_keys();
        let n_radio_groups = self.n_radio_groups();
        let n_types = self.n_types();
        let virtual_mods = self.virtual_mods().bits();
        let params = GetNamesReplyValueListParams {which: which as usize, group_names: group_names as usize, indicators: indicators as usize, n_key_aliases: n_key_aliases as usize, n_keys: n_keys as usize, n_radio_groups: n_radio_groups as usize, n_types: n_types as usize, virtual_mods: virtual_mods as usize};
        let mut offset = 32usize;
        unsafe {
            <Vec<GetNamesReplyValueList>>::unserialize(
                self.wire_ptr().add(offset), params, &mut offset
            )
        }
    }
}

impl base::Reply for GetNamesReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetNamesReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetNamesReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("which", &self.which())
            .field("min_key_code", &self.min_key_code())
            .field("max_key_code", &self.max_key_code())
            .field("n_types", &self.n_types())
            .field("group_names", &self.group_names())
            .field("virtual_mods", &self.virtual_mods())
            .field("first_key", &self.first_key())
            .field("n_keys", &self.n_keys())
            .field("indicators", &self.indicators())
            .field("n_radio_groups", &self.n_radio_groups())
            .field("n_key_aliases", &self.n_key_aliases())
            .field("n_kt_levels", &self.n_kt_levels())
            .field("pad", &4)
            .field("value_list", &self.value_list())
            .finish()
    }
}

impl Drop for GetNamesReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetNamesReply {}
unsafe impl std::marker::Sync for GetNamesReply {}

/// Cookie type for [GetNames].
///
/// This cookie can be used to get a [GetNamesReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetNamesCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetNames].
///
/// This cookie can be used to get a [GetNamesReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetNamesCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetNamesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetNamesCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetNamesCookie {
}

unsafe impl base::CookieWithReplyChecked for GetNamesCookie {
    type Reply = GetNamesReply;
}

impl base::Cookie for GetNamesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetNamesCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetNamesCookieUnchecked {
    type Reply = GetNamesReply;
}

/// The `GetNames` request.
///
/// This request replies [GetNamesReply].
///
/// Associated cookie types are [GetNamesCookie] and [GetNamesCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetNames {
    pub device_spec: DeviceSpec,
    pub which: NameDetail,
}

unsafe impl base::RawRequest for GetNames {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 17,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        self.which.bits().serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetNames {
    type Cookie = GetNamesCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetNames {
    type Reply = GetNamesReply;
    type Cookie = GetNamesCookie;
    type CookieUnchecked = GetNamesCookieUnchecked;
}

/// The `SetNames` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetNames<'a> {
    pub device_spec: DeviceSpec,
    pub virtual_mods: VMod,
    pub first_type: u8,
    pub n_types: u8,
    pub first_kt_levelt: u8,
    pub n_kt_levels: u8,
    pub indicators: u32,
    pub group_names: SetOfGroup,
    pub n_radio_groups: u8,
    pub first_key: xproto::Keycode,
    pub n_keys: u8,
    pub n_key_aliases: u8,
    pub total_kt_level_names: u16,
    pub values: &'a [SetNamesValues],
}

unsafe impl<'a> base::RawRequest for SetNames<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(SetNamesValues::is_sorted_distinct(self.values), "SetNames::values must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 18,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 28];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (self.virtual_mods.bits() as u16).serialize(&mut buf0[6 .. ]);
        (SetNamesValues::get_mask(self.values).bits() as u32).serialize(&mut buf0[8 .. ]);
        self.first_type.serialize(&mut buf0[12 .. ]);
        self.n_types.serialize(&mut buf0[13 .. ]);
        self.first_kt_levelt.serialize(&mut buf0[14 .. ]);
        self.n_kt_levels.serialize(&mut buf0[15 .. ]);
        self.indicators.serialize(&mut buf0[16 .. ]);
        (self.group_names.bits() as u8).serialize(&mut buf0[20 .. ]);
        self.n_radio_groups.serialize(&mut buf0[21 .. ]);
        self.first_key.serialize(&mut buf0[22 .. ]);
        self.n_keys.serialize(&mut buf0[23 .. ]);
        self.n_key_aliases.serialize(&mut buf0[24 .. ]);
        self.total_kt_level_names.serialize(&mut buf0[26 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.values.wire_len();
        let mut buf1 = vec![0u8; len1];
        self.values.serialize(&mut buf1);
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetNames<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetNames<'a> {
}

/// Reply type for [PerClientFlags].
///
/// Can be obtained from a [PerClientFlagsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [PerClientFlagsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct PerClientFlagsReply {
    raw: *const u8,
}

impl PerClientFlagsReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn supported(&self) -> PerClientFlag {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, PerClientFlag>(val)
        }
    }

    pub fn value(&self) -> PerClientFlag {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, PerClientFlag>(val)
        }
    }

    pub fn auto_ctrls(&self) -> BoolCtrl {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BoolCtrl>(val)
        }
    }

    pub fn auto_ctrls_values(&self) -> BoolCtrl {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, BoolCtrl>(val)
        }
    }

}

impl base::Reply for PerClientFlagsReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for PerClientFlagsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PerClientFlagsReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("supported", &self.supported())
            .field("value", &self.value())
            .field("auto_ctrls", &self.auto_ctrls())
            .field("auto_ctrls_values", &self.auto_ctrls_values())
            .field("pad", &8)
            .finish()
    }
}

impl Drop for PerClientFlagsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for PerClientFlagsReply {}
unsafe impl std::marker::Sync for PerClientFlagsReply {}

/// Cookie type for [PerClientFlags].
///
/// This cookie can be used to get a [PerClientFlagsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct PerClientFlagsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [PerClientFlags].
///
/// This cookie can be used to get a [PerClientFlagsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct PerClientFlagsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for PerClientFlagsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        PerClientFlagsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for PerClientFlagsCookie {
}

unsafe impl base::CookieWithReplyChecked for PerClientFlagsCookie {
    type Reply = PerClientFlagsReply;
}

impl base::Cookie for PerClientFlagsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        PerClientFlagsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for PerClientFlagsCookieUnchecked {
    type Reply = PerClientFlagsReply;
}

/// The `PerClientFlags` request.
///
/// This request replies [PerClientFlagsReply].
///
/// Associated cookie types are [PerClientFlagsCookie] and [PerClientFlagsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct PerClientFlags {
    pub device_spec: DeviceSpec,
    pub change: PerClientFlag,
    pub value: PerClientFlag,
    pub ctrls_to_change: BoolCtrl,
    pub auto_ctrls: BoolCtrl,
    pub auto_ctrls_values: BoolCtrl,
}

unsafe impl base::RawRequest for PerClientFlags {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 21,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 28];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        self.change.bits().serialize(&mut buf0[8 .. ]);
        self.value.bits().serialize(&mut buf0[12 .. ]);
        self.ctrls_to_change.bits().serialize(&mut buf0[16 .. ]);
        self.auto_ctrls.bits().serialize(&mut buf0[20 .. ]);
        self.auto_ctrls_values.bits().serialize(&mut buf0[24 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for PerClientFlags {
    type Cookie = PerClientFlagsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for PerClientFlags {
    type Reply = PerClientFlagsReply;
    type Cookie = PerClientFlagsCookie;
    type CookieUnchecked = PerClientFlagsCookieUnchecked;
}

/// Reply type for [ListComponents].
///
/// Can be obtained from a [ListComponentsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [ListComponentsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListComponentsReply {
    raw: *const u8,
}

impl ListComponentsReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    unsafe fn compute_keycodes_offset(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // n_keymaps
        let n_keymaps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_keycodes
        let n_keycodes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_types
        let n_types = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_compat_maps
        let n_compat_maps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_symbols
        let n_symbols = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_geometries
        let n_geometries = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // extra
        sz += 2usize;
        // pad
        sz += 10usize;
        // keymaps
        for _ in 0 .. (n_keymaps as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }

    unsafe fn compute_types_offset(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // n_keymaps
        let n_keymaps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_keycodes
        let n_keycodes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_types
        let n_types = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_compat_maps
        let n_compat_maps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_symbols
        let n_symbols = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_geometries
        let n_geometries = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // extra
        sz += 2usize;
        // pad
        sz += 10usize;
        // keymaps
        for _ in 0 .. (n_keymaps as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // keycodes
        for _ in 0 .. (n_keycodes as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }

    unsafe fn compute_compat_maps_offset(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // n_keymaps
        let n_keymaps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_keycodes
        let n_keycodes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_types
        let n_types = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_compat_maps
        let n_compat_maps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_symbols
        let n_symbols = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_geometries
        let n_geometries = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // extra
        sz += 2usize;
        // pad
        sz += 10usize;
        // keymaps
        for _ in 0 .. (n_keymaps as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // keycodes
        for _ in 0 .. (n_keycodes as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // types
        for _ in 0 .. (n_types as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }

    unsafe fn compute_symbols_offset(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // n_keymaps
        let n_keymaps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_keycodes
        let n_keycodes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_types
        let n_types = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_compat_maps
        let n_compat_maps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_symbols
        let n_symbols = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_geometries
        let n_geometries = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // extra
        sz += 2usize;
        // pad
        sz += 10usize;
        // keymaps
        for _ in 0 .. (n_keymaps as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // keycodes
        for _ in 0 .. (n_keycodes as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // types
        for _ in 0 .. (n_types as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // compat_maps
        for _ in 0 .. (n_compat_maps as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }

    unsafe fn compute_geometries_offset(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // n_keymaps
        let n_keymaps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_keycodes
        let n_keycodes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_types
        let n_types = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_compat_maps
        let n_compat_maps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_symbols
        let n_symbols = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_geometries
        let n_geometries = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // extra
        sz += 2usize;
        // pad
        sz += 10usize;
        // keymaps
        for _ in 0 .. (n_keymaps as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // keycodes
        for _ in 0 .. (n_keycodes as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // types
        for _ in 0 .. (n_types as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // compat_maps
        for _ in 0 .. (n_compat_maps as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // symbols
        for _ in 0 .. (n_symbols as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }

    unsafe fn compute_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // n_keymaps
        let n_keymaps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_keycodes
        let n_keycodes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_types
        let n_types = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_compat_maps
        let n_compat_maps = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_symbols
        let n_symbols = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // n_geometries
        let n_geometries = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // extra
        sz += 2usize;
        // pad
        sz += 10usize;
        // keymaps
        for _ in 0 .. (n_keymaps as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // keycodes
        for _ in 0 .. (n_keycodes as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // types
        for _ in 0 .. (n_types as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // compat_maps
        for _ in 0 .. (n_compat_maps as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // symbols
        for _ in 0 .. (n_symbols as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        // geometries
        for _ in 0 .. (n_geometries as usize) {
            sz += <&Listing>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    fn n_keymaps(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    fn n_keycodes(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    fn n_types(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    fn n_compat_maps(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    fn n_symbols(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    fn n_geometries(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn extra(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }


    pub fn keymaps(&self) -> ListingIterator<'_> {
        unsafe {
            let offset = 32usize;
            ListingIterator {
                params: (),
                rem: (self.n_keymaps() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }

    pub fn keycodes(&self) -> ListingIterator<'_> {
        unsafe {
            let offset = Self::compute_keycodes_offset(self.wire_ptr(), ());
            ListingIterator {
                params: (),
                rem: (self.n_keycodes() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }

    pub fn types(&self) -> ListingIterator<'_> {
        unsafe {
            let offset = Self::compute_types_offset(self.wire_ptr(), ());
            ListingIterator {
                params: (),
                rem: (self.n_types() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }

    pub fn compat_maps(&self) -> ListingIterator<'_> {
        unsafe {
            let offset = Self::compute_compat_maps_offset(self.wire_ptr(), ());
            ListingIterator {
                params: (),
                rem: (self.n_compat_maps() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }

    pub fn symbols(&self) -> ListingIterator<'_> {
        unsafe {
            let offset = Self::compute_symbols_offset(self.wire_ptr(), ());
            ListingIterator {
                params: (),
                rem: (self.n_symbols() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }

    pub fn geometries(&self) -> ListingIterator<'_> {
        unsafe {
            let offset = Self::compute_geometries_offset(self.wire_ptr(), ());
            ListingIterator {
                params: (),
                rem: (self.n_geometries() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::Reply for ListComponentsReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for ListComponentsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListComponentsReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("n_keymaps", &self.n_keymaps())
            .field("n_keycodes", &self.n_keycodes())
            .field("n_types", &self.n_types())
            .field("n_compat_maps", &self.n_compat_maps())
            .field("n_symbols", &self.n_symbols())
            .field("n_geometries", &self.n_geometries())
            .field("extra", &self.extra())
            .field("pad", &10)
            .field("keymaps", &self.keymaps())
            .field("keycodes", &self.keycodes())
            .field("types", &self.types())
            .field("compat_maps", &self.compat_maps())
            .field("symbols", &self.symbols())
            .field("geometries", &self.geometries())
            .finish()
    }
}

impl Drop for ListComponentsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for ListComponentsReply {}
unsafe impl std::marker::Sync for ListComponentsReply {}

/// Cookie type for [ListComponents].
///
/// This cookie can be used to get a [ListComponentsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct ListComponentsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [ListComponents].
///
/// This cookie can be used to get a [ListComponentsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListComponentsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for ListComponentsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListComponentsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for ListComponentsCookie {
}

unsafe impl base::CookieWithReplyChecked for ListComponentsCookie {
    type Reply = ListComponentsReply;
}

impl base::Cookie for ListComponentsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListComponentsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for ListComponentsCookieUnchecked {
    type Reply = ListComponentsReply;
}

/// The `ListComponents` request.
///
/// This request replies [ListComponentsReply].
///
/// Associated cookie types are [ListComponentsCookie] and [ListComponentsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct ListComponents {
    pub device_spec: DeviceSpec,
    pub max_names: u16,
}

unsafe impl base::RawRequest for ListComponents {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 22,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        self.max_names.serialize(&mut buf0[6 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for ListComponents {
    type Cookie = ListComponentsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for ListComponents {
    type Reply = ListComponentsReply;
    type Cookie = ListComponentsCookie;
    type CookieUnchecked = ListComponentsCookieUnchecked;
}

/// Reply type for [GetKbdByName].
///
/// Can be obtained from a [GetKbdByNameCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetKbdByNameCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetKbdByNameReply {
    raw: *const u8,
}

impl GetKbdByNameReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn min_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn max_key_code(&self) -> xproto::Keycode {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Keycode;
            ptr.read_unaligned()
        }
    }

    pub fn loaded(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(10usize)) };
        val != 0
    }

    pub fn new_keyboard(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(11usize)) };
        val != 0
    }

    pub fn found(&self) -> GbnDetail {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, GbnDetail>(val)
        }
    }

    pub fn reported(&self) -> GbnDetail {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, GbnDetail>(val)
        }
    }


    pub fn replies(&self) -> Vec<GetKbdByNameReplyReplies> {
        let reported = self.reported().bits();
        let params = GetKbdByNameReplyRepliesParams {reported: reported as usize};
        let mut offset = 32usize;
        unsafe {
            <Vec<GetKbdByNameReplyReplies>>::unserialize(
                self.wire_ptr().add(offset), params, &mut offset
            )
        }
    }
}

impl base::Reply for GetKbdByNameReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetKbdByNameReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetKbdByNameReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("min_key_code", &self.min_key_code())
            .field("max_key_code", &self.max_key_code())
            .field("loaded", &self.loaded())
            .field("new_keyboard", &self.new_keyboard())
            .field("found", &self.found())
            .field("reported", &self.reported())
            .field("pad", &16)
            .field("replies", &self.replies())
            .finish()
    }
}

impl Drop for GetKbdByNameReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetKbdByNameReply {}
unsafe impl std::marker::Sync for GetKbdByNameReply {}

/// Cookie type for [GetKbdByName].
///
/// This cookie can be used to get a [GetKbdByNameReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetKbdByNameCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetKbdByName].
///
/// This cookie can be used to get a [GetKbdByNameReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetKbdByNameCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetKbdByNameCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetKbdByNameCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetKbdByNameCookie {
}

unsafe impl base::CookieWithReplyChecked for GetKbdByNameCookie {
    type Reply = GetKbdByNameReply;
}

impl base::Cookie for GetKbdByNameCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetKbdByNameCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetKbdByNameCookieUnchecked {
    type Reply = GetKbdByNameReply;
}

/// The `GetKbdByName` request.
///
/// This request replies [GetKbdByNameReply].
///
/// Associated cookie types are [GetKbdByNameCookie] and [GetKbdByNameCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetKbdByName {
    pub device_spec: DeviceSpec,
    pub need: GbnDetail,
    pub want: GbnDetail,
    pub load: bool,
}

unsafe impl base::RawRequest for GetKbdByName {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 23,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (self.need.bits() as u16).serialize(&mut buf0[6 .. ]);
        (self.want.bits() as u16).serialize(&mut buf0[8 .. ]);
        (if self.load { 1u8 } else { 0u8 }).serialize(&mut buf0[10 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetKbdByName {
    type Cookie = GetKbdByNameCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetKbdByName {
    type Reply = GetKbdByNameReply;
    type Cookie = GetKbdByNameCookie;
    type CookieUnchecked = GetKbdByNameCookieUnchecked;
}

/// Reply type for [GetDeviceInfo].
///
/// Can be obtained from a [GetDeviceInfoCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetDeviceInfoCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetDeviceInfoReply {
    raw: *const u8,
}

impl GetDeviceInfoReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    unsafe fn compute_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // device_id
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // present
        sz += 2usize;
        // supported
        sz += 2usize;
        // unsupported
        sz += 2usize;
        // n_device_led_f_bs
        let n_device_led_f_bs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // first_btn_wanted
        sz += 1usize;
        // n_btns_wanted
        sz += 1usize;
        // first_btn_rtrn
        sz += 1usize;
        // n_btns_rtrn
        let n_btns_rtrn = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // total_btns
        sz += 1usize;
        // has_own_state
        sz += 1usize;
        // dflt_kbd_fb
        sz += 2usize;
        // dflt_led_fb
        sz += 2usize;
        // pad
        sz += 2usize;
        // dev_type
        sz += 4usize;
        // name_len
        let name_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // name
        sz += (name_len as usize);
        // align pad
        sz += base::align_pad(sz, 4);
        // btn_actions
        sz += ((n_btns_rtrn as usize) * 8usize);
        // leds
        for _ in 0 .. (n_device_led_f_bs as usize) {
            sz += <&DeviceLedInfo>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn device_id(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn present(&self) -> XiFeature {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, XiFeature>(val)
        }
    }

    pub fn supported(&self) -> XiFeature {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, XiFeature>(val)
        }
    }

    pub fn unsupported(&self) -> XiFeature {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = ptr.read_unaligned() as u32;
            std::mem::transmute::<u32, XiFeature>(val)
        }
    }

    fn n_device_led_f_bs(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn first_btn_wanted(&self) -> u8 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn n_btns_wanted(&self) -> u8 {
        unsafe {
            let offset = 17usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn first_btn_rtrn(&self) -> u8 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    fn n_btns_rtrn(&self) -> u8 {
        unsafe {
            let offset = 19usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn total_btns(&self) -> u8 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }

    pub fn has_own_state(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(21usize)) };
        val != 0
    }

    pub fn dflt_kbd_fb(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn dflt_led_fb(&self) -> u16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }


    pub fn dev_type(&self) -> xproto::Atom {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            ptr.read_unaligned()
        }
    }

    fn name_len(&self) -> u16 {
        unsafe {
            let offset = 32usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn name(&self) -> &[String8] {
        unsafe {
            let offset = 34usize;
            let len = (self.name_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const String8;
            std::slice::from_raw_parts(ptr, len)
        }
    }


    pub fn btn_actions(&self) -> &[Action] {
        unsafe {
            let offset = ((34usize + (self.name_len() as usize)) + base::align_pad((34usize + (self.name_len() as usize)), 4));
            let len = (self.n_btns_rtrn() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Action;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn leds(&self) -> DeviceLedInfoIterator<'_> {
        unsafe {
            let offset = (((34usize + (self.name_len() as usize)) + base::align_pad((34usize + (self.name_len() as usize)), 4)) + ((self.n_btns_rtrn() as usize) * 8usize));
            DeviceLedInfoIterator {
                params: (),
                rem: (self.n_device_led_f_bs() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::Reply for GetDeviceInfoReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetDeviceInfoReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetDeviceInfoReply")
            .field("response_type", &self.response_type())
            .field("device_id", &self.device_id())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("present", &self.present())
            .field("supported", &self.supported())
            .field("unsupported", &self.unsupported())
            .field("n_device_led_f_bs", &self.n_device_led_f_bs())
            .field("first_btn_wanted", &self.first_btn_wanted())
            .field("n_btns_wanted", &self.n_btns_wanted())
            .field("first_btn_rtrn", &self.first_btn_rtrn())
            .field("n_btns_rtrn", &self.n_btns_rtrn())
            .field("total_btns", &self.total_btns())
            .field("has_own_state", &self.has_own_state())
            .field("dflt_kbd_fb", &self.dflt_kbd_fb())
            .field("dflt_led_fb", &self.dflt_led_fb())
            .field("pad", &2)
            .field("dev_type", &self.dev_type())
            .field("name_len", &self.name_len())
            .field("name", &self.name())
            .field("align_pad", &4)
            .field("btn_actions", &self.btn_actions())
            .field("leds", &self.leds())
            .finish()
    }
}

impl Drop for GetDeviceInfoReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetDeviceInfoReply {}
unsafe impl std::marker::Sync for GetDeviceInfoReply {}

/// Cookie type for [GetDeviceInfo].
///
/// This cookie can be used to get a [GetDeviceInfoReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetDeviceInfoCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetDeviceInfo].
///
/// This cookie can be used to get a [GetDeviceInfoReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetDeviceInfoCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetDeviceInfoCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetDeviceInfoCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetDeviceInfoCookie {
}

unsafe impl base::CookieWithReplyChecked for GetDeviceInfoCookie {
    type Reply = GetDeviceInfoReply;
}

impl base::Cookie for GetDeviceInfoCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetDeviceInfoCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetDeviceInfoCookieUnchecked {
    type Reply = GetDeviceInfoReply;
}

/// The `GetDeviceInfo` request.
///
/// This request replies [GetDeviceInfoReply].
///
/// Associated cookie types are [GetDeviceInfoCookie] and [GetDeviceInfoCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetDeviceInfo {
    pub device_spec: DeviceSpec,
    pub wanted: XiFeature,
    pub all_buttons: bool,
    pub first_button: u8,
    pub n_buttons: u8,
    pub led_class: LedClass,
    pub led_id: IdSpec,
}

unsafe impl base::RawRequest for GetDeviceInfo {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 24,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        (self.wanted.bits() as u16).serialize(&mut buf0[6 .. ]);
        (if self.all_buttons { 1u8 } else { 0u8 }).serialize(&mut buf0[8 .. ]);
        self.first_button.serialize(&mut buf0[9 .. ]);
        self.n_buttons.serialize(&mut buf0[10 .. ]);
        (std::mem::transmute::<_, u32>(self.led_class) as LedClassSpec).serialize(&mut buf0[12 .. ]);
        self.led_id.serialize(&mut buf0[14 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetDeviceInfo {
    type Cookie = GetDeviceInfoCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetDeviceInfo {
    type Reply = GetDeviceInfoReply;
    type Cookie = GetDeviceInfoCookie;
    type CookieUnchecked = GetDeviceInfoCookieUnchecked;
}

/// The `SetDeviceInfo` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetDeviceInfo<'a> {
    pub device_spec: DeviceSpec,
    pub first_btn: u8,
    pub change: XiFeature,
    pub btn_actions: &'a [Action],
    pub leds: &'a [DeviceLedInfoBuf],
}

unsafe impl<'a> base::RawRequest for SetDeviceInfo<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 6,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 25,
            isvoid: 1,
        };

        let mut sections: [iovec; 8] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 8];

        let buf0: &mut [u8] = &mut [0; 12];
        self.device_spec.serialize(&mut buf0[4 .. ]);
        self.first_btn.serialize(&mut buf0[6 .. ]);
        (self.btn_actions.len() as u8).serialize(&mut buf0[7 .. ]);
        (self.change.bits() as u16).serialize(&mut buf0[8 .. ]);
        (self.leds.len() as u16).serialize(&mut buf0[10 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.btn_actions.iter().map(|el| el.wire_len()).sum::<usize>();
        let mut buf1 = vec![0u8; len1];
        let mut offset1 = 0usize;
        for el in self.btn_actions {
            offset1 += el.serialize(&mut buf1[offset1 ..]);
        }
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let len2: usize = self.leds.iter().map(|el| el.wire_len()).sum();
        let mut buf2 = vec![0u8; len2];
        let mut offset = 0usize;
        for el in self.leds {
            offset += el.serialize(&mut buf2[offset..]);
        }
        sections[6].iov_base = buf2.as_ptr() as *mut _;
        sections[6].iov_len = buf2.len();
        sections[7].iov_len = base::align_pad(sections[6].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetDeviceInfo<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetDeviceInfo<'a> {
}

/// Reply type for [SetDebuggingFlags].
///
/// Can be obtained from a [SetDebuggingFlagsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [SetDebuggingFlagsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetDebuggingFlagsReply {
    raw: *const u8,
}

impl SetDebuggingFlagsReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            ptr.read_unaligned()
        }
    }


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            ptr.read_unaligned()
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn current_flags(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn current_ctrls(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn supported_flags(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

    pub fn supported_ctrls(&self) -> u32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            ptr.read_unaligned()
        }
    }

}

impl base::Reply for SetDebuggingFlagsReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for SetDebuggingFlagsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetDebuggingFlagsReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("current_flags", &self.current_flags())
            .field("current_ctrls", &self.current_ctrls())
            .field("supported_flags", &self.supported_flags())
            .field("supported_ctrls", &self.supported_ctrls())
            .field("pad", &8)
            .finish()
    }
}

impl Drop for SetDebuggingFlagsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for SetDebuggingFlagsReply {}
unsafe impl std::marker::Sync for SetDebuggingFlagsReply {}

/// Cookie type for [SetDebuggingFlags].
///
/// This cookie can be used to get a [SetDebuggingFlagsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct SetDebuggingFlagsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [SetDebuggingFlags].
///
/// This cookie can be used to get a [SetDebuggingFlagsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetDebuggingFlagsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for SetDebuggingFlagsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetDebuggingFlagsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for SetDebuggingFlagsCookie {
}

unsafe impl base::CookieWithReplyChecked for SetDebuggingFlagsCookie {
    type Reply = SetDebuggingFlagsReply;
}

impl base::Cookie for SetDebuggingFlagsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetDebuggingFlagsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for SetDebuggingFlagsCookieUnchecked {
    type Reply = SetDebuggingFlagsReply;
}

/// The `SetDebuggingFlags` request.
///
/// This request replies [SetDebuggingFlagsReply].
///
/// Associated cookie types are [SetDebuggingFlagsCookie] and [SetDebuggingFlagsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct SetDebuggingFlags<'a> {
    pub affect_flags: u32,
    pub flags: u32,
    pub affect_ctrls: u32,
    pub ctrls: u32,
    pub message: &'a [String8],
}

unsafe impl<'a> base::RawRequest for SetDebuggingFlags<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 101,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 24];
        (self.message.len() as u16).serialize(&mut buf0[4 .. ]);
        self.affect_flags.serialize(&mut buf0[8 .. ]);
        self.flags.serialize(&mut buf0[12 .. ]);
        self.affect_ctrls.serialize(&mut buf0[16 .. ]);
        self.ctrls.serialize(&mut buf0[20 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.message.as_ptr() as *mut _;
        sections[4].iov_len = self.message.len() * std::mem::size_of::<String8>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetDebuggingFlags<'a> {
    type Cookie = SetDebuggingFlagsCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for SetDebuggingFlags<'a> {
    type Reply = SetDebuggingFlagsReply;
    type Cookie = SetDebuggingFlagsCookie;
    type CookieUnchecked = SetDebuggingFlagsCookieUnchecked;
}
