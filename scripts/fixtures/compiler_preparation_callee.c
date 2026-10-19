/* Native callee for the controller's process/output/environment boundary. */
#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

extern char **environ;

static void failed(const char *operation) {
    perror(operation);
    exit(90);
}

static char *copy(const char *text) {
    char *result = strdup(text);
    if (result == NULL) failed("strdup");
    return result;
}

static void parent(char *path) {
    char *slash = strrchr(path, '/');
    if (slash == NULL || slash == path) {
        fputs("invalid controlled output path\n", stderr);
        exit(91);
    }
    *slash = '\0';
}

static char *joined(const char *directory, const char *name) {
    size_t a = strlen(directory), b = strlen(name);
    if (a > (size_t)-1 - b - 2) exit(92);
    char *result = malloc(a + b + 2);
    if (result == NULL) failed("malloc");
    memcpy(result, directory, a);
    result[a] = '/';
    memcpy(result + a + 1, name, b + 1);
    return result;
}

static void write_all(int output, const char *bytes, size_t length) {
    while (length != 0) {
        ssize_t count = write(output, bytes, length);
        if (count < 0 && errno == EINTR) continue;
        if (count <= 0) failed("write");
        bytes += count;
        length -= (size_t)count;
    }
}

int main(int argc, char **argv) {
    if (argc != 7 || strcmp(argv[1], "--edition=2021") != 0 ||
        strcmp(argv[2], "--crate-name") != 0 ||
        strcmp(argv[3], "archogen_compiler_observer") != 0 ||
        strcmp(argv[5], "-o") != 0) {
        fputs("unexpected controlled compiler argv\n", stderr);
        return 93;
    }
    struct stat source;
    if (stat(argv[4], &source) != 0) failed("observer source");
    if (!S_ISREG(source.st_mode) || source.st_size == 0) {
        fputs("observer source is not a nonempty regular file\n", stderr);
        return 96;
    }
    char *directory = copy(argv[6]);
    parent(directory);
    char *receipt = joined(directory, "environment.bin");
    int output = open(receipt, O_WRONLY | O_CREAT | O_EXCL, 0600);
    if (output < 0) failed("environment receipt");
    for (char **entry = environ; *entry != NULL; ++entry)
        write_all(output, *entry, strlen(*entry) + 1);
    if (close(output) != 0) failed("close receipt");
    free(receipt);
    char *case_directory = copy(directory);
    parent(case_directory);
    parent(case_directory);
    const char *mode = strrchr(case_directory, '/') + 1;
    if (strcmp(mode, "failed") != 0 && strcmp(mode, "empty") != 0 && strcmp(mode, "success") != 0)
    {
        fputs("unexpected controlled compiler case\n", stderr);
        return 94;
    }
    FILE *binary = fopen(argv[6], "wb");
    if (binary == NULL) failed("controlled output");
    if (strcmp(mode, "empty") != 0) {
        const char *path = getenv("PATH");
        if (path == NULL) return 95;
        char *first = copy(path);
        char *separator = strchr(first, ':');
        if (separator != NULL) *separator = '\0';
        char *seed = joined(first, "seed-observer");
        FILE *input = fopen(seed, "rb");
        if (input == NULL) failed("seed observer");
        char buffer[8192];
        size_t count;
        while ((count = fread(buffer, 1, sizeof(buffer), input)) != 0)
            if (fwrite(buffer, 1, count, binary) != count) failed("copy observer");
        if (ferror(input) || fclose(input) != 0) failed("read observer");
        free(seed);
        free(first);
    }
    if (fclose(binary) != 0 || chmod(argv[6], 0755) != 0) failed("finish observer");
    int status = strcmp(mode, "failed") == 0 ? 7 : 0;
    if (status != 0) fputs("controlled compiler failure\n", stderr);
    free(case_directory);
    free(directory);
    return status;
}
