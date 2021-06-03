## 一、Shell已实现功能概述

1. **支持管道**

   - 单管道

     如 `ls | wc`等

   - 多重管道

     如`ls | cat -n | grep .txt`等

2. **支持基本文件重定向**

   - 标准输出重定向

     如`ls > out.txt` 或`cat read,txt > out.txt`

   - 标准输出追加重定向

     如`ls >> out.txt` 或`cat read,txt >> out.txt`

   - 标准输入重定向

     如`cat < read.txt`或`wc < read.txt`

3. **处理`Ctrl-C`按键**

   - 丢弃当前输入或进程，并打印提示符`#`

     如`ls > ^C`

4. **处理`Ctrl-D`按键**

   - 如`^D`则退出shell程序

5. **支持基于文件描述符的文件重定向、文件重定向组合(选做)**

   - 文件描述符重定向

     如:`ls 1> out.txt` 或 `cat 0< in.txt` 或 `cat 0< in.txt 1> out.txt`

   - `EOF`截取标准输入重定向

     如:

     ```
     grep t << EOF
     >this 
     >output
     >EOF
     ```

     或

     ```
     wc << EOF
     >How
     >many
     >words
     >EOF
     ```

   - 字符串标准输入重定向

     如` cat -n <<< HelloWorld!\n`或 `wc <<< I‘mKasugaNoSora\n`

6. **支持文件重定向与管道的组合**

   - 管道与文件重定向组合

     如` ls | cat -n | grep .swp < in.txt > out.txt`



## 二、编译运行方式与特性

1. **编译运行方式**

   在目录下使用`make`命令即可

2. **特性**

   - 每次进行外部命令调用后会打印显示本次命令执行情况，分为：

     1）Invalid Command

     2)  Outside Command Run Correctly

     3)  Outside Command Fail to Execute

   - 多重文件重定向时输入格式需要遵循标准输入格式

     在`>`与文件名称间不加空格可能导致指令执行出错

     

## 三、使用 strace 工具追踪系统调用

1. **`mmap()`**

   - 函数原型: `void* mmap(void* start,size_t length,int prot,int flags,int fd,off_t offset)`

   - 函数功能:

     1) 将一个普通文件映射进内存，用内存读写取代I/O读写

     2) 将特殊文件进行匿名映射，可以为关联进程提供共享内存空间

     3) 为无关联的进程提供共享内存空间

2. **`mprotect()`**

   - 函数原型: `int mprotect(const void *start, size_t len, int prot)`

   - 函数功能

     1) 把自`start`开始的、长度为`len`的内存区的保护属性修改为`prot`指定的值

     2)` PROT_READ`：表示内存段内的内容可写

     ​	`PROT_WRITE`：表示内存段内的内容可读

     ​	`PROT_EXEC`：表示内存段中的内容可执行

     ​	`PROT_NONE`：表示内存段中的内容根本没法访问

3. **`ioctl()`**

   - 函数原型: ` int ioctl(int fd, int cmd, ...) `

   - 函数功能:

     ​	驱动程序中设备控制接口函数，一个字符设备驱动通常会实现设备打开、关闭、读、写等功能,

     ​	通过增设ioctl()命令的方式可以扩展新的功能。

