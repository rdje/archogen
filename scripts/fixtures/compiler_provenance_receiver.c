#include <stdio.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
    FILE *out = fopen(getenv("ARGV_SINK"), "wb");
    if (!out) return 90;
    for (int i = 0; i < argc; ++i) {
        if (fwrite(argv[i], 1, strlen(argv[i]) + 1, out) != strlen(argv[i]) + 1) return 91;
    }
    if (fclose(out)) return 92;
    if (getenv("RUSTC_WRAPPER") || getenv("ARCHOGEN_PROTO_TRACE_DIR") ||
        getenv("ARCHOGEN_PROTO_ROLE") || getenv("ARCHOGEN_PROTO_SESSION")) return 93;
    if (getenv("ARGV_SIGNAL")) raise(SIGTERM);
    puts("native receiver stdout");
    fputs("native receiver stderr\n", stderr);
    return 7;
}
