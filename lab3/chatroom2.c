#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <pthread.h>
#include <fcntl.h>
#define port 5028
int mutex = 1;

//record the valid socket
int status[32] = {0};
int person = 0;
int wating[32] = {0};
int fd;

//thread for each thread
pthread_t threads[32] = {0};

//fd socket 
void *handle_chat(void *fd) {
    char buffer[8] = "Messege:";
    char RecvBuffer[1024] = "";
    ssize_t len;

    while ((len = recv(*(int*)fd, RecvBuffer, 1024, 0)) > 0) {
        while(mutex == 0);
        mutex = 0;
        for(int j = 0; j < 32; j++){
            if(status[j] != 0 && status[j] != *(int*) fd){
                int last = 0;
                for(int i = 0; i < len; i++){
                    if(RecvBuffer[i] == '\n'){
                        send(status[j],buffer,8,0);
                        send(status[j],RecvBuffer+last,i-last+1,0);
                        last = i + 1;
                    }
                }
            }
        }
        mutex = 1;
    }
    return NULL;
}

void *send_prompt(void * fd){
    char buffer[40] = "There are    people in this chartroom\n";
    buffer[10] = person/10 + '0';
    buffer[11] = person%10 + '0';

    for(int j = 0; j < 32; j++){
        if(status[j] != 0 && status[j]!= *(int*) fd){
            send(status[j],buffer,40,0);
        }
        else if(status[j] !=0){
            char buf[30] = "Welcome to Sora Chatroom\n";
            send(status[j],buf,30,0);
        }
    }

}

void* create_new_user(void *i){

    int fd_create;
    int pos;
    //exit(0);
    pos = *(int*)i;

    fd_create = accept(fd,NULL,NULL);
    
    person++;
    //exit(0);

    status[pos] = fd_create;
    wating[pos] = 0;
    pthread_t thread1,thread2;
    pthread_create(&thread1,NULL,send_prompt,(void*)&fd_create);
    pthread_detach(thread1);
    pthread_create(&thread2,NULL,handle_chat,(void*)&fd_create);
    
    pthread_join(thread2,NULL);
    person--;
    status[pos] = 0;
    //exit(0);

    return NULL;
}

void* manage_user(void *fd){

    while(1){
        for(int i = 0; i < 32; i++){
            //exit(0);
            if(status[i] == 0 && wating[i] == 0){
                //exit(0);
                pthread_create(&threads[i],NULL,create_new_user,(void*)&i);
                wating[i] = 1;
                pthread_detach(threads[i]);
                //exit(0);
            }
        }
    }


}

int main(int argc, char **argv) {

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

    //waiting for connection
    if (listen(fd, 32)) {
        perror("listen");
        return 1;
    }

    pthread_t thread1;

    //管理线程
    pthread_create(&thread1,NULL,manage_user,(void*)&fd);
    pthread_join(thread1,NULL);

    return 0;
}