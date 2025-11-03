#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>      // shm_open
#include <sys/mman.h>   // mmap
#include <sys/stat.h>   // mode constants
#include <unistd.h>     // ftruncate, close

struct MyData {
    int counter;
    char msg[64];
};

int main() {
    const char *name = "/my_shared_mem";
    int fd = shm_open(name, O_CREAT | O_RDWR, 0666);
    if (fd < 0) {
        perror("shm_open failed");
        return 1;
    }

    ftruncate(fd, sizeof(struct MyData));

    struct MyData *data = mmap(NULL, sizeof(struct MyData),
                               PROT_READ | PROT_WRITE,
                               MAP_SHARED, fd, 0);
    if (data == MAP_FAILED) {
        perror("mmap failed");
        return 1;
    }

    close(fd); // not needed anymore

    data->counter = 123;
    strcpy(data->msg, "Hello from writer!");

    printf("Writer: wrote counter=%d, msg='%s'\n", data->counter, data->msg);
    printf("Now waiting 15s so reader can read...\n");
    sleep(15);

    munmap(data, sizeof(struct MyData));
    shm_unlink(name); // remove shared mem when done
    return 0;
}
