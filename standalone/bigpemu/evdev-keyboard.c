#include <errno.h>
#include <fcntl.h>
#include <linux/input.h>
#include <linux/uinput.h>
#include <poll.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define MAX_INPUTS 16

static volatile sig_atomic_t running = 1;

static void stop(int signal_number) {
    (void)signal_number;
    running = 0;
}

static void emit(int fd, unsigned short type, unsigned short code, int value) {
    struct input_event event = {.type = type, .code = code, .value = value};
    if (write(fd, &event, sizeof(event)) != sizeof(event))
        fprintf(stderr, "BigPEmu input: uinput write failed: %s\n", strerror(errno));
}

static int key_for(unsigned short code) {
    switch (code) {
    case 0x130: return KEY_D;       /* A -> Jaguar A */
    case 0x131: return KEY_S;       /* B -> Jaguar B */
    case 0x133: return KEY_A;       /* X -> Jaguar C */
    case 0x132: return KEY_W;       /* Y -> Option */
    case 0x137: return KEY_Q;       /* Start -> Pause */
    case 0x138: return KEY_ESC;     /* Menu -> BigPEmu menu */
    case 0x13b: return KEY_F2;      /* R2 -> fast-forward */
    default: return -1;
    }
}

static void emit_hat(int output, unsigned short axis, int value, int *previous) {
    int negative = axis == ABS_HAT0X ? KEY_LEFT : KEY_UP;
    int positive = axis == ABS_HAT0X ? KEY_RIGHT : KEY_DOWN;
    if (*previous < 0) emit(output, EV_KEY, negative, 0);
    if (*previous > 0) emit(output, EV_KEY, positive, 0);
    if (value < 0) emit(output, EV_KEY, negative, 1);
    if (value > 0) emit(output, EV_KEY, positive, 1);
    *previous = value;
    emit(output, EV_SYN, SYN_REPORT, 0);
}

int main(void) {
    struct pollfd inputs[MAX_INPUTS];
    int input_count = 0;
    for (int i = 0; i < MAX_INPUTS; i++) {
        char path[32];
        snprintf(path, sizeof(path), "/dev/input/event%d", i);
        int fd = open(path, O_RDONLY | O_NONBLOCK);
        if (fd >= 0) inputs[input_count++] = (struct pollfd){.fd = fd, .events = POLLIN};
    }
    int output = open("/dev/uinput", O_WRONLY | O_NONBLOCK);
    if (output < 0) {
        fprintf(stderr, "BigPEmu input: cannot open /dev/uinput: %s\n", strerror(errno));
        return 1;
    }
    ioctl(output, UI_SET_EVBIT, EV_KEY);
    const int keys[] = {KEY_A, KEY_S, KEY_D, KEY_W, KEY_Q, KEY_ESC, KEY_F2,
                        KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT};
    for (unsigned i = 0; i < sizeof(keys) / sizeof(keys[0]); i++)
        ioctl(output, UI_SET_KEYBIT, keys[i]);
    struct uinput_setup setup = {0};
    snprintf(setup.name, UINPUT_MAX_NAME_SIZE, "BaseOS BigPEmu Keyboard");
    setup.id.bustype = BUS_VIRTUAL;
    setup.id.vendor = 0x4241;
    setup.id.product = 0x5345;
    if (ioctl(output, UI_DEV_SETUP, &setup) < 0 || ioctl(output, UI_DEV_CREATE) < 0) {
        fprintf(stderr, "BigPEmu input: cannot create virtual keyboard: %s\n", strerror(errno));
        return 1;
    }
    signal(SIGINT, stop);
    signal(SIGTERM, stop);
    fprintf(stderr, "BigPEmu input: virtual keyboard ready, sources=%d\n", input_count);
    int hat_x = 0, hat_y = 0;
    while (running && poll(inputs, input_count, 250) >= 0) {
        for (int i = 0; i < input_count; i++) {
            if (!(inputs[i].revents & POLLIN)) continue;
            struct input_event event;
            while (read(inputs[i].fd, &event, sizeof(event)) == sizeof(event)) {
                if (event.type == EV_KEY) {
                    int key = key_for(event.code);
                    if (key >= 0) {
                        emit(output, EV_KEY, key, event.value);
                        emit(output, EV_SYN, SYN_REPORT, 0);
                    }
                } else if (event.type == EV_ABS && event.code == ABS_HAT0X) {
                    emit_hat(output, event.code, event.value, &hat_x);
                } else if (event.type == EV_ABS && event.code == ABS_HAT0Y) {
                    emit_hat(output, event.code, event.value, &hat_y);
                }
            }
        }
    }
    ioctl(output, UI_DEV_DESTROY);
    close(output);
    for (int i = 0; i < input_count; i++) close(inputs[i].fd);
    return 0;
}
