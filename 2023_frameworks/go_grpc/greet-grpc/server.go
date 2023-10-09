package greet

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"

	"go.uber.org/multierr"
)

type Server struct{}

func (srv *Server) Greet(ctx context.Context, req *GreetRequest) (res *GreetResponse, err error) {
	log.Println(">>> Server.Greet")
	res = new(GreetResponse)
	// res.Result = fmt.Sprintf("Hello, %s %s!", req.Greeting.FirstName, req.Greeting.LastName)
	greet := req.GetGreeting()
	firstName := greet.GetFirstName()
	lastName := greet.GetLastName()
	log.Println("    recevied:", firstName, lastName)

	res.Result = fmt.Sprintf("Hello, %s %s!", firstName, lastName)

	return res, nil
}

func (srv *Server) GreetStream(req *GreetRequest, stream GreetService_GreetStreamServer) (
	err error) {

	log.Println(">>> Server.GreetStream")
	greet := req.GetGreeting()
	firstName := greet.GetFirstName()
	lastName := greet.GetLastName()
	log.Println("    recevied:", firstName, lastName)

	for i := 0; i < 10; i++ {
		msg := fmt.Sprintf("Hello, %s %s, number: %d", firstName, lastName, i)
		if err = stream.Send(&GreetResponse{Result: msg}); err != nil {
			break
		}
		time.Sleep(time.Second)
	}

	return err
}

func (srv *Server) Multiply(stream GreetService_MultiplyServer) (err error) {
	log.Println(">>> Server.Multiply")
	var req, res *Number

	res = &Number{Value: 1}
	for {
		if req, err = stream.Recv(); err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		log.Println("    received:", req.Value)
		res.Value *= req.Value
	}

	return stream.SendAndClose(res)
}

func (src *Server) Hello(stream GreetService_HelloServer) (err error) {
	log.Println(">>> Server.Hello")
	var req, res *Msg

	for {
		if req, err = stream.Recv(); err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		log.Println("    received:", req.Text)
		time.Sleep(time.Second)

		res = &Msg{Text: strings.ToUpper(req.Text)}
		log.Println("    send:", res.Text)
		if err = stream.Send(res); err != nil {
			return err
		}
	}

	return nil
}

func (src *Server) Upload(stream GreetService_UploadServer) (err error) {
	log.Println(">>> Server.Upload")
	var (
		fp   string
		file *os.File
		req  *File
		res  *Msg
	)

	defer func() {
		if file != nil {
			_ = file.Close()
		}
	}()

	// pending=0, in_progress=1, success=2, failed=3, abort=4
	for i := 0; ; i++ {
		//
		if req, err = stream.Recv(); err == io.EOF {
			break
		} else if err != nil {
			return err
		}
		log.Printf("    received: %d, %s", len(req.Content), req.Text)

		//
		if i == 0 {
			if fp = filepath.Base(req.Text); fp == "" {
				res = &Msg{Text: "failed: invalid filename", Code: 3}
				err = fmt.Errorf(res.Text)
				return multierr.Append(err, stream.Send(res))
			}

			fp = filepath.Join("data", strings.Replace(fp, "filename::", "", 1))
			if file, err = os.Create(fp); err != nil {
				res = &Msg{Text: "failed: create file", Code: 3}
				log.Println("    send:", res.Code, res.Text)
				return multierr.Append(err, stream.Send(res))
			}
		}
		if req.Text == "::abort" {
			_ = file.Close()
			_ = os.Remove(fp)
			file = nil
			break
		}

		if _, err = file.Write(bytes.Trim(req.Content, "\x00")); err != nil {
			res = &Msg{Text: "failed: write file", Code: 3}
			log.Println("    send:", res.Code, res.Text)
			return multierr.Append(err, stream.Send(res))
		}

		res = &Msg{Text: "in_progress", Code: 1}
		log.Println("    send:", res.Code, res.Text)
		if err = stream.Send(res); err != nil {
			return err
		}

		time.Sleep(3 * time.Second)
	}

	return nil
}
