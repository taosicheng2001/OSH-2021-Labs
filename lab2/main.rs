use std::io::{self,stdin, BufRead,Write};
use std::env;
use std::process::{exit, Command, Stdio};
use std::vec::Vec;
use std::fs::{File, OpenOptions};
use nix::sys::signal::{signal,Signal,SigHandler};
use std::char;
use std::os::unix::io::{FromRawFd};


extern fn handle_sigint(_signal: libc::c_int){
    let err = "Getting current dir failed";
    print!("\n{}#:", env::current_dir().expect(err).to_str().expect(err));
    io::stdout().flush().unwrap();
}

enum Fd {
    Right,
    RightAnd,
    DoubleRight,
    Left,
    DoubleLeft,
    TripleLeft,
    None,
}

fn fd_equal(a:&Fd,b:Fd)->bool {
    match a {
        Fd::Left => {
            match b {
                Fd::Left => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        Fd::DoubleLeft => {
            match b {
                Fd::DoubleLeft => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        Fd::TripleLeft => {
            match b {
                Fd::TripleLeft => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        Fd::Right => {
            match b {
                Fd::Right => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        Fd::DoubleRight => {
            match b {
                Fd::DoubleRight => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        Fd::RightAnd => {
            match b {
                Fd::RightAnd => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        _ => {
            match b {
                Fd::None => {
                    return true;
                }
                _ => {
                    return false;
                }
            }

        }
    }
}


enum Fdtype {
    Num2Num(i32,i32), 
    Num2File(i32,String),  
    File2Num(i32,String),
    Input2Std(String),
    AddFile(i32,String),
    EOF2Std,
}

enum FdValue {
    Num(i32),
    File(String),
}

struct FdImage {
    //输入输出对
    fd_in: FdValue,
    fd_out: FdValue,
}

fn main() -> ! {

    let mut fd_image = FdImage {
        fd_in: FdValue::Num(0),
        fd_out: FdValue::Num(1),
    };

    println!("-----Welcome-----");

    let handler = SigHandler::Handler(handle_sigint);
    unsafe{signal(Signal::SIGINT, handler)}.unwrap();

    loop {
        /*
        let console_pwd_err = "Get console pwd failed";
        println!("{}: ", env::current_dir().expect(console_pwd_err).to_str().expect(console_pwd_err));
        */

        let err = "Getting current dir failed";
        print!("{}$:", env::current_dir().expect(err).to_str().expect(err));
        io::stdout().flush().unwrap();

        let handler = SigHandler::Handler(handle_sigint);
        unsafe{signal(Signal::SIGINT, handler)}.unwrap();

        //let mut f = unsafe{File::from_raw_fd(1)};
        //f.write(b"how are you").unwrap();

        let mut cmds = String::new();
        for line_res in stdin().lock().read_line(&mut cmds) {
            match line_res {
                0 => {
                    exit(0);
                }
                _ => {
                    //println!("azhe");
                }
            }
        }
               
        //let mut pipe = Vec::new();

        //println!("{}",cmds);

        let (is_redirection,new_cmds,fd_all) = pre_redirection(&mut cmds);
        //is_redirection表示是否有重定向,new_cmds为去除重定向后的命令,fd_all为重定向文件向量
        
        //Fd映射表 默认标准输入 标准输出

        let mut append_flag = 0;

        if is_redirection == true {

            let cnt = fd_all.len();
            let mut i = 0;
            //println!("{}",cnt);
            //println!("{}",cnt);
            loop {

                if i >= cnt {
                    break;
                }

                i += 1;

                match &fd_all[i-1] {
                    Fdtype::Num2Num(num1,num2) => {
                        //println!("Num2NumType fd1:{}, fd2:{}",num1,num2);
                        match fd_image.fd_out {
                            FdValue::Num(num) => {
                                //更新映射
                                if num == *num1 {
                                    fd_image.fd_out = FdValue::Num(*num2);
                                }
                            }
                            _ => {
                                
                            }
                        }
                    }
                    Fdtype::Num2File(num1,filename) => {
                        //println!("Num2FileType fd1:{}, filename:{}",num1,filename);
                        match fd_image.fd_out {
                            FdValue::Num(num) => {
                                if num == *num1 {
                                    //update
                                    fd_image.fd_out = FdValue::File(filename.to_string());
                                }
                            }
                            _ => {
                                
                            }
                        }
                    }
                    Fdtype::File2Num(num1,filename) => {
                        //println!("File2NumType fd1:{}, filename:{}",num1,filename);
                        match fd_image.fd_in {
                            FdValue::Num(num) =>{
                                if num == *num1 {
                                    fd_image.fd_in = FdValue::File(filename.to_string());
                                }
                            }
                            _ => {

                            }
                        }
                    }
                    Fdtype::AddFile(num1,filename) => {
                        //println!("AddFileType fd1:{} filename:{}",num1,filename);
                        match fd_image.fd_out {
                            FdValue::Num(num) => {
                                if num == *num1 {
                                    append_flag = 1;
                                    //update
                                    fd_image.fd_out = FdValue::File(filename.to_string());
                                }
                            }
                            _ => {
                                
                            }
                        }                        
                    }
                    Fdtype::EOF2Std => {
                        //println!("EOF2Std");
                        //特殊处理
                        let filename = String::from(".swp");
                        let mut string_input = String::from("");
                        loop {
                            print!(">");
                            io::stdout().flush().unwrap();

                            let mut cmds_in = String::new();
                            stdin().lock().read_line(&mut cmds_in).expect("err");

                            if cmds_in == "EOF\n" {
                                break;
                            }
                            else {
                                string_input = string_input + &cmds_in;
                            }
                        }
                        std::fs::write(".swp",string_input).expect("err");
                        fd_image.fd_in = FdValue::File(filename);
                    }
                    Fdtype::Input2Std(inputstr) => {
                        //println!("Input2StdType input:{}",inputstr);
                        //特殊处理
                        let filename = String::from(".swp");
                        std::fs::write(".swp",inputstr).expect("err");
                        fd_image.fd_in = FdValue::File(filename);
                    }
                }
      
            }
        }

        //println!("{}",new_cmds);
        //println!("{:?} {:?}",f_in,f_out);
        //得到更改后的FD_MAP，现在的是new_cmds


        //处理管道
        let mut cmd_num = 0;
        let mut cmds = Vec::new();
        let mut begin_index = 0;
        let mut end_index = 0;
        for ch in new_cmds.bytes() {
            match ch {
                b'|' =>{
                    cmds.push(&new_cmds[begin_index..end_index]);
                    cmd_num += 1;
                    end_index += 1;
                    begin_index = end_index;
                }
                _ => {
                    end_index += 1;
                }
            }
            if end_index == new_cmds.len(){
                cmds.push(&new_cmds[begin_index..end_index]);
                cmd_num += 1;
                break;
            }

        }

        let mut args_vec = Vec::new();
        for cmd in cmds.iter().rev() {
            args_vec.push(cmd.split_whitespace());   
        }

        //分析f
        let mut f_in_num:i32 = 99999999;
        let mut f_in_file:String = String::from("");
        let mut f_out_num:i32 = 99999999;
        let mut f_out_file:String = String::from("");
        let mut child_vec = Vec::new();

        match &fd_image.fd_in {
            FdValue::Num(num) => {
                //println!("Input fd is {}",num);
                f_in_num = *num;
            }
            FdValue::File(file) => {
                //println!("Input fd is {}",file);
                f_in_file = file.to_string();
            }
        }
        match &fd_image.fd_out {
            FdValue::Num(num) => {
                //println!("Output fd is {}",num);
                f_out_num = *num;
            }
            FdValue::File(file) => {
                //println!("Output fd is {}",file);
                //println!("{}",append_flag);
                f_out_file = file.to_string();
            }
        }  
 
        match cmd_num {
            0 => {
                println!("No program input");
            }
            1 => {
                //no pipe
                let mut args = args_vec.pop().unwrap();
                excute_command(&mut args,&fd_image,append_flag);
            }
            2 => {  
                    //one pipe
                    let mut args = args_vec.pop().unwrap();
                    let prog = args.next().unwrap();

                    
                    match f_in_num {
                        0 => {
                            //标准输入
                            child_vec.push(Command::new(prog)
                            .args(args)
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect(""));
                        }
                        _ => {
                            if f_in_num == 99999999 {
                                //文件输入
                                child_vec.push(Command::new(prog)
                                .args(args)
                                .stdin(OpenOptions::new().read(true).open(f_in_file).unwrap())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect(""));
                            }
                            else {
                                //Fd输入
                                child_vec.push(Command::new(prog)
                                .args(args)
                                .stdin(unsafe {File::from_raw_fd(f_out_num)})
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect(""));
                            }

                        }
                    } 
                    let pipe_out = child_vec.pop().unwrap().stdout.expect("d");
                    let mut args = args_vec.pop().unwrap();
                    let prog = args.next().unwrap();
                    //println!("{}",prog);                
                    let outputs = Command::new(prog)
                    .args(args)
                    .stdin(Stdio::from(pipe_out))
                    .output()
                    //.stdout(Stdio::from(f_out.pop().unwrap()))
                    .expect("");
                    
                    match f_out_num {
                        1 => {
                            //标准输出
                            let mut f_out = io::stdout();
                            f_out.write(&outputs.stdout).unwrap();
                        }
                        _ => {
                            if f_out_num == 99999999 {
                                //文件输出
                                if append_flag == 1 {
                                let mut f_out = OpenOptions::new().create(true).append(true).open(f_out_file).unwrap();
                                f_out.write(&outputs.stdout).unwrap();
                                }
                                else{
                                    let mut f_out = OpenOptions::new().create_new(true).write(true).open(f_out_file).unwrap();
                                    f_out.write(&outputs.stdout).unwrap();
                                }
                            }
                            else {
                                let mut f_out = unsafe {File::from_raw_fd(f_out_num)};
                                f_out.write(&outputs.stdout).unwrap();
                            }

                        }
                    } 

                    //print!("{:?}",&outputs.stdout);

            }
            _ =>{   
                    let mut args = args_vec.pop().unwrap();
                    let prog = args.next().unwrap();

                    match f_in_num {
                        0 => {
                            //标准输入
                            child_vec.push(Command::new(prog)
                            .args(args)
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect(""));
                        }
                        _ => {
                            if f_in_num == 99999999 {
                                //文件输入
                                child_vec.push(Command::new(prog)
                                .args(args)
                                .stdin(OpenOptions::new().read(true).open(f_in_file).unwrap())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect(""));
                            }
                            else {
                                //Fd输入
                                child_vec.push(Command::new(prog)
                                .args(args)
                                .stdin(unsafe {File::from_raw_fd(f_out_num)})
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect(""));
                            }
    
                        }
                    } 

                    let mut pipe = Vec::new();
                    pipe.push(child_vec.pop().unwrap().stdout.expect("d"));
                    while cmd_num - 2 >= 1{
                        let mut args = args_vec.pop().unwrap();
                        let prog = args.next().unwrap();
                        let child = Command::new(prog)
                        .args(args)
                        .stdin(Stdio::from(pipe.pop().unwrap()))
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("");
                        pipe.push(child.stdout.expect("d"));
                        cmd_num -= 1;
                    }
                    let mut args = args_vec.pop().unwrap();
                    let prog = args.next().unwrap();
                    let outputs = Command::new(prog)
                    .args(args)
                    .stdin(Stdio::from(pipe.pop().unwrap()))
                    .output()
                    .expect("");

                    match f_out_num {
                        1 => {
                            //标准输出
                            let mut f_out = io::stdout();
                            f_out.write(&outputs.stdout).unwrap();
                        }
                        _ => {
                            if f_out_num == 99999999 {
                                //文件输出
                                if append_flag == 1 {
                                let mut f_out = OpenOptions::new().create(true).append(true).open(f_out_file).unwrap();
                                f_out.write(&outputs.stdout).unwrap();
                                }
                                else{
                                    let mut f_out = OpenOptions::new().create(true).write(true).open(f_out_file).unwrap();
                                    f_out.write(&outputs.stdout).unwrap();
                                }
                            }
                            else {
                                let mut f_out = unsafe {File::from_raw_fd(f_out_num)};
                                f_out.write(&outputs.stdout).unwrap();
                            }

                        }
                    } 

            }
        }

        fd_image = FdImage {
            fd_in: FdValue::Num(0),
            fd_out: FdValue::Num(1),
        };
    
        //println!("{}",cmd);
        //let mut outputs:std::process::Output;
    }
    
}

//执行命令[包括内置和外部]
fn excute_command (args: &mut std::str::SplitWhitespace, fd_image:&FdImage, append_flag:i32) -> bool{
    let prog = args.next();
    let mut f_in_num:i32 = 99999999;
    let mut f_in_file:String = String::from("");
    let mut f_out_num:i32 = 99999999;
    let mut f_out_file:String = String::from("");

    match prog {
        None => {
                println!("Not program input");
                false             
            }
        Some(prog) => {
            //println!("{}",prog);
            match prog {
                "cd" => {
                    let dir = args.next();
                    match dir {
                        Some(dir) => {
                            println!("> The target dir is {}",dir);
                            let stauts = env::set_current_dir(dir);
                            match stauts {
                                Ok(()) => {
                                    println!("> Change dir successfully");
                                    true
                                }
                                _ =>{
                                    println!("> Invalid target dir");
                                    false
                                }
                            }
                        }
                        None => {
                            println!("> No enough args to set current dir");
                            false
                        }
                    }
                    
                }
                "pwd" => {
                    let err = "Getting current dir failed";
                    println!("{}:", env::current_dir().expect(err).to_str().expect(err));
                    false
                }
                "export" => {
                    for arg in args {
                        let mut assign = arg.split("=");
                        let name = assign.next().expect("No variable name");
                        let value = assign.next().expect("No variable value");
                        env::set_var(name, value);
                    }
                    true
                }
                "exit" => {
                    println!("-----See you next time-----");
                    exit(0);
                }
                "x" =>{
                    println!("-----Bye-----");
                    exit(0);
                }
                _ => {
                    //处理外部命令或非法命令
                    //println!("{:?}",args);

                    match &fd_image.fd_in {
                        FdValue::Num(num) => {
                            //println!("Input fd is {}",num);
                            f_in_num = *num;
                        }
                        FdValue::File(file) => {
                            //println!("Input fd is {}",file);
                            f_in_file = file.to_string();
                        }
                    }

                    match &fd_image.fd_out {
                        FdValue::Num(num) => {
                            //println!("Output fd is {}",num);
                            f_out_num = *num;
                        }
                        FdValue::File(file) => {
                            //println!("Output fd is {}",file);
                            //println!("{}",append_flag);
                            f_out_file = file.to_string();
                        }
                    }  

                    match f_in_num{
                        //标准输入
                        0 => {
                            match f_out_num {
                                1 => {
                                    //标准输出
                                    let status = Command::new(prog).args(args).status();
                                    judge_status(status);
                                }
                                _ => {
                                    if f_out_num == 99999999 {
                                        //文件输出
                                        if append_flag == 1 {
                                            let status = Command::new(prog).args(args).stdout(OpenOptions::new().append(true).read(true).write(true).open(f_out_file).unwrap()).status();
                                            judge_status(status);
                                        }
                                        else {
                                            let status = Command::new(prog).args(args).stdout(OpenOptions::new().create(true).read(true).write(true).open(f_out_file).unwrap()).status();
                                            judge_status(status);
                                        }
                                    }
                                    else {
                                        //Fd输出
                                        let status = Command::new(prog).args(args).stdout(unsafe {File::from_raw_fd(f_out_num)}).status();
                                        judge_status(status);
                                    }
                                }
                            }
                        }
                        _ => {
                            if f_in_num == 99999999 {
                                //文件输入    
                                //println!("File input: {}",f_in_file);
                                match f_out_num {
                                    1 => {
                                        //标准输出
                                        let status = Command::new(prog).args(args).stdin(OpenOptions::new().read(true).open(f_in_file).unwrap()).status();
                                        judge_status(status);
                                    }
                                    _ => {
                                        if f_out_num == 99999999 {
                                            //文件输出
                                            //println!("File output {}",f_out_file);
                                            let status = Command::new(prog).args(args).stdin(OpenOptions::new().read(true).open(f_in_file).unwrap())
                                            .stdout(OpenOptions::new().create(true).read(true).write(true).open(f_out_file).unwrap()).status();
                                            judge_status(status);
                                        }
                                        else {
                                            //Fd输出
                                            let status = Command::new(prog).args(args).stdin(OpenOptions::new().read(true).open(f_in_file).unwrap())
                                            .stdout(unsafe {File::from_raw_fd(f_out_num)}).status();
                                            judge_status(status);
                                        }
                                    }
                                }
                            }
                            else {
                                //Fd输入
                                match f_out_num {
                                    1 => {
                                        //标准输出
                                        let status = Command::new(prog).args(args).stdin(unsafe {File::from_raw_fd(f_out_num)}).status();
                                        judge_status(status);
                                    }
                                    _ => {
                                        if f_out_num == 99999999 {
                                            //文件输出
                                            let status = Command::new(prog).args(args).stdin(unsafe {File::from_raw_fd(f_out_num)})
                                            .stdout(OpenOptions::new().create(true).read(true).write(true).open(f_out_file).unwrap()).status();
                                            judge_status(status);
                                        }
                                        else {
                                            //Fd输出
                                            let status = Command::new(prog).args(args).stdin(unsafe {File::from_raw_fd(f_out_num)})
                                            .stdout(unsafe {File::from_raw_fd(f_out_num)}).status();
                                            judge_status(status);
                                        }
                                    }
                                }
                            }

                        }
                    }
                    
                    false
                }
            }
        }
    }

}

fn pre_redirection (cmds: &mut String) -> (bool,String,Vec<Fdtype>){
    //找到并返回重定向的文件名
    //> 和 < 
    //返回所有fd

    //println!("Begin!");
    let mut all_fd: Vec<Fdtype> = Vec::new();

    let mut head_index = 0;
    let mut tail_index = 0;
    let mut cut_head_index = 0;
    let mut cut_tail_index = 0;
    let mut space_cnt = 0;
    let mut last_end = 0;

    let mut new_cmds = String::from("");
    let old_cmds = cmds[..].to_string();

    //默认没有重定向
    let mut fd_type = Fd::None;
    let mut read_flag = 0;
    
    for ch in cmds.bytes() {
        //tail_index
        
        //head_index永远指向最后空格后一位

        //println!("char read now is {}",ch);

        if ch == b'>' || ch == b' ' || ch == b'|' || ch == b'<' || ch == b'&' || ch == b'\n'{ 
            
            //println!("char deal now is {}",ch);

            //置位标志位 update fd_type
            if ch == b'>' || ch == b'<' || ch == b'&'{

                //println!("Begin to deal");

                //Fd::None时候
                if fd_equal(&fd_type,Fd::None){
                    read_flag = 1;
                    cut_head_index = head_index;
                }

                //println!("flag:{},cut_head_index:{}",read_flag,cut_head_index);

                match ch {
                    // > class
                    b'>' => {
                        if fd_equal(&fd_type,Fd::None){
                            fd_type = Fd::Right;
                        }
                        else {
                            fd_type = Fd::DoubleRight;
                        }
                    }
                    b'<' => {
                        // < class
                        if fd_equal(&fd_type,Fd::None){
                            fd_type = Fd::Left;
                        }
                        else {
                            if fd_equal(&fd_type,Fd::Left) {
                                fd_type = Fd::DoubleLeft;
                            }
                            else {
                                fd_type = Fd::TripleLeft;
                            }
                        }
                    }
                    _ => {
                        // >& class
                        if fd_equal(&fd_type,Fd::Right){
                            fd_type = Fd::RightAnd;
                        }

                    }
                    
                }
            }
            else {

                if ch == b'\n'{
                    if read_flag == 1 {
                        read_flag = 0;
                        cut_tail_index = tail_index+1;
                    }
                    else {
                        break;
                    }
                } 

                if read_flag == 1 {
                    if ch == b' '{
                        space_cnt += 1;
                    }

                    if space_cnt == 1 && fd_equal(&fd_type,Fd::RightAnd){
                        read_flag = 0;
                        cut_tail_index = tail_index;
                        space_cnt = 0;
                    }

                    if space_cnt == 2 {
                        cut_tail_index = tail_index;
                        space_cnt = 0;
                    }
                }
            }

            //切割
            if cut_head_index < cut_tail_index {
                //被切部分
                let cmd_cut = cmds[cut_head_index..cut_tail_index].to_string();

                //剩下部分
                if last_end == 0 {
                    new_cmds.push_str(&cmds[..cut_head_index]);
                    last_end = cut_head_index;
                }
                else {
                    new_cmds.clear();
                    new_cmds.push_str(&cmds[..last_end]);
                }

                new_cmds.push_str(&cmds[cut_tail_index..]);

                //解析生成数字与文件名
                let mut choose = 1;
                let mut num1:i32 = 99999999;
                let mut num2:i32 = 99999999;
                let mut filename = String::from("");
                for ch in cmd_cut.bytes() {
                    //是数字
                    if ch >= b'0' && ch <= b'9' {
                        if choose == 1 {
                            if num1 == 99999999{
                                num1 = (ch - b'0').into();
                            }
                            else {
                                let adder:i32 = (ch - b'0').into();
                                num1 = num1*10 + adder;
                            }
                        }
                        else {
                            if num2 == 99999999{
                                num2 = (ch - b'0').into();
                            }
                            else {
                                let adder:i32 = (ch - b'0').into();
                                num2 = num2*10 + adder;
                            }
                        }
                    }
                    else   {//是字符
                        if ch != b'>' && ch != b'<' && ch != b' ' && ch != b'\n' {
                            //字符加入文件名
                            let add_ch = char::from_u32(ch.into()).unwrap();
                            filename.push(add_ch);
                        }
                        else {
                            choose = 2;
                        }
                    
                    }
                }
                match fd_type {
                    Fd::Right =>{
                        if num1 == 99999999 {
                            //stdout
                            let new_fd_type =  Fdtype::Num2File(1,filename);
                            all_fd.push(new_fd_type);
                        }
                        else {
                            //fd
                            let new_fd_type = Fdtype::Num2File(num1,filename);
                            all_fd.push(new_fd_type);
                        }
                    }
                    Fd::DoubleRight =>{
                        if num1 == 99999999{
                            //std
                            let new_fd_type = Fdtype::AddFile(1,filename);
                            all_fd.push(new_fd_type);
                        }
                        else {
                            let new_fd_type = Fdtype::AddFile(num1,filename);
                            all_fd.push(new_fd_type);
                        }
                    }
                    Fd::RightAnd => {
                        if num1 != 99999999 && num2 != 99999999 {
                            let new_fd_type = Fdtype::Num2Num(num1,num2);
                            all_fd.push(new_fd_type);
                        }
                    }
                    Fd::Left => {
                        if num1 == 99999999 {
                            //stdin
                            let new_fd_type = Fdtype::File2Num(0,filename);
                            all_fd.push(new_fd_type);
                        }
                        else {
                            //fd
                            let new_fd_type = Fdtype::File2Num(num1,filename);
                            all_fd.push(new_fd_type);
                        }
                    }
                    Fd::DoubleLeft => {
                        let new_fd_type = Fdtype::EOF2Std;
                        all_fd.push(new_fd_type);

                    }
                    Fd::TripleLeft => {
                        let return_string = filename + "\n";
                        let new_fd_type = Fdtype::Input2Std(return_string);
                        all_fd.push(new_fd_type);
                    }
                    _ => {
                        println!("No FD!");
                    }
                }
                fd_type = Fd::None;

                //println!("num1:{} num2:{}",num1,num2);
            }            
        }

        tail_index += 1;

        if ch == b' '{
            head_index = tail_index;
        }
    }

    //println!("That is {} or {}",new_cmds,old_cmds);

    if all_fd.len() != 0 {
        return (true,new_cmds,all_fd);
    }
    else {
        return (false,old_cmds,all_fd);
    }
}

fn judge_status(status:std::result::Result<std::process::ExitStatus, std::io::Error>) -> bool{
    match status {
        Ok(exitstatus) => {
            match exitstatus.success() {
                false => {
                    println!("> OutSide Command failed to execute");
                    false
                } 
                true => {
                    println!("> OutSide Command Run Correctly");
                    false
                }
            }
        }
        _ => { 
            println!("> Invalid Command");
            false
        }
    }
}