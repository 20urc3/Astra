#include <stdio.h>
#include <string.h>

void fn_a() {
    printf("A indeed!");
}

void fn_b() {
    printf("B indeed!");
}

void fn_c() {
    printf("C indeed!");
}

int main(int argc, char **argv) {

    // Check if we receive an input
    if (argc != 2 || strlen(argv[1]) < 1) {
        fprintf(stderr, "The program failed to receive input.\n");
    }

    // Read file received from user into a buffer
    FILE *fileptr;
    fileptr = fopen(argv[1], "r");
    if (fileptr == NULL) {
        fprintf(stderr, "The file is empty.\n");
        return 1;
    }

    char buffer[256];
    fread(buffer, sizeof(buffer)-1, 1, fileptr);

    // Check for value in the buffer
    // Depending on the value calls different functions
    // The goal is to have multiple branch to gather coverage
    if (buffer[0] == 'A') {
        fn_a();
    }

    if (buffer[0] == 'B') {
        fn_b();
    }

    if (buffer[0] == 'C') {
        fn_c();
    }
    
    return 0;
}

