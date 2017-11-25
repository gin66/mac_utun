use std::io::Result;
use std::os::unix::io::{AsRawFd, RawFd};

extern "C" {
    fn open_utun(num :u64) -> i32;
    fn close_utun(sock :i32);
}

#[derive(Debug)]
pub struct UtunSocket {
    sock: RawFd,
    name: String
}

impl UtunSocket {
    pub fn new() -> Result<UtunSocket> {
        let sock = unsafe { open_utun(1) as i32};
        if sock < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let name = format!("utun{}",1);
        Ok(UtunSocket { sock, name })
    }

    pub fn if_name(&self) -> &String {
        &self.name
    }
}


impl AsRawFd for UtunSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.sock
    }
}

impl Drop for UtunSocket {
    fn drop(&mut self) {
        unsafe { close_utun(self.sock) }
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
        let sock = res.unwrap();
        let if_name = sock.if_name();
        let iflist_after = get_interfaces();
        assert!(!iflist_before.contains(if_name));
        assert!( iflist_after.contains(if_name));
    }
}
