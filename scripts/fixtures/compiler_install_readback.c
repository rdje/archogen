/* Test-only installation corruption after successful chmod, before controller readback. */
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/param.h>
#include <sys/stat.h>
#include <unistd.h>

static int changed;
static void event(const char *kind, ino_t inode) {
    int saved = errno;
    const char *trace = getenv("ARCHOGEN_TEST_INSTALL_TRACE");
    if (trace == NULL) _exit(90);
    char line[128];
    int size = snprintf(line, sizeof(line), "%s inode=%llu\n", kind, (unsigned long long)inode);
    if (size <= 0 || (size_t)size >= sizeof(line)) _exit(91);
    int output = open(trace, O_WRONLY | O_CREAT | O_APPEND, 0600);
    if (output < 0) _exit(92);
    size_t offset = 0;
    while (offset < (size_t)size) {
        ssize_t count = write(output, line + offset, (size_t)size - offset);
        if (count < 0 && errno == EINTR) continue;
        if (count <= 0) _exit(93);
        offset += (size_t)count;
    }
    if (close(output) != 0) _exit(94);
    errno = saved;
}

__attribute__((constructor)) static void loaded(void) {
    event("loaded", 0);
    if (unsetenv("DYLD_INSERT_LIBRARIES") != 0 || unsetenv("LD_PRELOAD") != 0) _exit(95);
}

static int selected(const char *path) {
    const char *root = getenv("ARCHOGEN_TEST_INSTALL_ROOT");
    if (root == NULL) _exit(96);
    size_t length = strlen(root);
    const char suffix[] = "/.archogen-provenance/";
    return strncmp(path, root, length) == 0
        && strncmp(path + length, suffix, sizeof(suffix) - 1) == 0
        && strcmp(strrchr(path, '/'), "/observer") == 0;
}

static int fault_chmod(const char *path, mode_t mode) {
    int status = chmod(path, mode);
    if (status != 0 || !selected(path)) return status;
    struct stat metadata;
    if (stat(path, &metadata) != 0) _exit(97);
    event("permissions", metadata.st_ino);
    const char *inject = getenv("ARCHOGEN_TEST_INSTALL_CORRUPT");
    if (!changed && inject != NULL && strcmp(inject, "1") == 0) {
        int file = open(path, O_RDWR);
        unsigned char byte;
        if (file < 0 || pread(file, &byte, 1, 0) != 1) _exit(98);
        byte ^= 1;
        if (pwrite(file, &byte, 1, 0) != 1 || close(file) != 0) _exit(99);
        changed = 1;
        event("corrupt", metadata.st_ino);
    }
    return status;
}

static ssize_t trace_read(int file, void *buffer, size_t size) {
    int saved = errno;
    char path[MAXPATHLEN];
    if (fcntl(file, F_GETPATH, path) == 0 && selected(path)) {
        struct stat metadata;
        if (fstat(file, &metadata) != 0) _exit(100);
        event("readback", metadata.st_ino);
    }
    errno = saved;
    return read(file, buffer, size);
}

__attribute__((used, section("__DATA,__interpose,interposing")))
static const struct { const void *function; const void *symbol; } hooks[] = {
    { (const void *)fault_chmod, (const void *)chmod },
    { (const void *)trace_read, (const void *)read }
};
