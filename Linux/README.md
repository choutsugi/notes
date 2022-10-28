## 使用终端跨设备拷贝文件

拷贝远程服务器文件到本地：
```bash
scp root@47.100.163.22:/home/jihui/images/kafka.tar ./kafka.tar
```
上传本地文件到服务器：
```bash
scp ./kafka.tar root@101.133.137.176:/home/jihui/images/kafka.tar
```
> 使用-r可拷贝目录。

