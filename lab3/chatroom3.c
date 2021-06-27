#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/socket.h>
#include <sys/select.h>
#include <sys/time.h>
#include <netinet/in.h>
#include <errno.h>
#define MAX 4

struct Pipe {
    int fd_send;
    int fd_recv;
};

int status[1024] = {0};
int maxfd = 0;
int person = 0;

int main(int argc, char **argv) {
    int port = atoi(argv[1]);
    int server_fd;
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("socket");
        return 1;
    }
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons(port);
    socklen_t addr_len = sizeof(addr);
    if (bind(server_fd, (struct sockaddr *)&addr, sizeof(addr))) {
        perror("bind");
        return 1;
    }
    if (listen(server_fd, MAX)) {
        perror("listen");
        return 1;
    }
    

    maxfd = server_fd;
    struct timeval timv;
    timv.tv_sec = 0;
    timv.tv_usec = 1;

    
    
    char buffer[1024] = "Message:";
    fcntl(server_fd, F_SETFL, fcntl(server_fd, F_GETFL, 0) | O_NONBLOCK);
    fd_set clients;
    fd_set acceptor;
    fd_set rfd;
    while (1) {
        int client_fd;
        FD_ZERO(&rfd);
        FD_SET(client_fd,&rfd);
        FD_ZERO(&acceptor);
        FD_SET(server_fd,&acceptor);
        select(maxfd+1,&rfd,NULL,NULL,&timv);
        //exit(0);
        if(FD_ISSET(server_fd,&acceptor)){
            //exit(0);
            client_fd = accept(server_fd,NULL,NULL);
            if(client_fd < 0){
                if(errno == EWOULDBLOCK){
                    //未有新连接

                    // exit(0);
                    FD_ZERO(&clients);
                    //创建集合
                    for(int i=0;i<1024;i++){
                        if(status[i]!=0){
                            FD_SET(status[i],&clients);
                        }
                    }
                    if(select(maxfd+1,&clients,NULL,NULL,&timv) > 0){
                        for(int i=0;i<1024;i++){
                            if(FD_ISSET(status[i],&clients)){
                                ssize_t len = recv(status[i],buffer+8,1000,0);
                                //exit(0);
                                if(len == 0){
                                    status[i] = 0;
                                    person--;
                                }
                                else{
                                    int pos=0;
                                    for(int x=0;x<len;x++){
                                        if(buffer[x+8]!='\n')
                                            continue;
                                        for(int j=0;j<1024;j++){
                                            if(status[j]!=0 && status[j]!=status[i]){
                                                send(status[j],buffer,8,0);
                                                send(status[j],buffer+8+pos,x-pos+1,0);
                                            }
                                        }
                                        pos = x;
                                    }
                                }

                            }
                        }
                    }
                }
            }
            else{
                //exit(0);
                //有新连接
                
                if(person == MAX){
                    printf("FULL!");
                    continue;
                }
                person++;

                for(int i=0;i<1024;i++){
                    if(status[i] == 0){
                        status[i] = client_fd;
                        break;
                    }
                }
                fcntl(client_fd, F_SETFL, fcntl(client_fd, F_GETFL, 0) | O_NONBLOCK);
                if(maxfd < client_fd)
                    maxfd = client_fd;
            }
        }
        //exit(0);


    }
    return 0;
}
