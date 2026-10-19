/* Test-only macOS trace/fault boundary for the explicit-close proof. */
#ifndef __APPLE__
#error This trace fixture requires the public Mach-O interposition interface
#endif
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static int record_fd = -1;

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
    record_fd = descriptor;
    event("full-sync", descriptor);
    return fcntl(descriptor, operation);
}

static int fault_close(int descriptor) {
    if (descriptor != record_fd) return close(descriptor);
    record_fd = -1;
    int result = close(descriptor);
    event(result == 0 ? "closed" : "close-error", descriptor);
    if (getenv("ARCHOGEN_TEST_CLOSE_ERROR") != NULL) { errno = EIO; return -1; }
    return result;
}
static int fault_linkat(int olddir, const char *oldpath, int newdir, const char *newpath, int flags) {
    event("link", record_fd);
    if (getenv("ARCHOGEN_TEST_LINK_ERROR") != NULL) { errno = EIO; return -1; }
    return linkat(olddir, oldpath, newdir, newpath, flags);
}
static int fault_unlink(const char *path) {
    event("unlink", record_fd);
    if (getenv("ARCHOGEN_TEST_UNLINK_ERROR") != NULL) { errno = EIO; return -1; }
    return unlink(path);
}

/* Two-address records are the public Mach-O interposition interface. */
__attribute__((used, section("__DATA,__interpose,interposing")))
static const struct { const void *function; const void *symbol; } hooks[] = {
    { (const void *)fault_fcntl, (const void *)fcntl },
    { (const void *)fault_close, (const void *)close },
    { (const void *)fault_linkat, (const void *)linkat },
    { (const void *)fault_unlink, (const void *)unlink }
};
