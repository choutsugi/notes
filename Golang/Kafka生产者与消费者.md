## sarama
自定义消费者组：
```go
package kq

import (
	"github.com/Shopify/sarama"
	"log"
)

type CustomConsumerGroup struct {
}

func (CustomConsumerGroup) Setup(_ sarama.ConsumerGroupSession) error {
	return nil
}

func (CustomConsumerGroup) Cleanup(_ sarama.ConsumerGroupSession) error {
	return nil
}

func (h CustomConsumerGroup) ConsumeClaim(session sarama.ConsumerGroupSession, claim sarama.ConsumerGroupClaim) error {
	for msg := range claim.Messages() {
		switch msg.Topic {
		case "TestTopic":
			// ...
			log.Printf("consumer received msg: %s\n", string(msg.Value))
		default:
			log.Printf("unidentified topic: %s\n", msg.Topic)
		}

		// mark as consumed
		session.MarkMessage(msg, "")
	}
	return nil
}
```

初始化消费者组：
```go
package kq

import (
	"context"
	"github.com/Shopify/sarama"
	"github.com/pkg/errors"
	"log"
)

var (
	ConsumerGroup sarama.ConsumerGroup
)

func InitConsumerGroup(addrs []string, groupId string) {

	cfg := sarama.NewConfig()
	cfg.Version = sarama.V0_10_2_0
	cfg.Consumer.Return.Errors = true
	cfg.Consumer.Offsets.Initial = sarama.OffsetOldest

	var err error
	ConsumerGroup, err = sarama.NewConsumerGroup(addrs, groupId, cfg)
	if err != nil {
		panic(errors.Wrap(err, "failed to init consumer"))
	}

	go func() {
		for err := range ConsumerGroup.Errors() {
			log.Printf("consumer group errors: %+v", err)
		}
	}()

	go func() {
		for {
			topics := []string{"TestTopic"}
			if err := ConsumerGroup.Consume(context.Background(), topics, CustomConsumerGroup{}); err != nil {
				log.Printf("consume failed; err: %+v", err)
				return
			}
		}
	}()
}
```
初始化生产者：
```go
package kq

import (
	"github.com/Shopify/sarama"
	"github.com/pkg/errors"
)

var (
	Producer *SyncProducer
)

func InitProducer(addrs []string) {
	cfg := sarama.NewConfig()
	cfg.Producer.Return.Successes = true
	cfg.Producer.Return.Errors = true
	cfg.Producer.RequiredAcks = sarama.WaitForAll
	cfg.Producer.Partitioner = sarama.NewRandomPartitioner

	cli, err := sarama.NewSyncProducer(addrs, cfg)
	if err != nil {
		panic(errors.Wrap(err, "failed to init producer"))
	}
	Producer = &SyncProducer{cli: cli}
}
```
测试程序：
```go
package main

import (
	"fmt"
	"kafka_test/kq"
	"log"
	"time"
)

func main() {

	addrs := []string{"127.0.0.1:9092", "127.0.0.1:9094"}

    // 初始化生产者
	kq.InitProducer(addrs)
    // 初始化消费者
	kq.InitConsumerGroup(addrs, "TestGroup")

	for i := 0; i < 10; i++ {
		value := fmt.Sprintf("TEST:%d:%s", i, time.Now().Format("2006-01-02 15:04:05"))
		if err := kq.Producer.SendMessage("TestTopic", value); err != nil {
			log.Printf("%+v", err)
		}
		fmt.Println("Send message: ", value)
	}

	select {}
}
```