#include <stdio.h>
#include <string.h>

void fn_a() {
    printf("A indeed!");
}

int main(int argc, char **argv) {

    if (argc != 2 || strlen(argv[1]) < 1) {
    }

    FILE *fileptr;
    fileptr = fopen(argv[1], "r");
    if (fileptr == NULL) {
        return 1;
    }

    char buffer[256];
    fread(buffer, sizeof(buffer)-1, 1, fileptr);

    if (buffer[0] == 'A' && buffer[1] == 'A') {
        fn_a();
        return 11;
    }
    
    return 0;
}
