#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

struct MyData {
    int counter;
    char msg[64];
};

int main() {
    const char *name = "/my_shared_mem";
    int fd = shm_open(name, O_RDWR, 0666);
    if (fd < 0) {
        perror("shm_open failed");
        return 1;
    }

    struct MyData *data = mmap(NULL, sizeof(struct MyData),
                               PROT_READ | PROT_WRITE,
                               MAP_SHARED, fd, 0);
    if (data == MAP_FAILED) {
        perror("mmap failed");
        return 1;
    }

    close(fd);

    printf("Reader: counter=%d, msg='%s'\n", data->counter, data->msg);

    data->counter += 1;
    strcpy(data->msg, "Reader changed the msg!");

    printf("Reader: updated counter=%d, msg='%s'\n", data->counter, data->msg);

    munmap(data, sizeof(struct MyData));
    return 0;
}
