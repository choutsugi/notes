## 一、Kubernetes简介

Kubernetes是一个开源的容器编排引擎和容器集群管理工具，用于对容器化应用进行自动化部署、扩缩和管理。主要功能：

- 服务发现与负载均衡：使用DNS名称或自身IP暴露容器..
- 存储编排：允许自动挂载存储系统，如本地存储、公共云存储。
- 自动部署与回滚：自动化部署..
- 自动装箱计算：每个容器所需的资源（CPU与RAM），以最佳方式利用资源。
- 自我修复：重启失败容器、替换容器、杀死不响应用户定义的运行状况检查的容器，未准备好服务之前不向客户端通告。
- 密钥与配置管理：允许存储和管理敏感信息，如密码、令牌和密钥，可在不重建容器镜像的情况下部署、更新密钥或应用程序配置。

> 云原生：以容器技术为载体、基于为u服务架构思想的一套技术体系与方法论。

## 二、Kubernetes架构

一个Kubernetes集群至少包含一个控制平面（control plane）和一个或多个工作节点（worker node）。

- 控制平面组件：负责管理工作节点和维护集群状态，所有任务分配来自于控制平面。
- 工作节点组件：负责执行由控制平面分配的请求任务，运行实际的应用和工作负载。

### 控制平面组件

控制平面为集群做出全局决策，如资源调度、检测和响应集群事件，控制平面组件包含：

- kube-apiserver：api处理内部和外部请求，即通过api与Kubernetes集群交互。
- kube-scheduler：调度程序考虑容器集的资源（CPU与RAM）需求以及集群的运行状况，将容器安排到适当的计算节点。
- kube-controller-manager：控制器负责实际运行集群，控制器管理器将多个控制器功能合而为一以降低复杂度，控制器管理器包含以下控制器：
  - 节点控制器（Node Controller）：负责节点在出现故障时进行通知和响应。
  - 任务控制器（Job Controller）：检测代表一次性任务的Job对象，然后创建Pods来运行这些任务直至完成。
  - 端点控制器（Endpoints Controller）：填充端点（Endpoints）对象，即加入Service与Pod。
  - 服务账户和令牌控制器（Service Account & Token Controllers）：为新的命名空间创建默认账户和API访问令牌。
- cloud-controller-manager（可选）：云控制器管理器允许将Kubernetes集权连接到云提供商的API之上..
- etcd：..

> 控制平面类比于【公司总部】
>
> - kube-controller-manager：CEO，管理公司、制定决策。
> - etcd：秘书，保管资料。
> - kube-scheduler：分管副总，负责执行与调度。
> - kube-apiserver：行政部门，上传下达。

### 工作节点组件

工作节点组件在每个节点上运行，负责维护运行的Pod组件并提供Kubernetes运行环境，工作节点组件包含：

- kubelet：kubelet在集群中每个节点上运行，保证容器都运行在Pod中；当控制平面需要在某个节点执行某个操作时，kubelet将执行该操作。
- kube-proxy：kube-proxy是集群中每个节点上运行的网络代理，负责维护节点网络规则和转发流量，实现从集群内部或外部的网络与Pod进行网络通信。
- Container runtime：容器运行时是负责运行容器的软件；Kubernetes支持许多容器运行环境，如containerd、docker或其它实现了Kubernetes CRI（容器运行环境接口）的容器。

> 工作节点类比于【分公司】
>
> - kubelet：分公司负责人，负责执行总部下发的任务。
> - kube-proxy：联络员，项目对接。

## 三、安装Minikube

[Minikube](https://minikube.sigs.k8s.io/docs/start/)是一个可本地运行的单机版Kubernetes，安装时需使用Docker或虚拟机，运行环境要求：CPU2核、内存2GB、硬盘20GB、可联网及Docker（或虚拟机）。

安装：

```bash
# 下载
wget https://github.com/kubernetes/minikube/releases/download/v1.28.0/minikube-installer.exe
# 安装
# 启动
minikube start --image-mirror-country='cn' --container-runtime=containerd
# 查看节点状态
kubectl get node
# 查看组件状态 
kubectl get pod -A
# 进入容器
minikube ssh
# 查看kubelet状态
systemctl status kubelet
```

## 四、安装K3s

 K3s集群分为K3s Server（控制平面）和K3s Agent（工作节点），所有组件均打包在单个二进制文件中。

### 运行环境

- 内存：512MB

- CPU：1核心
- K3s版本：v1.25.0+k3s1

### 集群规划

| 主机名      | IP             | 配置                  | 系统      | 网络                                 |
| ----------- | -------------- | --------------------- | --------- | ------------------------------------ |
| k8s-master  | 192.168.56.109 | 内存2G/CPU2核/硬盘20G | CentOS7.9 | 内部网络：NAT网络；互联网：Host-only |
| k8s-worker1 | 192.168.56.111 | 内存2G/CPU2核/硬盘20G | CentOS7.9 | 内部网络：NAT网络；互联网：Host-only |
| k8s-worker2 | 192.168.56.112 | 内存2G/CPU2核/硬盘20G | CentOS7.9 | 内部网络：NAT网络；互联网：Host-only |

设置主机名：

```bash
hostnamectl set-hostname k8s-master
hostnamectl set-hostname k8s-worker1
hostnamectl set-hostname k8s-worker2
```

### 准备工作

关闭防火墙：

```bash
systemctl disable firewalld --now
# systemctl disable nm-cloud-setup.service nm-cloud-setup.timer
```

设置selinux（需联网）：

```bash
yum install -y container-selinux selinux-policy-base
yum install -y https://rpm.rancher.io/k3s/latest/common/centos/7/noarch/k3s-selinux-0.2-1.el7_8.noarch.rpm
```

### 下载安装包

安装脚本：[install.sh](https://get.k3s.io/)

k3s二进制文件：https://github.com/k3s-io/k3s/releases/download/v1.25.0%2Bk3s1/k3s

镜像：https://github.com/k3s-io/k3s/releases/download/v1.25.0%2Bk3s1/k3s-airgap-images-amd64.tar.gz

> 以上文件均可在https://github.com/k3s-io/k3s中找到。

### 执行安装脚本

步骤一：将k3s二进制文件移动到`/usr/local/bin`目录下，并添加可执行权限：

```bash
mv k3s /usr/local/bin
chmod +x /usr/local/bin/k3s
```

步骤二：将镜像移动到`/var/lib/rancher/k3s/agent/images/`目录下：

```bash
mkdir -p /var/lib/rancher/k3s/agent/images/
cp ./k3s-airgap-images-amd64.tar.gz /var/lib/rancher/k3s/agent/images/
```

- 在`k8s-master`节点执行：

  ```bash
  #修改权限
  chmod +x install.sh
  #离线安装
  INSTALL_K3S_SKIP_DOWNLOAD=true ./install.sh
  #安装完成后，查看节点状态
  kubectl get node
  #查看token
  cat /var/lib/rancher/k3s/server/node-token
  #K10c4b79481685b50e4bca2513078f4e83b62d1d0b5f133a8a668b65c8f9249c53e::server:bf7b63be7f3471838cbafa12c1a1964d
  ```

- 在`k8s-worker1`和`k8s-worker2`节点执行：

  ```bash
  INSTALL_K3S_SKIP_DOWNLOAD=true \
  K3S_URL=https://192.168.56.109:6443 \
  K3S_TOKEN=K1012bdc3ffe7a5d89ecb125e56c38f9fe84a9f9aed6db605f7698fa744f2f2f12f::server:fdf33f4921dd607cadf2ae3c8eaf6ad9 \
  ./install.sh
  ```

执行失败的可能原因：

- 时间不统一
- IP冲突
- 主机名重复
- MAC冲突

### 镜像加速

> Kubernetes自`V1.24`版本开始默认使用`containerd`，欲使Pod镜像使用镜像加速器必先修改`containerd`的配置文件，配置文件路径一般为`/etc/containerd/config.toml`。

K3s会自动生成containerd的配置文件，位于`/var/lib/rancher/k3s/agent/etc/containerd/config.toml`，为简化配置，k3s可通过`/etc/rancher/k3s/registries.yaml`文件配置镜像仓库，故在每个节点上新建配置文件，内容如下：

```yaml
mirrors:
  docker.io:
    endpoint:
      - "https://fsp2sfpr.mirror.aliyuncs.com/"
```

重启k3s（master节点）：

```bash
systemctl restart k3s
```

重启k3s-agent（worker1和worker2节点）：

```bash
systemctl restart k3s-agent
```

查看containerd配置文件：

```yaml
cat /var/lib/rancher/k3s/agent/etc/containerd/config.toml
```

## 五、Pod

Pod是包含一个或多个容器的容器组，是Kubernetes中创建和管理的最小对象；Pod特点如下：

- Pod是Kubernetes中最小的调度单位，Kubernetes直接管理Pod而非容器。
- 同一Pod中的容器总是会被自动安排到集群中的同一节点，并一起调度。
- Pod可理解为运行特定应用的“逻辑主机”，其中容器共享存储、网络和配置声明（如资源限制）。
- 每个Pod有唯一的IP地址（IP分配到Pod而非容器），同一Pod内的所有容器共享一个IP地址和端口空间，Pod内的容器可使用localhost互相通信。

### 创建和管理Pod

创建Pod：

```bash
kubectl run mynginx --image=nginx:1.22
```

查看Pod：

```bash
kubectl get pod
```

查看Pod运行日志：

```bash
kubectl logs -f mynginx
```

查看Pod信息：

```bash
kubectl describe pod mynginx
```

查看Pod的IP和运行节点信息：

```bash
kubectl get pod -owide
```

根据Pod的IP访问：

```bash
curl 10.42.1.3
```

进入Pod：

```bash
kubectl exec -it mynginx -- /bin/bash
```

创建一次性Pod（退出时自动删除）：

```bash
kubectl run my-busybox --image=busybox -it --rm
```

> --rm参数：退出Pod时自动删除容器。

删除Pod：

```bash
kubectl delete pod mynginx
```

## 六、Deployment（部署）与ReplicaSet（副本集）

***Deployment***，ReplicaSet和Pod更高级的抽象，使得Pod拥有多副本、自愈、扩缩容、滚动升级等能力。

***ReplicaSet***，一个Pod的集合，可设置运行Pod的数量，确保任何时间都有指定数量的Pod副本在运行；通常无需直接使用ReplicaSet，而是在Deployment中声明。

创建Deployment：

```bash
kubectl create deployment nginx-deploy --image=nginx:1.22 --replicas=3
```

查看Deployment：

```bash
kubectl get deploy
```

查看Pod：

```bash
kubectl get pod
```

查看副本集：

```bash
kubectl get replicaSet
```

> 副本集维护指定数量的Pod，当检测到Pod数量小于指定数量时将自动创建Pod（自愈能力）。

监视副本集：

```bash
kubectl get replicaSet --watch
```

手动扩容：

```bash
kubectl scale deploy nginx-deploy --replicas=5
```

手动缩放：

```bash
kubectl scale deploy nginx-deploy --replicas=3
```

自动缩放（略）：

```bash
kubectl autoscale deployment/nginx-auto --min=3 --max=10 --cpu-percent=75
```

滚动升级：

```bash
kubectl set image deploy/nginx-deploy nginx=nginx:1.23
```

查看部署版本列表：

```bash
kubectl rollout history deploy/nginx-deploy
```

查看部署版本详情：

```bash
kubectl rollout history deploy/nginx-deploy --revision=1
```

版本回滚：

```bash
kubectl rollout undo deploy/nginx-deploy --to-revision=1
```

## 七、Service

Service将运行在一组Pods上的应用程序公开为网络服务的抽象方法。Service为一组Pod提供相同的DNS名，并在它们之间进行负载均衡。

> Kubernetes为Pod分配了IP地址，但IP地址可能会发生变化，创建Service之后，集群内的容器可通过Service名称访问服务，无需担心Pod的IP发生变化。

将Deployment公开为Service：

```bash
kubectl expose deploy/nginx-deploy --name=nginx-service --port=8080 --target-port=80
```

查看Service：

```bash
kubectl get service
```

集群内部通过IP访问Service：

```bash
curl 10.43.228.137:8080
```

通过名称访问Service：

```bash
kubectl run test -it --image=nginx:1.22 --rm --bash
curl nginx-service:8080
```

查看Service信息：

```bash
kubectl describe service nginx-service
```

### 创建Service对象

ServiceType取值：

- ClusterIP：将服务公开在集群内部，Kubernetes为服务分配一个集群内部的IP，集群内的所有主机可通过Cluster-IP访问服务，集群内部的Pod可通过Service名称访问服务。
- NodePort：通过每个节点的主机IP和静态端口暴露服务，集群的外部主机可使用节点IP和NodePort访问服务。
- ExternalName：将集群外部的网络引入集群内部。
- LoadBalancer：使用云服务商的负载均衡器向外部暴露服务。

创建服务：

```bash
kubectl expose deploy/nginx-deploy --name=nginx-outside --port=8081 --target-port=80
```

> 服务公开在每个节点，可通过任意节点的IP访问。

## 八、Namespace

命名空间是一种资源隔离机制，将同一集群中的资源划分为相互隔离的组，同一命名空间中的资源名称应唯一。

查看命名空间：

```bash
kubectl get namespace
```

Kubernetes初始命名空间：

- default：默认命名空间，不可删除，未指定命名空间的对象都会分配到defalut。
- kube-system：Kubernetes系统对象（控制平面和Node组件）所使用的命名空间。
- kube-public：自动创建的公共命名空间，所有用户都可读取，通常用于存储集群中公共可见、可读的资源。
- kube-node-lease：租约对象使用的命名空间，每个节点都有一个关联的lease对象，lease是一种轻量级资源；lease对象通过发送心跳，检测集群中的每个节点是否发生鼓掌。

查看lease对象：

```bash
kubectl get lease -A
```

创建命名空间：

```bash
kubectl create ns develop
```

查看命名空间：

```bash
kubectl get ns
```

在命名空间下创建Pod：

```bash
Kubectl run nginx --image=nginx:1.22 -n=develop
```

查看命名空间下的Pod：

```bash
kubectl get pod -n=develop
```

设置指定命名空间为默认命名空间：

```bash
kubectl config set-context $(kubectl config current-context) --namespace=develop
```

删除命名空间（同时清空命名空间下的所有内容）：

```bash
kubectl delete ns develop
```

> 若命名空间下的某些内容无法自动删除，则命名空间不会被删除。

## 九、声明式对象配置

Kubernetes管理对象的方式：

- 通过命令行指令：使用`kubectl`命令创建和管理Kubernetes对象；不易于追溯，多用于开发与调试。
- 通过声明式配置：使用yaml文件描述Kubertenes对象；操作留痕，多用于生产。

常用命令缩写：

| 名称         | 缩写   | Kind        |
| ------------ | ------ | ----------- |
| namespace    | ns     | Namespace   |
| nodes        | no     | Node        |
| pods         | po     | Pod         |
| services     | svc    | Service     |
| deployments  | deploy | Deployment  |
| replicasets  | rs     | ReplicaSet  |
| statefulsets | sts    | StatefulSet |

### 配置对象

声明Pod：my-pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: my-pod
spec:
  containers:
  - name: nginx
    image: nginx:1.22
    ports:
    - containerPort: 80
```

创建Pod：

```bash
kubectl apply -f my-pod.yaml
```

删除Pod：

```bash
kubectl delete -f my-pod.yaml
```

### 标签

声明包含标签的Pod：my-pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: my-pod-label
  labels:
    environment: production
    app: nginx
spec:
  containers:
  - name: nginx
    image: nginx:1.22
    ports:
    - containerPort: 80
```

查看Pod标签：

```bash
kubectl get pod --show-labels
```

查看指定标签的Pod：

```bash
kubectl get pod -l "app=nginx,environment=production"
```

### 选择器

声明Service：my-service.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: my-service
spec:
  type: NodePort
  selector:
    app: nginx
  ports:
  - port: 80
    targetPort: 80
    nodePort: 30007
```

> 标签通常配置选择器使用。

等值选择：

```yaml
selector:
  matchLabels: # component=redis && version=7.0
    component: redis
    version: 7.0
```

集合选择：

```yaml
selector:
  matchExpressions: # tier in (cache, backend) && environment not in (dev, prod)
    - {key: tier, operator: In, values: [cache, backend]}
    - {key: environment, operator: NotIn, values: [dev, prod]}
```

## 十、容器运行时接口（CRI）与镜像导入、导出

### 容器运行时接口（CRI）

容器运行时接口（CRI）是kubelet和容器运行时之间通信的主要协议；实现了CRI接口的容器引擎，都可作为Kubernetes的容器运行时。（容器运行时接口解耦容器运行时与Kubelet）

> Docker未实现CRI接口，Kubernetes使用`dockershim`兼容docker。自V1.24版本起，Dockershim已从Kubernetes项目中移除。

### 镜像导入、导出

查看正在运行的容器：

```bash
crictl ps
```

查看镜像：

```bash
crictl images
```

从docker导出镜像再导入到containerd中：

```bash
# 从docker导出镜像
docker images
docker pull alpine:3.15
docker save alpine:3.15 > alpine-3.15.tar
# 使用ctr导入镜像
ctr -n k8s.io images import alpine-3.15.tar --platform=linux/amd64
# 查看镜像
crictl images
```

导出镜像：

```bash
ctr -n k8s.io images export alpine-3.15.tar docker.io/library/alpine:3.15 --platform=linux/amd64
```

## 十一、金丝雀发布（灰度发布）

### 发布v1版本

发布v1版本的应用，镜像使用nginx:1.22，数量为3；对象配置文件：deploy-v1.yaml。

deploy-v1.yaml

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: dev
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment-v1
  namespace: dev
  labels:
    app: nginx-deployment-v1
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:1.22
        resources:
          limits:
            memory: "128Mi"
            cpu: "500m"
        ports:
        - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: canary-demo
  namespace: dev
spec:
  type: NodePort
  selector:
    app: nginx
  ports:
  - port: 80
    targetPort: 80
    nodePort: 30008
```

创建对象：

```bash
kubectl apply -f deploy-v1.yaml
kubectl get all -n=dev
```

### 发布v2版本

deploy-canary.yaml（使用Service选择器中选择的标签，创建后将自动加入Service）

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment-canary
  namespace: dev
  labels:
    app: nginx-deployment-canary
spec:
  replicas: 1
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
        track: canary
    spec:
      containers:
      - name: new-nginx
        image: docker/getting-started
        resources:
          limits:
            memory: "128Mi"
            cpu: "500m"
        ports:
        - containerPort: 80
```

扩展V2范围：

```bash
kubectl scale deploy nginx-deployment-canary --replicas=3 -n=dev
```

下线V1：

```bash
kubectl scale deploy nginx-deployment-v1 --replicas=0 -n=dev
```

清空环境：

```bash
kubectl delete all --all -n=dev
```

## 十二、运行有状态应用

### 创建MySQL数据库

mysql-pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: mysql-pod
  labels:
    app: mysql
spec:
  containers:
  - name: mysql
    image: mysql:5.7
    env:
      - name: MYSQL_ROOT_PASSWORD
        value: "123456"
    volumeMounts:
      - mountPath: /var/lib/mysql
        name: data-volume
    resources:
      limits:
        memory: "128Mi"
        cpu: "500m"
    ports:
      - containerPort: 3306
  volumes:
    - name: data-volume
      hostPath:
        path: /home/mysql/data
        type: DirectoryOrCreate
```

创建MySQL Pod：

```bash
kubectl apply -f mysql-pod.yaml
```

查看Pod：

```bash
kubectl get pod -owide
```

### ConfigMap

ConfigMap使用etcd存储非加密数据，可用作环境变量、命令行参数或者存储卷。ConfigMap解耦配置信息与容器镜像，便于配置修改。

> 在ConfigMap中存储的数据不可超过1MiB。

mysql-pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: mysql-pod
  labels:
    app: mysql
spec:
  containers:
  - name: mysql
    image: mysql:5.7
    env:
      - name: MYSQL_ROOT_PASSWORD
        value: "123456"
    volumeMounts:
      - mountPath: /var/lib/mysql
        name: data-volume
      - mountPath: /etc/mysql/conf.d
        name: config-volume
        readOnly: true
    resources:
      limits:
        memory: "128Mi"
        cpu: "500m"
    ports:
      - containerPort: 3306
  volumes:
    - name: config-volume
      configMap:
        name: mysql-config
    - name: data-volume
      hostPath:
        path: /home/mysql/data
        type: DirectoryOrCreate
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: mysql-config
data:
  mysql.cnf: |
    [mysqld]
    character-set-server=utf8mb4
    collation-server=utf8mb4_general_ci
    init-connect='SET NAMES utf8mb4'

    [client]
    default-character-set=utf8mb4

    [mysql]
    default-character-set=utf8mb4
```

创建MySQL Pod：

```yaml
kubectl apply -f mysql-pod.yaml
```

查看ConfigMap：

```bash
kubectl describe cm mysql-config
```

进入Pod：

```bash
kubectl exec -it mysql-pod -- bash
# mysql -uroot -p123456
> show variables like '%char%';
```

编辑配置：

```bash
kubectl edit cm mysql-config
```

> Pod的配置将同步更新。

### Secret

Secret是用于保存机密数据的对象，一般用于保存密码、令牌或密钥等。`data`字段用于存储base64编码数据，`stringData`存储未编码的字符串。Secret可用作环境变量、命令行参数或者存储卷文件。

mysql-pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: mysql-pod
  labels:
    app: mysql
spec:
  containers:
  - name: mysql
    image: mysql:5.7
    env:
      - name: MYSQL_ROOT_PASSWORD
        valueFrom:
          secretKeyRef:
            name: mysql-password
            key: PASSWORD
            optional: false
        value: "123456"
    volumeMounts:
      - mountPath: /var/lib/mysql
        name: data-volume
      - mountPath: /etc/mysql/conf.d
        name: config-volume
        readOnly: true
    resources:
      limits:
        memory: "128Mi"
        cpu: "500m"
    ports:
      - containerPort: 3306
  volumes:
    - name: config-volume
      configMap:
        name: mysql-config
    - name: data-volume
      hostPath:
        path: /home/mysql/data
        type: DirectoryOrCreate
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: mysql-config
data:
  mysql.cnf: |
    [mysqld]
    character-set-server=utf8mb4
    collation-server=utf8mb4_general_ci
    init-connect='SET NAMES utf8mb4'

    [client]
    default-character-set=utf8mb4

    [mysql]
    default-character-set=utf8mb4
---
apiVersion: v1
kind: Secret
metadata:
  name: mysql-password
type: Opaque
data:
  PASSWORD: "cGFzc3dvcmQK"
```

> 修改Secret后，不会自动同步到Pod，必须重启Pod。

### Volume

将数据存储在容器中，一旦容器被删除，数据也将被删除；卷是独立于容器之外的一块存储区域，通过挂载的方式供Pod中的容器使用。卷的使用场景如下：

- 卷可在多个容器之间共享数据。
- 卷可将容器数据存储在外部存储或云存储上。
- 卷更容易备份或迁移。

常见的卷类型：

- 临时卷（Ephemeral Volume）：与Pod一起创建和删除，生命周期同Mod。
- 持久卷（Persistent Volume）：删除Pod后，持久卷不会被删除。
  - 本地存储：
    - hostPath：节点主机上的目录或文件，仅供单节点测试使用；多节点需使用local卷代替。
    - local：节点上挂载的本地存储设备。
- 投射卷（Projected Volumes）：projected卷可将多个卷映射到同一个目录。









































