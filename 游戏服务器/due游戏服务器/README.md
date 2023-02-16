

## 一、网关服务器

### 1.1 配置

#### 集群服务（gate）

实例ID：网关集群中唯一，未填写则默认生成（uuid）。

实例名称："gate"。

> etcd存储：
>
> ```json
> {
>      "id": "f2510863-add1-11ed-bd81-00ff5786285e",
>      "name": "gate",
>      "kind": "gate",
>      "alias": "gatm",
>      "state": "work",
>      "routes": null,
>      "endpoint": "grpc://192.168.31.175:8661?is_secure=false"
> }
> ```

#### 消息传递

可选GRPC或RPCX。

#### 数据封包

可设置：字节序、序列号字节长度、路由字节长度。

#### 日志

日志组件可选。

#### 用户定位器

使用redis：前缀为`due`。

#### 服务器

可选：websocket、tcp。

#### 服务注册器

使用etcd：命名空间为`services`。

### 1.2 启动

创建容器与网关组件，添加网关组件到容器中并运行容器：main.go

```go
func main() {
	// 创建容器
	container := due.NewContainer()
	// 创建网关组件
	g := gate.NewGate(
		gate.WithServer(ws.NewServer()),				// 服务器（websocket）
		gate.WithLocator(redis.NewLocator()),			// 用户定位器（redis）
		gate.WithRegistry(etcd.NewRegistry()),			// 服务注册器（etcd）
		gate.WithTransporter(rpcx.NewTransporter()),	// 数据传递器（rpcx）
	)
	// 添加网关组件
	container.Add(g)
	// 启动容器
	container.Serve()
}
```

## 二、登录服务器

### 2.1 配置

#### 集群服务（node）

编解码器：proto/json。

### 2.2 启动

创建容器与登录组件，添加登录组件到容器中并运行容器：main.go

```go
func main() {
	// 创建容器
	container := due.NewContainer()
	// 创建网关组件
	n := node.NewNode(
		node.WithLocator(redis.NewLocator()),			// 用户定位器
		node.WithRegistry(etcd.NewRegistry()),			// 服务注册器
		node.WithTransporter(rpcx.NewTransporter()),	// 数据传递器
	)
	// 初始化路由
	route.Init(n.Proxy())
	// 添加网关组件
	container.Add(n)
	// 启动容器
	container.Serve()
}
```

初始化路由：

```go
// 初始化登录逻辑
func Init(proxy node.Proxy) {
	logic.NewLogin(proxy).Init()
}

// 登录逻辑
type login struct {
	proxy node.Proxy
	um    *user.Manager	// 在注册、登录回调中存取用户数据
}

type (
    // 用户信息
    User struct {
		ID       int32  `json:"id"`
		Nickname string `json:"nickname"`
		Account  string `json:"account"`
		Password string `json:"password"`
		Age      int    `json:"age"`
	}

    // 用户管理
	Manager struct {
		id    int32		// 原子自增
		users sync.Map	// 保存用户信息（线程安全，适用于读多写少场景）
	}
)

// 路由ID
const (
	Register      int32 = iota + 1 // 注册账号
	Login                          // 登录账号
)

// 绑定路由到node.Proxy
func (l *login) Init() {
	// 用户注册
	l.proxy.AddRouteHandler(route.Register, false, l.register)
	// 用户登录
	l.proxy.AddRouteHandler(route.Login, false, l.login)
}
```

启动服务后路由信息将注册到etcd中。

## 三、游戏服务器

### 3.1 配置

编解码器：proto/json。

### 







