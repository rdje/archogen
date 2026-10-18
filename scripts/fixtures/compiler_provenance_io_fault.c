/* Test-only native synchronization fault at the observer/libc boundary. */
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static void event(const char *kind, int descriptor) {
    int saved = errno;
    const char *path = getenv("ARCHOGEN_TEST_IO_TRACE");
    if (path == NULL) _exit(90);
    char bytes[96];
    int length = snprintf(bytes, sizeof(bytes), "%s pid=%ld fd=%d\n", kind, (long)getpid(), descriptor);
    if (length < 0 || (size_t)length >= sizeof(bytes)) _exit(91);
    int output = open(path, O_WRONLY | O_CREAT | O_APPEND, 0600);
    if (output < 0) _exit(92);
    size_t offset = 0;
    while (offset < (size_t)length) {
        ssize_t count = write(output, bytes + offset, (size_t)length - offset);
        if (count < 0 && errno == EINTR) continue;
        if (count <= 0) _exit(93);
        offset += (size_t)count;
    }
    if (close(output) != 0) _exit(94);
    errno = saved;
}

__attribute__((constructor)) static void loaded(void) {
    event("loaded", -1);
    /* The compiler child must run without this test library. */
    if (unsetenv("DYLD_INSERT_LIBRARIES") != 0 || unsetenv("LD_PRELOAD") != 0) _exit(97);
}

static int should_fail(void) {
    const char *mode = getenv("ARCHOGEN_TEST_IO_ERROR");
    return mode != NULL && strcmp(mode, "1") == 0;
}

static int fault_sync(int descriptor) {
    event("sync", descriptor);
    if (should_fail()) {
        errno = EIO;
        return -1;
    }
#ifdef __APPLE__
    /* Mach-O keeps direct references in this interposing image bound to originals. */
    return fsync(descriptor);
#else
    int (*actual)(int) = (int (*)(int))dlsym(RTLD_NEXT, "fsync");
    if (actual == NULL) _exit(95);
    return actual(descriptor);
#endif
}

#ifdef __APPLE__
static int fault_fcntl(int descriptor, int operation, ...) {
    /* Forward only documented argument shapes; an unexpected operation fails the fixture. */
    switch (operation) {
        case F_GETFD:
        case F_GETFL:
        case F_GETOWN:
            return fcntl(descriptor, operation);
        case F_SETFD:
        case F_SETFL:
        case F_SETOWN:
        case F_DUPFD:
#ifdef F_DUPFD_CLOEXEC
        case F_DUPFD_CLOEXEC:
#endif
        {
            va_list arguments;
            va_start(arguments, operation);
            int value = va_arg(arguments, int);
            va_end(arguments);
            return fcntl(descriptor, operation, value);
        }
        case F_GETPATH: {
            va_list arguments;
            va_start(arguments, operation);
            void *buffer = va_arg(arguments, void *);
            va_end(arguments);
            return fcntl(descriptor, operation, buffer);
        }
    }
    if (operation != F_FULLFSYNC) {
        event("unexpected-fcntl", operation);
        _exit(96);
    }
    event("full-sync", descriptor);
    if (should_fail()) {
        errno = EIO;
        return -1;
    }
    return fcntl(descriptor, operation);
}

/* Two-address records are the public Mach-O interposition interface. */
__attribute__((used, section("__DATA,__interpose,interposing")))
static const struct { const void *function; const void *symbol; } hooks[] = {
    { (const void *)fault_sync, (const void *)fsync },
    { (const void *)fault_fcntl, (const void *)fcntl }
};
#else
int fsync(int descriptor) { return fault_sync(descriptor); }
#endif
