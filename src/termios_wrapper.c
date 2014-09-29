#include <fcntl.h>
#include <stdlib.h>
#include <errno.h>
#include <termios.h>
#include <sys/ioctl.h>

/* very simplistic, ignores a lot of the settings that i don't understand,
 * patches welcome */

/* XXX XXX XXX - major hack - most of this C stuff should go away anyway */
extern int tty_fd;

static int get_tty_fd(void)
{
    if (tty_fd < 0)
        tty_fd = open("/dev/tty", O_RDONLY);

    return tty_fd;
}
/* end hack */

int cooked()
{
    struct termios t;

    if (tcgetattr(get_tty_fd(), &t) == -1) {
        return errno;
    }

    t.c_lflag |= (ICANON | ISIG | IEXTEN);
    t.c_iflag |= (IXON | BRKINT);

    return tcsetattr(get_tty_fd(), TCSANOW, &t) == 0 ? 0 : errno;
}

int cbreak()
{
    struct termios t;

    if (tcgetattr(get_tty_fd(), &t) == -1) {
        return errno;
    }

    t.c_lflag |= ISIG;
    t.c_lflag &= ~(ICANON | IEXTEN);
    t.c_iflag |= (IXON | BRKINT);

    return tcsetattr(get_tty_fd(), TCSANOW, &t) == 0 ? 0 : errno;
}

int raw()
{
    struct termios t;

    if (tcgetattr(get_tty_fd(), &t) == -1) {
        return errno;
    }

    t.c_lflag &= ~(ICANON | ISIG | IEXTEN);
    t.c_iflag &= ~(IXON | BRKINT);

    return tcsetattr(get_tty_fd(), TCSANOW, &t) == 0 ? 0 : errno;
}

int echo(int enabled)
{
    struct termios t;

    if (tcgetattr(get_tty_fd(), &t) == -1) {
        return errno;
    }

    if (enabled) {
        t.c_lflag |= ECHO;
    }
    else {
        t.c_lflag &= ~ECHO;
    }

    return tcsetattr(get_tty_fd(), TCSANOW, &t) == 0 ? 0 : errno;
}

struct termios *get()
{
    struct termios *t;

    t = malloc(sizeof(struct termios));
    if (tcgetattr(get_tty_fd(), t) == -1) {
        return NULL;
    }

    return t;
}

void set(struct termios *t)
{
    if (t == NULL) {
        return;
    }

    tcsetattr(get_tty_fd(), TCSANOW, t);
    free(t);
}

void size(unsigned int *cols, unsigned int *rows)
{
    struct winsize ws;
    ioctl(get_tty_fd(), TIOCGWINSZ, &ws);
    *cols = ws.ws_col;
    *rows = ws.ws_row;
}
