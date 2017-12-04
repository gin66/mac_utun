mac_utun
========

# Create utun device on macos.

## Import

At the top of the file:
``` Rust
extern crate mac_utun;

use mac_utun::get_utun;
```

## Usage

Just need to call this function. It will search for the first available utun-device
starting from 0. As you may know, root permission is necessary to create a utun-device.

``` Rust
pub fn get_utun() -> Result<(UdpSocket,String)>
```

In case of error, the last OS-Error will be returned.
In case of success, an UDP-socket and the interface name (e.g. "utun0") will be returned.

UDP-socket is a nice choice, because rust will perform the necessary clean up itself.

## Test

There is only one test case defined:

It checks the list of network interfaces before, during and after utun is opened.
Expected behaviour: The returned utun is only in the list _during_ utun is opened.

