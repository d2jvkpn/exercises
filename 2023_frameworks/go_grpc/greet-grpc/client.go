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
)

// ->, <-
func CallGreet(ctx context.Context, client GreetServiceClient) (err error) {
	log.Println(">>> Client.CallGreet")
	var res *GreetResponse

	req := &GreetRequest{Greeting: &Greeting{
		FirstName: "Rover",
		LastName:  "Chan",
	}}

	if res, err = client.Greet(ctx, req); err != nil {
		return err
	}
	fmt.Println("    received:", res.Result)

	return nil
}

// ->, <---
func CallGreetStream(ctx context.Context, client GreetServiceClient) (err error) {
	log.Println(">>> Client.CallGreetStream")

	var (
		stream GreetService_GreetStreamClient
		res    *GreetResponse
	)

	req := &GreetRequest{Greeting: &Greeting{
		FirstName: "Rover",
		LastName:  "Chan",
	}}

	if stream, err = client.GreetStream(ctx, req); err != nil {
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
func CallMultiply(ctx context.Context, client GreetServiceClient) (err error) {
	log.Println(">>> Client.CallMultiply")
	var (
		stream GreetService_MultiplyClient
		res    *Number
	)

	if stream, err = client.Multiply(ctx); err != nil {
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
func CallHello(ctx context.Context, client GreetServiceClient) (err error) {
	log.Println(">>> Client.CallHello")
	var (
		wg     *sync.WaitGroup
		mutex  *sync.Mutex
		stream GreetService_HelloClient
	)

	wg = new(sync.WaitGroup)
	mutex = new(sync.Mutex)
	wg.Add(2)

	appendErr := func(e error) {
		mutex.Lock()
		err = multierr.Append(err, e)
		mutex.Unlock()
	}

	if stream, err = client.Hello(ctx); err != nil {
		return err
	}

	go func() {
		defer wg.Done()

		for _, v := range []string{"hello", "world", "2022"} {
			req := &Msg{Text: v}
			log.Println("    send:", req.Text)
			if e1 := stream.Send(req); e1 != nil {
				appendErr(e1)
				return
			}
			time.Sleep(time.Second)
		}

		if e1 := stream.CloseSend(); e1 != nil {
			appendErr(e1)
		}
	}()

	go func() {
		defer wg.Done()

		for {
			res, e1 := stream.Recv()
			if e1 == io.EOF {
				break
			}
			if e1 != nil {
				appendErr(e1)
			}
			log.Println("    received:", res.Text)
		}
	}()

	wg.Wait()
	return err
}

func CallUpload(ctx context.Context, client GreetServiceClient) (err error) {
	log.Println(">>> Client.CallUpload")
	var (
		n      int
		bts    []byte
		file   *os.File
		req    *File
		res    *Msg
		stream GreetService_UploadClient
	)

	if stream, err = client.Upload(ctx); err != nil {
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
