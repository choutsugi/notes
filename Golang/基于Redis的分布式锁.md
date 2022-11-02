源码：
```go
package redlock

import (
	"context"
	"fmt"
	"github.com/go-redis/redis/v8"
	"main/pkg/randstring"
	"strconv"
	"sync/atomic"
	"time"
)

const (
	lockCommand     = `if redis.call("GET", KEYS[1]) == ARGV[1] then redis.call("SET", KEYS[1], ARGV[1], "PX", ARGV[2]) return "OK" else return redis.call("SET", KEYS[1], ARGV[1], "NX", "PX", ARGV[2]) end`
	delCommand      = `if redis.call("GET", KEYS[1]) == ARGV[1] then return redis.call("DEL", KEYS[1]) else return 0 end`
	randomLen       = 16
	tolerance       = 500
	millisPerSecond = 1000
)

type RedLock interface {
	Acquire() (bool, error)
	Block(seconds int64) bool
	Release() (bool, error)
	SetExpire(seconds int) *redLock
}

type redLock struct {
	rdb     *redis.Client
	ctx     context.Context
	name    string
	owner   string
	seconds uint32
}

func (rl *redLock) Acquire() (bool, error) {
	seconds := atomic.LoadUint32(&rl.seconds)
	cmd := rl.rdb.Eval(rl.ctx, lockCommand, []string{rl.name}, []string{
		rl.owner, strconv.Itoa(int(seconds)*millisPerSecond + tolerance),
	})

	err := cmd.Err()
	val := cmd.Val()

	if err == redis.Nil {
		return false, nil
	} else if err != nil {
		return false, fmt.Errorf("error on acquiring lock for %s, %s", rl.name, err.Error())
	} else if val == nil {
		return false, nil
	}
	reply, ok := val.(string)
	if ok && reply == "OK" {
		return true, nil
	}
	return false, fmt.Errorf("unknown reply when acquiring lock for %s: %v", rl.name, val)
}

func (rl *redLock) Block(seconds int64) bool {
	starting := time.Now().Unix()

	for {
		isAcquired, err := rl.Acquire()
		if err == nil && isAcquired {
			return true
		} else {
			time.Sleep(time.Duration(500) * time.Millisecond)
			if time.Now().Unix()-seconds >= starting {
				return false
			}
		}
	}
}

func (rl *redLock) Release() (bool, error) {
	cmd := rl.rdb.Eval(rl.ctx, delCommand, []string{rl.name}, []string{rl.owner})

	err := cmd.Err()
	val := cmd.Val()

	if err != nil {
		return false, err
	}
	reply, ok := val.(int64)
	if !ok {
		return false, nil
	}
	return reply == 1, nil
}

// SetExpire value: (500 + seconds * 1000)ms
func (rl *redLock) SetExpire(seconds int) *redLock {
	atomic.StoreUint32(&rl.seconds, uint32(seconds))
	return rl
}

func New(rdb *redis.Client, name string) RedLock {
	return &redLock{
		ctx:   context.Background(),
		rdb:   rdb,
		name:  name,
		owner: randstring.RandomStr(randomLen),
	}
}
```
