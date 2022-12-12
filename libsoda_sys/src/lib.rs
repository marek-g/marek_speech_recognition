use libloading::{library_filename, Library, Symbol};
use std::ffi::{c_char, c_int, c_void};
use std::path::PathBuf;

include!(concat!(env!("OUT_DIR"), "/speech.soda.chrome.rs"));

/// The callback that gets executed on a SODA event. It takes in a
/// char*, which is a serialized SodaResponse proto, an int specifying the
/// length of the char* and a void pointer to the object that is associated
/// with the callback.
pub type SodaResultHandler = unsafe extern "C" fn(
    response: *const c_char,
    res_length: c_int,
    callback_handle: *const c_void,
);

#[repr(C)]
pub struct SodaConfig {
    /// A ExtendedSodaConfigMsg that's been serialized as a string. Not owned.
    pub soda_config: *const c_char,

    /// Length of char* in soda_config.
    pub soda_config_size: c_int,

    // The callback that gets executed on a SODA event.
    pub callback: Option<SodaResultHandler>,

    /// A void pointer to the object that is associated with the callback.
    pub callback_handle: *const c_void,
}

pub type SodaHandle = *mut c_void;

type CreateSodaAsync = unsafe extern "C" fn(SodaConfig) -> SodaHandle;
type DeleteSodaAsync = unsafe extern "C" fn(SodaHandle);
type SodaStart = unsafe extern "C" fn(SodaHandle);
type SodaStop = unsafe extern "C" fn(SodaHandle);
type AddAudio = unsafe extern "C" fn(SodaHandle, buffer: *const c_char, buffer_length: c_int);
type SodaMarkDone = unsafe extern "C" fn(SodaHandle);

pub struct LibSoda {
    _library: Library,

    pub create_soda_async: CreateSodaAsync,
    pub delete_soda_async: DeleteSodaAsync,
    pub soda_start: SodaStart,
    pub soda_stop: SodaStop,
    pub add_audio: AddAudio,
    pub mark_done: SodaMarkDone,
}

impl LibSoda {
    pub fn load<T: Into<PathBuf>>(path: T) -> Result<Self, libloading::Error> {
        unsafe {
            let library = Library::new(path.into().join(library_filename("soda")))?;

            let func: Symbol<CreateSodaAsync> = library.get(b"CreateExtendedSodaAsync\0")?;
            let create_soda_async = std::mem::transmute(func.into_raw().into_raw());

            let func: Symbol<DeleteSodaAsync> = library.get(b"DeleteExtendedSodaAsync\0")?;
            let delete_soda_async = std::mem::transmute(func.into_raw().into_raw());

            let func: Symbol<SodaStart> = library.get(b"ExtendedSodaStart\0")?;
            let soda_start = std::mem::transmute(func.into_raw().into_raw());

            let func: Symbol<SodaStop> = library.get(b"ExtendedSodaStop\0")?;
            let soda_stop = std::mem::transmute(func.into_raw().into_raw());

            let func: Symbol<AddAudio> = library.get(b"ExtendedAddAudio\0")?;
            let add_audio = std::mem::transmute(func.into_raw().into_raw());

            let func: Symbol<SodaMarkDone> = library.get(b"ExtendedSodaMarkDone\0")?;
            let mark_done = std::mem::transmute(func.into_raw().into_raw());

            Ok(Self {
                _library: library,

                create_soda_async,
                delete_soda_async,
                soda_start,
                soda_stop,
                add_audio,
                mark_done,
            })
        }
    }
}
