/* Test-only per-process descriptor scarcity between installation chmod and readback. */
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/param.h>
#include <sys/resource.h>
#include <sys/stat.h>
#include <unistd.h>

static int trace_file = -1;
static int armed;
static int fillers[128];
static size_t filled;
static struct rlimit previous;

static void event(const char *format, ...) {
    int saved = errno;
    char line[256];
    va_list arguments;
    va_start(arguments, format);
    int length = vsnprintf(line, sizeof(line), format, arguments);
    va_end(arguments);
    if (length <= 0 || (size_t)length >= sizeof(line)) _exit(90);
    size_t offset = 0;
    while (offset < (size_t)length) {
        ssize_t count = write(trace_file, line + offset, (size_t)length - offset);
        if (count < 0 && errno == EINTR) continue;
        if (count <= 0) _exit(91);
        offset += (size_t)count;
    }
    errno = saved;
}

__attribute__((constructor)) static void loaded(void) {
    const char *path = getenv("ARCHOGEN_TEST_CLOSE_TRACE");
    if (path == NULL) _exit(92);
    trace_file = open(path, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
    if (trace_file < 0) _exit(93);
    event("loaded\n");
    if (unsetenv("DYLD_INSERT_LIBRARIES") != 0 || unsetenv("LD_PRELOAD") != 0) _exit(94);
}

static const char *suffix(const char *path) {
    const char *root = getenv("ARCHOGEN_TEST_CLOSE_ROOT");
    if (root == NULL) _exit(95);
    size_t length = strlen(root);
    return strncmp(path, root, length) == 0 ? path + length : NULL;
}

static int selected(const char *path) {
    const char *tail = suffix(path);
    const char prefix[] = "/.archogen-provenance/";
    return tail != NULL && strncmp(tail, prefix, sizeof(prefix) - 1) == 0
        && strcmp(strrchr(tail, '/'), "/observer") == 0;
}

static void release_fillers(void) {
    if (!armed) { event("release-pass\n"); return; }
    for (size_t index = 0; index < filled; ++index) {
        if (close(fillers[index]) != 0) _exit(96);
    }
    for (size_t index = 0; index < filled; ++index) {
        if (fcntl(fillers[index], F_GETFD) != -1 || errno != EBADF) _exit(97);
    }
    if (setrlimit(RLIMIT_NOFILE, &previous) != 0) _exit(98);
    struct rlimit restored;
    if (getrlimit(RLIMIT_NOFILE, &restored) != 0
        || restored.rlim_cur != previous.rlim_cur
        || restored.rlim_max != previous.rlim_max) _exit(99);
    event("released closed=%zu soft=%llu hard=%llu\n", filled,
        (unsigned long long)restored.rlim_cur, (unsigned long long)restored.rlim_max);
    filled = 0;
}

static void fill_descriptors(const char *path) {
    if (getrlimit(RLIMIT_NOFILE, &previous) != 0 || previous.rlim_cur < 16) _exit(100);
    struct rlimit limited = previous;
    if (limited.rlim_cur > 64) limited.rlim_cur = 64;
    int writer = -1;
    for (int file = 0; file < (int)limited.rlim_cur; ++file) {
        char current[MAXPATHLEN];
        if (fcntl(file, F_GETPATH, current) == 0 && strcmp(current, path) == 0) {
            if (writer != -1) _exit(101);
            writer = file;
        }
    }
    if (writer < 0 || setrlimit(RLIMIT_NOFILE, &limited) != 0) _exit(102);
    int file;
    for (;;) {
        file = open("/dev/null", O_RDONLY | O_CLOEXEC);
        if (file < 0) {
            if (errno == EINTR) continue;
            break;
        }
        if (filled == sizeof(fillers) / sizeof(fillers[0])) _exit(103);
        fillers[filled++] = file;
    }
    if (errno != EMFILE) _exit(104);
    unsigned int count = 0;
    for (int descriptor = 0; descriptor < (int)limited.rlim_cur; ++descriptor) {
        if (fcntl(descriptor, F_GETFD) >= 0) ++count;
        else if (errno != EBADF) _exit(105);
    }
    if (count != limited.rlim_cur) _exit(106);
    armed = 1;
    event("filled writer=%d fillers=%zu open=%u soft=%llu prior=%llu hard=%llu errno=%d\n",
        writer, filled, count, (unsigned long long)limited.rlim_cur,
        (unsigned long long)previous.rlim_cur, (unsigned long long)previous.rlim_max, EMFILE);
}

static int limited_chmod(const char *path, mode_t mode) {
    const char *tail = suffix(path);
    if (tail != NULL && strcmp(tail, "/release") == 0) release_fillers();
    int status = chmod(path, mode);
    if (status != 0 || !selected(path)) return status;
    event("permissions\n");
    const char *scarce = getenv("ARCHOGEN_TEST_CLOSE_SCARCE");
    if (!armed && scarce != NULL && strcmp(scarce, "1") == 0) fill_descriptors(path);
    return status;
}

static ssize_t traced_read(int file, void *buffer, size_t size) {
    int saved = errno;
    char path[MAXPATHLEN];
    if (fcntl(file, F_GETPATH, path) == 0 && selected(path)) {
        struct stat metadata;
        if (fstat(file, &metadata) != 0) _exit(107);
        event("readback fd=%d inode=%llu\n", file, (unsigned long long)metadata.st_ino);
    }
    errno = saved;
    return read(file, buffer, size);
}

__attribute__((used, section("__DATA,__interpose,interposing")))
static const struct { const void *function; const void *symbol; } hooks[] = {
    { (const void *)limited_chmod, (const void *)chmod },
    { (const void *)traced_read, (const void *)read }
};
