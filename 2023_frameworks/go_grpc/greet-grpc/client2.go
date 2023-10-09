package greet

import (
	"context"
	"fmt"
	"io"
	"log"
	"os"
	"sync"
	"time"

	"go.uber.org/multierr"
	"google.golang.org/grpc"
)

type GreetClient struct {
	cli GreetServiceClient
}

func NewGreetClient(conn *grpc.ClientConn) *GreetClient {
	return &GreetClient{
		cli: NewGreetServiceClient(conn),
	}
}

// ->, <-
func (client *GreetClient) Greet(ctx context.Context) (err error) {
	log.Println(">>> GreetClient.Greet")
	var res *GreetResponse

	req := &GreetRequest{Greeting: &Greeting{
		FirstName: "Rover",
		LastName:  "Chan",
	}}

	if res, err = client.cli.Greet(ctx, req); err != nil {
		return err
	}
	fmt.Println("    received:", res.Result)

	return nil
}

// ->, <---
func (client *GreetClient) GreetStream(ctx context.Context) (err error) {
	log.Println(">>> GreetClient.GreetStream")

	var (
		stream GreetService_GreetStreamClient
		res    *GreetResponse
	)

	req := &GreetRequest{Greeting: &Greeting{
		FirstName: "Rover",
		LastName:  "Chan",
	}}

	if stream, err = client.cli.GreetStream(ctx, req); err != nil {
		return err
	}

	for {
		if res, err = stream.Recv(); err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		fmt.Println("    received:", res.Result)
	}

	return nil
}

// --->, <-
func (client *GreetClient) Multiply(ctx context.Context) (err error) {
	log.Println(">>> GreetClient.Multiply")
	var (
		stream GreetService_MultiplyClient
		res    *Number
	)

	if stream, err = client.cli.Multiply(ctx); err != nil {
		return err
	}

	for i := 1; i <= 15; i++ {
		fmt.Println("    send:", i)
		if err = stream.Send(&Number{Value: int64(i)}); err != nil {
			return err
		}
		time.Sleep(time.Second)
	}

	if res, err = stream.CloseAndRecv(); err != nil {
		return err
	}

	fmt.Printf("    result: %d\n", res.Value)
	return nil
}

// ->, <---
func (client *GreetClient) Hello(ctx context.Context) (err error) {
	log.Println(">>> GreetClient.Hello")
	var (
		wg     *sync.WaitGroup
		e1, e2 error
		// mutex  *sync.Mutex
		stream GreetService_HelloClient
	)

	wg = new(sync.WaitGroup)
	// mutex = new(sync.Mutex)
	wg.Add(2)

	//	appendErr := func(e error) {
	//		mutex.Lock()
	//		err = multierr.Append(err, e)
	//		mutex.Unlock()
	//	}

	if stream, err = client.cli.Hello(ctx); err != nil {
		return err
	}

	go func() {
		defer wg.Done()

		for _, v := range []string{"hello", "world", "2022"} {
			req := &Msg{Text: v}
			log.Println("    send:", req.Text)
			if e1 = stream.Send(req); e1 != nil {
				return
			}
			time.Sleep(time.Second)
		}

		e1 = stream.CloseSend()
	}()

	go func() {
		defer wg.Done()
		var msg *Msg

		for {
			if msg, e2 = stream.Recv(); e2 == io.EOF {
				break
			}
			if e2 != nil {
				break
			}
			log.Println("    received:", msg.Text)
		}
	}()

	wg.Wait()
	return multierr.Append(e1, e2)
}

func (client *GreetClient) Upload(ctx context.Context) (err error) {
	log.Println(">>> GreetClient.Upload")
	var (
		n      int
		bts    []byte
		file   *os.File
		req    *File
		res    *Msg
		stream GreetService_UploadClient
	)

	if stream, err = client.cli.Upload(ctx); err != nil {
		return err
	}

	if file, err = os.Open("data/grpc_send.data"); err != nil {
		return err
	}
	defer file.Close()

	bts = make([]byte, 1<<20)
	for i := 0; ; i++ {
		// for i := range bts { bts[i] = '' }
		if n, err = file.Read(bts); err == io.EOF {
			break
		}
		if err != nil {
			if i > 0 {
				req = &File{Text: "action::abort"}
				multierr.Append(err, stream.Send(req))
			}
			return err
		}
		req = &File{Content: bts[:n]}
		if i == 0 {
			req.Text = "filename::data/grpc_recevied.data"
		} else {
			req.Text = "action::upload"
		}

		fmt.Printf("    send: %d, %d\n", i, n)
		if err = stream.Send(req); err != nil {
			return err
		}

		if res, err = stream.Recv(); err == io.EOF {
			return nil
		}
		fmt.Printf("    received: %d, %s\n", res.Code, res.Text)
	}

	err = stream.CloseSend()
	return err
}
