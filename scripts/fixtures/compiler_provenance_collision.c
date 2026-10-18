#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/* Create independent evidence at the observer's destination before it publishes. */
int main(int argc, char **argv) {
    if (argc != 2) return 90;
    size_t size = strlen(argv[1]) + 64;
    char *path = malloc(size);
    if (!path) return 91;
    int count = snprintf(path, size, "%s/%ld.json", argv[1], (long)getppid());
    if (count < 0 || (size_t)count >= size) { free(path); return 92; }
    FILE *out = fopen(path, "wx");
    free(path);
    if (!out) return 93;
    if (fputs("retained native evidence\n", out) == EOF) { fclose(out); return 94; }
    if (fclose(out)) return 95;
    puts("native compiler completed");
    return 0;
}
