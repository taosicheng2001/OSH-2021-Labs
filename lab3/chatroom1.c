#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <pthread.h>
#include <fcntl.h>
#define port 3048


struct Pipe {
    int fd_send;
    int fd_recv;
};

void *handle_chat(void *data) {
    struct Pipe *pipe = (struct Pipe *)data;
    char buffer[8] = "Messege:";
    char RecvBuffer[1024] = "";
    ssize_t len;

    while ((len = recv(pipe->fd_send, RecvBuffer, 1024, 0)) > 0) {
        int last = 0;
        for(int i = 0; i < len; i++){
            if(RecvBuffer[i] == '\n'){
                send(pipe->fd_recv,buffer,8,0);
                send(pipe->fd_recv,RecvBuffer+last,i-last+1,0);
                last = i + 1;
            }
        }
   
    }
    return NULL;
}

int main(int argc, char **argv) {
    int fd;
    if ((fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("socket");
        return 1;
    }
    
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons(atoi(argv[1]));
    socklen_t addr_len = sizeof(addr);
    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr))) {
        perror("bind");
        return 1;
    }
    if (listen(fd, 2)) {
        perror("listen");
        return 1;
    }
    int fd1 = accept(fd, NULL, NULL);
    int fd2 = accept(fd, NULL, NULL);
    if (fd1 == -1 || fd2 == -1) {
        perror("accept");
        return 1;
    }
    pthread_t thread1, thread2;
    struct Pipe pipe1;
    struct Pipe pipe2;
    pipe1.fd_send = fd1;
    pipe1.fd_recv = fd2;
    pipe2.fd_send = fd2;
    pipe2.fd_recv = fd1;

    // fcntl(fd1, F_SETFL, fcntl(fd1, F_GETFL, 0) | O_NONBLOCK); 
    // fcntl(fd2, F_SETFL, fcntl(fd2, F_GETFL, 0) | O_NONBLOCK);


    pthread_create(&thread1, NULL, handle_chat, (void *)&pipe1);
    pthread_create(&thread2, NULL, handle_chat, (void *)&pipe2);

    pthread_join(thread1, NULL);
    pthread_join(thread2, NULL);

    return 0;
}