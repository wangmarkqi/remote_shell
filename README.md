# remote_shell
The backend of remote_shell is a P2P framework upd_hole_punching (https://github.com/wangmarkqi/udp_hole_punching). The use scenarios include iot device with no fixed ip, home computer behind local network, etc. with remote shell client side installed on slave machine, you can access from remote_shell host side like normal ssh process.
![avatar](./data/demo.jpg)

## How to use:


### step 1: Start slave 
To use remote shell, 2 kinds of binary are needed to be build, the salve and the host. This step is for slave side. 
Git clone remote_shell in slave machine and run slave::slave_dispatch::dispatch() in main.rs;
```
  let swap = "x.x.x.x:xxxx".to_string();
  slave::slave_entrance::dispatch("wq", &swap,"./data/slave");

```


### step 2: Start host 
This step  is for host side. 
Git clone remote_shell in host machine and run host::shell::run_shell()
```
    let swap = "x.x.x.x:xxxx".to_string();
    host::shell::run_shell(&swap,"./data/master");

```

### step 3: Stared with command "use"
 First specify slave id by "use <slave id which from step 2>" and send command as normal shell,like "cd /home" etc. 


## Terminal operation:
 You may find the common used key like del,arrow left,arrow right not working ,because remote_shell rewrite the terminal from raw mode. The shell operational keys include:

###  press key arrow up:
Back to last cmd input.

###  press key arrow down:
Go to next cmd input.

###  press key backspace:
Delete input char before.

###  press key Tab:
Complement input cmd based on history inputs.

###  press key Home:
Clear Terminal.

###  press key Esc:
Quit the shell.

## The build_in function: 
Besides standard commands,for example ls,pwd in linux or dir in win10, remote_shell provides following build_in commands. 
### use <slave_id>
This command will specify the slave you want to control.You should run step3 on slave pc and config the slave id in the .env file. This salve_id (use arg) should be same with id in the .env file.

example: use remote

### send <local_file_path> <remote_file_path>
Send local file to remote. only small file works.

example: send C://test.txt /home/test.txt
### rec <local_file_path> <remote_file_path>
Receive file from remote. only small file works.

example: rec C://test1.txt /home/test2.txt

### restart
Will restart remote computer.This command is sent by async message pub channel and act as the final rescue when shell crash. Pay attention to restart remote_shell automatically after system reboot.

example: restart

