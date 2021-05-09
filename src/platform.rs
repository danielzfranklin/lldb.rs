// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::error::SBError;
use super::launchinfo::SBLaunchInfo;
use super::lldb_pid_t;
use std::ffi::{CStr, CString};
use std::fmt::Write;
use sys;

/// A platform that can represent the current host or a
/// remote host debug platform.
///
/// The `SBPlatform` class represents the current host, or a remote host.
/// It can be connected to a remote platform in order to provide ways
/// to remotely launch and attach to processes, upload/download files,
/// create directories, run remote shell commands, find locally cached
/// versions of files from the remote system, and much more.
///
/// `SBPlatform` objects can be created and then used to connect to a remote
/// platform which allows the `SBPlatform` to be used to get a list of the
/// current processes on the remote host, attach to one of those processes,
/// install programs on the remote system, attach and launch processes,
/// and much more.
///
/// Every [`SBTarget`] has a corresponding `SBPlatform`. The platform can be
/// specified upon target creation, or the currently selected platform
/// will attempt to be used when creating the target automatically as long
/// as the currently selected platform matches the target architecture
/// and executable type. If the architecture or executable type do not match,
/// a suitable platform will be found automatically.
///
/// [`SBTarget`]: struct.SBTarget.html
#[derive(Debug)]
pub struct SBPlatform {
    /// The underlying raw `SBPlatformRef`.
    pub raw: sys::SBPlatformRef,
}

impl SBPlatform {
    /// Construct a new `SBPlatform`.
    pub fn wrap(raw: sys::SBPlatformRef) -> SBPlatform {
        SBPlatform { raw }
    }

    /// Construct a new `Some(SBPlatform)` or `None`.
    pub fn maybe_wrap(raw: sys::SBPlatformRef) -> Option<SBPlatform> {
        if unsafe { sys::SBPlatformIsValid(raw) != 0 } {
            Some(SBPlatform { raw })
        } else {
            None
        }
    }

    /// Connect to a remote.
    ///
    /// Use [`Self::connect_remote_with_options`] if you need to provide a path
    /// or don't want to provide a port.
    pub fn connect_remote(
        &mut self,
        scheme: RemoteScheme,
        host: &str,
        port: u16,
    ) -> Result<(), SBError> {
        let options = RemoteConnectOptions::new(scheme, host, Some(port), None);
        self.connect_remote_with_options(options)
    }

    /// Connect to a remote.
    ///
    /// Use [`Self::connect_remote`] if you only need to provide a url scheme,
    /// host, and port.
    pub fn connect_remote_with_options(
        &mut self,
        options: RemoteConnectOptions,
    ) -> Result<(), SBError> {
        let result = SBError::wrap(unsafe { sys::SBPlatformConnectRemote(self.raw, options.0) });
        if result.is_success() {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Check whether or not this is a valid `SBPlatform` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBPlatformIsValid(self.raw) != 0 }
    }

    /// The working directory for this platform.
    pub fn working_directory(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetWorkingDirectory(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The name of the platform.
    ///
    /// When debugging on the host platform, this would be `"host"`.
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The triple used to describe this platform.
    ///
    /// An example value might be `"x86_64-apple-macosx"`.
    pub fn triple(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetTriple(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The hostname for this platform.
    pub fn hostname(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetHostname(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The build ID for the platforms' OS version.
    pub fn os_build(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetOSBuild(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The long form description of the platform's OS version.
    ///
    /// On Mac OS X, this might look like `"Darwin Kernel Version 15.5.0:
    /// Tue Apr 19 18:36:36 PDT 2016; root:xnu-3248.50.21~8/RELEASE_X86_64"`.
    pub fn os_description(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetOSDescription(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The major component of the platform's OS version.
    ///
    /// On Mac OS X 10.11.4, this would have the value `10`.
    pub fn os_major_version(&self) -> u32 {
        unsafe { sys::SBPlatformGetOSMajorVersion(self.raw) }
    }

    /// The minor component of the platform's OS version.
    ///
    /// On Mac OS X 10.11.4, this would have the value `11`.
    pub fn os_minor_version(&self) -> u32 {
        unsafe { sys::SBPlatformGetOSMinorVersion(self.raw) }
    }

    /// The patch or update component of the platform's OS version.
    ///
    /// On Mac OS X 10.11.4, this would have the value `4`.
    pub fn os_update_version(&self) -> u32 {
        unsafe { sys::SBPlatformGetOSUpdateVersion(self.raw) }
    }

    /// Launch a process. This is not for debugging that process.
    pub fn launch(&self, launch_info: &SBLaunchInfo) -> Result<(), SBError> {
        let error = SBError::wrap(unsafe { sys::SBPlatformLaunch(self.raw, launch_info.raw) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    /// Kill a process.
    pub fn kill(&self, pid: lldb_pid_t) -> Result<(), SBError> {
        let error = SBError::wrap(unsafe { sys::SBPlatformKill(self.raw, pid) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }
}

impl Clone for SBPlatform {
    fn clone(&self) -> SBPlatform {
        SBPlatform {
            raw: unsafe { sys::CloneSBPlatform(self.raw) },
        }
    }
}

impl Drop for SBPlatform {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBPlatform(self.raw) };
    }
}

unsafe impl Send for SBPlatform {}
unsafe impl Sync for SBPlatform {}

pub struct RemoteConnectOptions(sys::SBPlatformConnectOptionsRef);

impl RemoteConnectOptions {
    pub fn new(scheme: RemoteScheme, host: &str, port: Option<u16>, path: Option<&str>) -> Self {
        let url = Self::serialize_url(scheme, host, port, path);
        // NOTE: Based on source code we are transferring ownership of url
        // to the caller
        let url = Box::leak(Box::new(url));
        let raw = unsafe { sys::CreateSBPlatformConnectOptions(url.as_ptr()) };
        Self(raw)
    }

    fn serialize_url(
        scheme: RemoteScheme,
        host: &str,
        port: Option<u16>,
        path: Option<&str>,
    ) -> CString {
        // For details of URL format supported see <https://github.com/llvm/llvm-project/blob/d480f968ad8b56d3ee4a6b6df5532d485b0ad01e/lldb/source/Utility/UriParser.cpp>
        let mut url = format!("{}://{}", scheme.as_str(), host);
        if let Some(port) = port {
            write!(&mut url, ":{}", port).unwrap();
        }
        if let Some(path) = path {
            write!(&mut url, "/{}", path).unwrap();
        }

        CString::new(url).expect("URL doesn't contain nul")
    }

    pub fn wrap(raw: sys::SBPlatformConnectOptionsRef) -> Self {
        Self(raw)
    }

    // TODO: Setters and getters for URL, rsync, local cache dir
}

impl Clone for RemoteConnectOptions {
    fn clone(&self) -> Self {
        let raw = unsafe { sys::CloneSBPlatformConnectOptions(self.0) };
        Self(raw)
    }
}

impl Drop for RemoteConnectOptions {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBPlatformConnectOptions(self.0) };
    }
}

unsafe impl Send for RemoteConnectOptions {}
unsafe impl Sync for RemoteConnectOptions {}

pub enum RemoteScheme {
    // See <https://github.com/llvm/llvm-project/blob/d480f968ad8b56d3ee4a6b6df5532d485b0ad01e/lldb/source/Host/posix/ConnectionFileDescriptorPosix.cpp#L53>
    Listen,
    Accept,
    UnixAccept,
    Connect,
    TcpConnect,
    Udp,
    UnixConnect,
    UnixAbstractConnect,
    Fd,
    File,
}

impl RemoteScheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Listen => "listen",
            Self::Accept => "accept",
            Self::UnixAccept => "unix-accept",
            Self::Connect => "connect",
            Self::TcpConnect => "tcp-connect",
            Self::Udp => "udp",
            Self::UnixConnect => "unix-connect",
            Self::UnixAbstractConnect => "unix-abstract-connect",
            Self::Fd => "fd",
            Self::File => "file",
        }
    }
}

#[cfg(feature = "graphql")]
graphql_object!(SBPlatform: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    field working_directory() -> &str {
        self.working_directory()
    }

    field name() -> &str {
        self.name()
    }

    field triple() -> &str {
        self.triple()
    }

    field hostname() -> &str {
        self.hostname()
    }

    field os_build() -> &str {
        self.os_build()
    }

    field os_description() -> &str {
        self.os_description()
    }

    // TODO(bm) This should be u32
    field os_major_version() -> i32 {
        self.os_major_version() as i32
    }

    // TODO(bm) This should be u32
    field os_minor_version() -> i32 {
        self.os_minor_version() as i32
    }

    // TODO(bm) This should be u32
    field os_update_version() -> i32 {
        self.os_update_version() as i32
    }
});
