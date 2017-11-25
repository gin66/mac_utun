#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <netinet/in.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/kern_control.h>
#include <net/if_utun.h>
#include <sys/ioctl.h>
#include <sys/kern_event.h>

int32_t open_utun(uint64_t num) {
    int err;
    int fd;
    struct sockaddr_ctl addr;
    struct ctl_info info;

    fd = socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL);
    if (fd < 0) {
        return fd;
    }
    memset(&info, 0, sizeof (info));
    strncpy(info.ctl_name, UTUN_CONTROL_NAME, strlen(UTUN_CONTROL_NAME));
    err = ioctl(fd, CTLIOCGINFO, &info);
    if (err < 0) {
        close(fd);
        return err;
    }

    addr.sc_id = info.ctl_id;
    addr.sc_len = sizeof(addr);
    addr.sc_family = AF_SYSTEM;
    addr.ss_sysaddr = AF_SYS_CONTROL;
    addr.sc_unit = num + 1; // utunX where X is sc.sc_unit -1

    err = connect(fd, (struct sockaddr*)&addr, sizeof(addr));
    if (err < 0) {
        // this utun is in use
        close(fd);
        return err;
    }
    return fd;
}

void close_utun(int32_t fd) {
    close(fd);
}
