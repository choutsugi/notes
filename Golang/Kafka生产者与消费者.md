## sarama
### 消费者组
```go
package main

import (
	"context"
	"github.com/Shopify/sarama"
	"log"
)

const (
	TopicForOrderProcess = "foo"
	ConsumerGroupID      = "Group"
)

type MetaAppConsumerGroupHandler struct {
}

func (MetaAppConsumerGroupHandler) Setup(_ sarama.ConsumerGroupSession) error {
	return nil
}

func (MetaAppConsumerGroupHandler) Cleanup(_ sarama.ConsumerGroupSession) error {
	return nil
}

func (h MetaAppConsumerGroupHandler) ConsumeClaim(session sarama.ConsumerGroupSession, claim sarama.ConsumerGroupClaim) error {
	for msg := range claim.Messages() {
		switch msg.Topic {
		case TopicForOrderProcess:
			SuccessCount += 1
			log.Printf("接收消息(%s)，已接收数量为%d：partition = %d, offset = %d", msg.Value, SuccessCount, msg.Partition, msg.Offset)
		default:
			log.Printf("未识别的topic：%s", msg.Topic)
		}
		session.MarkMessage(msg, "")
	}
	return nil
}

var (
	ConsumerGroup sarama.ConsumerGroup
	SuccessCount  int
)

func main() {

	cfg := sarama.NewConfig()
	cfg.Version = sarama.V0_10_2_0
	cfg.Consumer.Return.Errors = true
	cfg.Consumer.Offsets.Initial = sarama.OffsetOldest
	cfg.Consumer.Fetch.Max = 5
	cfg.Consumer.Fetch.Default = 3

	var err error
	ConsumerGroup, err = sarama.NewConsumerGroup([]string{"172.26.114.212:9092", "172.16.0.119:9092"}, ConsumerGroupID, cfg)
	if err != nil {
		panic(err.Error())
	}

	go func() {
		for err := range ConsumerGroup.Errors() {
			log.Printf(err.Error())
		}
	}()

	ctx := context.Background()
	for {
		topics := []string{TopicForOrderProcess}
		handler := MetaAppConsumerGroupHandler{}
		if err := ConsumerGroup.Consume(ctx, topics, handler); err != nil {
			log.Printf(err.Error())
			return
		}
	}

}
```

### 同步生产者
```go
package main

import (
	"github.com/Shopify/sarama"
	"kafka_producer/randstring"
	"log"
	"time"
)

const (
	TopicForOrderProcess = "foo"
)

func main() {

	cfg := sarama.NewConfig()
	cfg.Producer.Return.Successes = true
	cfg.Producer.Return.Errors = true
	cfg.Producer.RequiredAcks = sarama.WaitForAll
	cfg.Producer.Partitioner = sarama.NewRandomPartitioner
	cfg.Producer.Timeout = 5 * time.Second

	client, err := sarama.NewClient([]string{"172.26.114.212:9092", "172.16.0.119:9092"}, cfg)
	if err != nil {
		panic(err.Error())
	}
	defer client.Close()

	producer, err := sarama.NewSyncProducerFromClient(client)
	if err != nil {
		panic(err.Error())
	}

	successCount, failCount := 0, 0

	value := randstring.RandomStr(17)
	startTime := time.Now().Unix()
	for i := 0; i < 10000; i++ {
		partition, offset, err := producer.SendMessage(&sarama.ProducerMessage{
			Topic: TopicForOrderProcess,
			Value: sarama.StringEncoder(value),
		})
		if err != nil {
			failCount += 1
			log.Printf("发送消息（%s）失败（%s）：%d", value, err.Error(), failCount)
			continue
		}
		successCount += 1
		log.Printf("发送消息（%s）成功（%d）：partition = %d, offset = %d", value, successCount, partition, offset)
	}

	log.Printf("成功：%d，失败：%d，耗时：%d", successCount, failCount, time.Now().Unix()-startTime)
}
```
### 异步生产者
```go
package main

import (
	"fmt"
	"github.com/Shopify/sarama"
	"kafka_producer_async/randstring"
	"log"
	"time"
)

const (
	TopicForOrderProcess = "foo"
)

func main() {

	cfg := sarama.NewConfig()
	cfg.Producer.Return.Successes = true
	cfg.Producer.Return.Errors = true
	cfg.Producer.RequiredAcks = sarama.WaitForAll
	cfg.Producer.Partitioner = sarama.NewRandomPartitioner
	cfg.Producer.Timeout = 5 * time.Second

	client, err := sarama.NewClient([]string{"172.26.114.212:9092", "172.16.0.119:9092"}, cfg)
	if err != nil {
		panic(err.Error())
	}
	defer client.Close()

	producer, err := sarama.NewAsyncProducerFromClient(client)
	if err != nil {
		panic(err.Error())
	}

	go func(producer sarama.AsyncProducer) {
		successCount, failCount := 0, 0

		for {
			select {
			case err := <-producer.Errors():
				failCount += 1
				log.Printf("发送消息（%s）失败（%d）：%s", err.Msg.Value, failCount, err.Error())
			case msg := <-producer.Successes():
				successCount += 1
				log.Printf("发送消息（%s）成功（%d）：partition = %d, offset = %d, time = %s", msg.Value, successCount, msg.Partition, msg.Offset, time.Now().Format("2006-01-02 15:04:05"))
			}
		}
	}(producer)

	value := randstring.RandomStr(17)

	fmt.Println(time.Now().Format("2006-01-02 15:04:05"))
	for i := 0; i < 1000; i++ {
		producer.Input() <- &sarama.ProducerMessage{
			Topic: TopicForOrderProcess,
			Value: sarama.StringEncoder(value),
		}
	}

	select {}
}
```