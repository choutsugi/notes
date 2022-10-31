## 一、环境准备
### 1.1 主机环境
| 主机      | IP             | 服务          |
| --------- | -------------- | ------------- |
| 主机test1 | 172.24.211.113 | Kafka_1 |
| 主机test2 | 172.24.211.114 | Kafka_2 |

开放端口：
```bash
iptables -I INPUT -p tcp --dport 9092 -j ACCEPT
iptables -I INPUT -p tcp --dport 9093 -j ACCEPT
iptables -I INPUT -p tcp --dport 2181 -j ACCEPT
iptables -I INPUT -p tcp --dport 2888 -j ACCEPT
iptables -I INPUT -p tcp --dport 3888 -j ACCEPT
```
### 1.2 Docker环境
安装docker：
```bash
yum install docker
```
安装docker-compose：
```bash
yum install docker-compose
```
启动docker：
```bash
systemctl start docker
```

## 二、容器编排
### 2.1 节点一
docker-compose.yaml（172.24.211.113）
```bash
version: '3.1'
services:
  zookeeper1:
    image: wurstmeister/zookeeper
    restart: always
    hostname: zookeeper1
    container_name: zookeeper1
    ports:
      - 2181:2181
      - 2888:2888
      - 3888:3888
    volumes:
      - /data/zkcluster/zookeeper1/data:/data:Z
      - /data/zkcluster/zookeeper1/datalog:/datalog:Z
    environment:
      ZOO_MY_ID: 1
      ZOO_SERVERS: server.1=172.24.211.113:2888:3888;2181 server.2=172.24.211.114:2888:3888;2181
    network_mode: host

  kafka1:
    image: wurstmeister/kafka
    restart: always
    hostname: kafka1
    container_name: kafka1
    ports:
      - 9092:9092
    environment:
      KAFKA_ADVERTISED_HOST_NAME: 172.24.211.113
      KAFKA_HOST_NAME: 172.24.211.113
      KAFKA_ADVERTISED_PORT: 9092
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: 172.24.211.113:2181,172.24.211.114:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://172.24.211.113:9092
      KAFKA_LISTENERS: PLAINTEXT://172.24.211.113:9092
    volumes:
      - /data/kfkcluster/kafka1/logs:/kafka:Z
    network_mode: host
```
启动：
```bash
root@test1:/# docker-compose up -d
```
查看容器状态：
```bash
root@test1:/# docker ps
CONTAINER ID        IMAGE                    COMMAND                  CREATED             STATUS              PORTS               NAMES
96ee86119eb3        wurstmeister/zookeeper   "/bin/sh -c '/usr/..."   9 minutes ago       Up 9 minutes                            zookeeper1
8aa398a34f3f        wurstmeister/kafka       "start-kafka.sh"         9 minutes ago       Up 9 minutes                            kafka1
```
### 2.1 节点二
docker-compose.yaml（172.24.211.114）
```bash
version: '3.1'
services:
  zookeeper2:
    image: wurstmeister/zookeeper
    restart: always
    hostname: zookeeper2
    container_name: zookeeper2
    ports:
      - 2181:2181
      - 2888:2888
      - 3888:3888
    volumes:
      - /data/zkcluster/zookeeper2/data:/data:Z
      - /data/zkcluster/zookeeper2/datalog:/datalog:Z
    environment:
      ZOO_MY_ID: 2
      ZOO_SERVERS: server.1=172.24.211.113:2888:3888;2181 server.2=172.24.211.114:2888:3888;2181
    network_mode: host

  kafka2:
    image: wurstmeister/kafka
    restart: always
    hostname: kafka2
    container_name: kafka2
    ports:
      - 9092:9092
    environment:
      KAFKA_ADVERTISED_HOST_NAME: 172.24.211.114
      KAFKA_HOST_NAME: 172.24.211.114
      KAFKA_ADVERTISED_PORT: 9092
      KAFKA_BROKER_ID: 2
      KAFKA_ZOOKEEPER_CONNECT: 172.24.211.113:2181,172.24.211.114:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://172.24.211.114:9092
      KAFKA_LISTENERS: PLAINTEXT://172.24.211.114:9092
    volumes:
      - /data/kfkcluster/kafka2/logs:/kafka:Z
    network_mode: host
```
启动：
```bash
root@test2:/# docker-compose up -d
```
查看容器状态：
```bash
root@test2:/# docker ps
CONTAINER ID        IMAGE                    COMMAND                  CREATED             STATUS              PORTS               NAMES
c39a3e69e9b6        wurstmeister/kafka       "start-kafka.sh"         21 minutes ago      Up 21 minutes                           kafka2
549135d4b6f0        wurstmeister/zookeeper   "/bin/sh -c '/usr/..."   21 minutes ago      Up 21 minutes                           zookeeper2
```

## 三、测试
进入容器：
```bash
root@test2:/# docker exec -it c39a3e69e9b6 /bin/bash
```
进入测试脚本目录：
```bash
root@c39a3e69e9b6:/# cd /opt/kafka_2.13-2.8.1/bin
```
### 3.1 创建topic
创建topic（主机一）：
```bash
root@test1:/# kafka-topics.sh --create --topic foo --partitions 5 --replication-factor 2 --bootstrap-server 172.24.211.113:9092,172.24.211.114:9092
Created topic foo.
```
查看topic（主机一）：
```bash
root@test1:/# kafka-topics.sh --list --bootstrap-server 172.24.211.113:9092,172.24.211.114:9092
foo
```
查看topic详情（主机二）：
```bash
root@test2:/# kafka-topics.sh --describe --topic foo --bootstrap-server 172.24.211.113:9092,172.24.211.114:9092
Topic: foo      TopicId: MRj-PkasTnurXnhm6l_yOg PartitionCount: 5       ReplicationFactor: 2    Configs:
        Topic: foo      Partition: 0    Leader: 2       Replicas: 2,1   Isr: 2,1
        Topic: foo      Partition: 1    Leader: 1       Replicas: 1,2   Isr: 1,2
        Topic: foo      Partition: 2    Leader: 2       Replicas: 2,1   Isr: 2,1
        Topic: foo      Partition: 3    Leader: 1       Replicas: 1,2   Isr: 1,2
        Topic: foo      Partition: 4    Leader: 2       Replicas: 2,1   Isr: 2,1
```
### 3.2 生产者与消费者
开启生产者（主机二）：
```bash
root@test2:/# kafka-console-producer.sh --broker-list 172.24.211.113:9092,172.24.211.114:9092 --topic foo
>hello
```
开启消费者（主机一）：

```bash
root@test1:/# kafka-console-consumer.sh --bootstrap-server 172.24.211.113:9092,172.24.211.114:9092 --topic foo --from-beginning
hello
```
### 3.3 删除Topic
```bash
root@test1:/# kafka-topics.sh --delete --topic foo --bootstrap-server 172.24.211.113:9092,172.24.211.114:9092
```
> 删除Topic前务必先停止生产者与消费者，否则删除失败！
