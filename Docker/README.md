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

## 设置容器时间
docker run添加-v参数：
```bash
docker run -p 3306:3306 --name mysql -v /etc/localtime:/etc/localtime
```

Dockerfile：
```dockerfile
# 方法1
# 添加时区环境变量，亚洲，上海
ENV TimeZone=Asia/Shanghai
# 使用软连接，并且将时区配置覆盖/etc/timezone
RUN ln -snf /usr/share/zoneinfo/$TimeZone /etc/localtime && echo $TimeZone > /etc/timezone

# 方法2
# CentOS
RUN echo "Asia/shanghai" > /etc/timezone
# Ubuntu
RUN cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
```

docker-compose：
```yaml
# 方法1
environment:
  TZ: Asia/Shanghai
  
# 方法2
environment:
  SET_CONTAINER_TIMEZONE=true
  CONTAINER_TIMEZONE=Asia/Shanghai

# 方法3
volumes:
  - /etc/timezone:/etc/timezone
  - /etc/localtime:/etc/localtime
```

执行指令同步：
```bash
# 方法1：直接在宿主机操作
docker cp /etc/localtime 【容器ID或者NAME】:/etc/localtime
docker cp -L /usr/share/zoneinfo/Asia/Shanghai 【容器ID或者NAME】:/etc/localtime

# 方法2：登录容器同步时区timezone
ln -sf /usr/share/zoneinfo/Asia/Singapore /etc/localtime
```

