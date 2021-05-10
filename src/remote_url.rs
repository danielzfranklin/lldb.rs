use std::{ffi::CString, fmt::Write};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RemoteUrl {
    scheme: RemoteUrlScheme,
    host: String,
    port: Option<u16>,
    path: Option<String>,
}

impl RemoteUrl {
    pub fn new(scheme: RemoteUrlScheme, host: impl Into<String>) -> Self {
        Self {
            scheme,
            host: host.into(),
            port: None,
            path: None,
        }
    }

    pub fn path(&mut self, path: impl Into<String>) -> &mut Self {
        self.path = Some(path.into());
        self
    }

    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }

    pub(crate) fn serialize(&self) -> CString {
        // For details of URL format supported see <https://github.com/llvm/llvm-project/blob/d480f968ad8b56d3ee4a6b6df5532d485b0ad01e/lldb/source/Utility/UriParser.cpp>
        let mut url = format!("{}://{}", self.scheme.as_str(), self.host);
        if let Some(port) = self.port {
            write!(&mut url, ":{}", port).unwrap();
        }
        if let Some(path) = self.path.as_ref() {
            write!(&mut url, "/{}", path).unwrap();
        }

        CString::new(url).expect("URL doesn't contain nul")
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RemoteUrlScheme {
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

impl RemoteUrlScheme {
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
