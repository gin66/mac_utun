use std::io::{Error, ErrorKind, Result};
use std::os::unix::io::FromRawFd;
use std::net::UdpSocket;

#[cfg(target_os = "macos")]
extern "C" {
    fn open_utun(num :u64) -> i32;
}

#[cfg(target_os = "macos")]
pub fn get_utun() -> Result<(UdpSocket,String)> {
    for utun_n in 0..255 {
        let fd = unsafe { open_utun(utun_n) as i32 };
        if fd >= 0 {
            let name = format!("utun{}",1);
            let sock = unsafe { UdpSocket::from_raw_fd(fd) };
            return Ok((sock, name));
        }
    }
    Err(std::io::Error::last_os_error())
}

#[cfg(not(target_os = "macos"))]
pub fn get_utun() -> Result<(UdpSocket,String)> {
    Err(ErrorKind::NotFound as Error)
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    fn get_interfaces() -> String {
        let output = Command::new("ifconfig")
                    .args(&["-l"])
                    .output()
                    .expect("failed to execute ifconfig");
        String::from_utf8_lossy(&output.stdout).into_owned()
    }
    
    #[cfg(target_os = "macos")]
    #[test]
    fn it_works() {
        let iflist_before = get_interfaces();
        let res = ::get_utun();
        assert!(res.is_ok());
        let (_utun_sock,if_name) = res.unwrap();
        let iflist_after = get_interfaces();
        assert!(!iflist_before.contains(&if_name));
        assert!( iflist_after.contains(&if_name));
    }
}
