use std::io::Result;
use std::os::unix::io::{AsRawFd, RawFd, FromRawFd};
use std::net::UdpSocket;

#[cfg(target_os = "macos")]
extern "C" {
    fn open_utun(num :u64) -> i32;
}

#[derive(Debug)]
pub struct UtunSocket {
    sock: UdpSocket,
    name: String
}

impl UtunSocket {
    #[cfg(target_os = "macos")]
    pub fn new() -> Result<UtunSocket> {
        for utun_n in 0..255 {
            let fd = unsafe { open_utun(utun_n) as i32};
            if fd >= 0 {
                let name = format!("utun{}",1);
                let sock = unsafe { UdpSocket::from_raw_fd(fd) };
                return Ok(UtunSocket { sock, name });
            }
        }
        Err(std::io::Error::last_os_error())
    }

    #[cfg(not(target_os = "macos"))]
    pub fn new() -> Result<UtunSocket> {
        Err("Can open utun only on macos")
    }

    pub fn if_name(&self) -> &String {
        &self.name
    }
}


impl AsRawFd for UtunSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.sock.as_raw_fd()
    }
}

#[cfg(test)]
mod tests {
    use UtunSocket;
    use std::process::Command;

    fn get_interfaces() -> String {
        let output = Command::new("ifconfig")
                    .args(&["-l"])
                    .output()
                    .expect("failed to execute ifconfig");
        String::from_utf8_lossy(&output.stdout).into_owned()
    }
    
    #[test]
    fn it_works() {
        let iflist_before = get_interfaces();
        let res = UtunSocket::new();
        assert!(res.is_ok());
        let utun_sock = res.unwrap();
        let if_name = utun_sock.if_name();
        let iflist_after = get_interfaces();
        assert!(!iflist_before.contains(if_name));
        assert!( iflist_after.contains(if_name));
    }
}
