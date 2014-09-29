#include <fcntl.h>
#include <stdlib.h>
#include <sys/select.h>

/* XXX XXX XXX - major hack - most of this C stuff should go away anyway */
static int tty_fd = -1;

static int get_tty_fd(void)
{
    if (tty_fd < 0)
        tty_fd = open("/dev/tty", O_RDONLY);

    return tty_fd;
}
/* end hack */

int timed_read(int timeout)
{
    char byte;

    if (timeout >= 0) {
        fd_set readfds;
        struct timeval t;

        FD_ZERO(&readfds);
        FD_SET(get_tty_fd(), &readfds);

        t.tv_sec  = timeout / 1000000;
        t.tv_usec = timeout % 1000000;

        if (select(1, &readfds, NULL, NULL, &t) == 1) {
            if (read(get_tty_fd(), &byte, 1) == 1) {
                return byte;
            }
            else {
                return -1;
            }
        }
        else {
            return -1;
        }
    }
    else {
        if (read(get_tty_fd(), &byte, 1) == 1) {
            return byte;
        }
        else {
            return -1;
        }
    }
}
