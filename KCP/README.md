## KCP
### Server
```go
package main

import (
	"fmt"
	"github.com/fwhezfwhez/errorx"
	"github.com/xtaci/kcp-go"
	"io"
	"net"
)

func main() {
	fmt.Println("kcp listens on 10000")
	lis, err := kcp.ListenWithOptions(":10000", nil, 10, 3)
	if err != nil {
		panic(err)
	}
	for {
		conn, e := lis.AcceptKCP()
		if e != nil {
			panic(e)
		}
		go func(conn net.Conn) {
			var buffer = make([]byte, 1024, 1024)
			for {
				n, e := conn.Read(buffer)
				if e != nil {
					if e == io.EOF {
						break
					}
					fmt.Println(errorx.Wrap(e))
					break
				}

				fmt.Println("receive from client:", buffer[:n])
				conn.Write(buffer[:n])
			}
		}(conn)

	}
}
```
### Client
```go
package main

import (
	"fmt"
	"github.com/xtaci/kcp-go"
	"time"
)

func main() {
	kcpconn, err := kcp.DialWithOptions("localhost:10000", nil, 10, 3)
	if err != nil {
		panic(err)
	}

	for {
		time.Sleep(time.Second * 1)
		kcpconn.Write([]byte("hello kcp.emmmmmmmmmmmmmmm"))

		var buffer = make([]byte, 1024, 1024)
		n, err := kcpconn.Read(buffer)
		if err != nil {
			panic(err.Error())
		}
		fmt.Println(string(buffer[0:n]))
	}
}
```

## KCP Session
### Server
```go
package main

import (
	"github.com/xtaci/kcp-go"
	"log"
	"net"
	"os"
	"os/signal"
	"syscall"
)

const portEcho = "127.0.0.1:8081"

func listenEcho() (net.Listener, error) {
	return kcp.Listen(portEcho)
}

func handleEcho(sess *kcp.UDPSession) {
	sess.SetWindowSize(4096, 4096)
	sess.SetWriteDelay(true)
	sess.SetACKNoDelay(false)
	// NoDelay options
	// fastest: ikcp_nodelay(kcp, 1, 20, 2, 1)
	// nodelay: 0:disable(default), 1:enable
	// interval: internal update timer interval in millisec, default is 100ms
	// resend: 0:disable fast resend(default), 1:enable fast resend
	// nc: 0:normal congestion control(default), 1:disable congestion control
	sess.SetNoDelay(1, 100, 2, 0)

	for {
		buf := make([]byte, 65536)
		n, err := sess.Read(buf)
		if err != nil {
			panic(err)
		}
		sess.Write(buf[:n])
	}
}

func echoServer() {
	// 获取侦听器
	l, err := listenEcho()
	if err != nil {
		log.Println("listenEcho", err)
		panic(err)
	}

	go func() {
		// 配置listener
		kcplistener := l.(*kcp.Listener)
		kcplistener.SetReadBuffer(4 * 1024 * 1024)
		kcplistener.SetWriteBuffer(4 * 1024 * 1024)
		kcplistener.SetDSCP(46)
		for {
			// 等待接收连接并创建session
			s, err := l.Accept()
			if err != nil {
				log.Println("Accept", err)
				return
			}

			// coverage test
			s.(*kcp.UDPSession).SetReadBuffer(4 * 1024 * 1024)
			s.(*kcp.UDPSession).SetWriteBuffer(4 * 1024 * 1024)
			go handleEcho(s.(*kcp.UDPSession))
		}
	}()
}

func main() {
	echoServer()
	ch := make(chan os.Signal)
	signal.Notify(ch, syscall.SIGTERM, syscall.SIGQUIT, syscall.SIGINT)
	sig := <-ch
	log.Println("get signal", sig)
}
```
### Client
```go
package main

import (
	"bytes"
	"fmt"
	"github.com/xtaci/kcp-go"
	"io"
	"log"
	"time"
)

const serverPortEcho = "127.0.0.1:8081"
const N = 100

func dialEcho() (*kcp.UDPSession, error) {
	conn, err := kcp.Dial(serverPortEcho)
	if err != nil {
		fmt.Println(err)
		panic(err)
	}

	return conn.(*kcp.UDPSession), err
}

func test1() {
	sess, err := dialEcho()
	if err != nil {
		panic(err)
	}
	sess.SetWindowSize(4096, 4096)
	sess.SetWriteDelay(true)
	sess.SetACKNoDelay(false)
	// NoDelay options
	// fastest: ikcp_nodelay(kcp, 1, 20, 2, 1)
	// nodelay: 0:disable(default), 1:enable
	// interval: internal update timer interval in millisec, default is 100ms
	// resend: 0:disable fast resend(default), 1:enable fast resend
	// nc: 0:normal congestion control(default), 1:disable congestion control
	sess.SetNoDelay(1, 100, 2, 0)

	for i := 0; i < N; i++ {
		time.Sleep(1 * time.Second)
		data := time.Now().String()
		sess.Write([]byte(data))
		buf := make([]byte, len(data))
		if n, err := io.ReadFull(sess, buf); err == nil {
			log.Println("got len of(data)", n, data)
			if string(buf[:n]) != data {
				log.Println("不一致", n, len([]byte(data)))
			}
		} else {
			panic(err)
		}

	}
	time.Sleep(1 * time.Second)
	sess.Close()
}

func test2() {
	sess, err := dialEcho()
	if err != nil {
		panic(err)
	}
	sess.SetWindowSize(4096, 4096)
	sess.SetWriteDelay(true)
	sess.SetACKNoDelay(false)
	// NoDelay options
	// fastest: ikcp_nodelay(kcp, 1, 20, 2, 1)
	// nodelay: 0:disable(default), 1:enable
	// interval: internal update timer interval in millisec, default is 100ms
	// resend: 0:disable fast resend(default), 1:enable fast resend
	// nc: 0:normal congestion control(default), 1:disable congestion control
	sess.SetNoDelay(1, 100, 2, 0)

	var buffer bytes.Buffer
	for i := 0; i < 1000; i++ {
		buffer.WriteString(fmt.Sprintf("%5d", i))
	}

	bt := buffer.Bytes()

	for i := 0; i < 1; i++ {
		sess.Write(bt)
		buf := make([]byte, len(bt))
		if n, err := io.ReadFull(sess, buf); err == nil {
			log.Println("got len of(data)", n, buffer.String())
			if string(buf[:n]) != buffer.String() {
				log.Println("不一致", n, len(bt))
			}
		} else {
			panic(err)
		}

	}
	time.Sleep(10 * time.Second)
	sess.Close()
}

func main() {
	// 测试小包
	//test1()
	// 测试拆包的情况
	test2()
}
```