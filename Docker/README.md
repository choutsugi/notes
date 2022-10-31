## 设置阿里镜像源
```bash
sudo nano /etc/docker/daemon.json
```
添加以下内容：
```
{
"registry-mirrors": ["https://y0qd3iq.mirror.aliyuncs.com"]
}
```
> 阿里云镜像服务申请地址：https://cr.console.aliyun.com/cn-shanghai/instances
