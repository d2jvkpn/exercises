package internal

import (
	"context"
	// "fmt"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/gin-gonic/gin"
)

func NewServer(releaseMode bool, buildInfo [][2]string) (server *http.Server, err error) {
	var (
		engi *gin.Engine
	)

	//// http server
	//	engi.GET("/", func(ctx *gin.Context) {
	//		ctx.String(200, fmt.Sprintf("Hello, %s.\n", time.Now().Format(TIME_FORMAT)))
	//		return
	//	})

	if releaseMode {
		gin.SetMode(gin.ReleaseMode)
		engi = gin.New()
	} else {
		engi = gin.Default()
	}

	engi.Use(CORS, ContextTimeout(10*time.Second))

	engi.NoRoute(func(ctx *gin.Context) {
		ctx.JSON(http.StatusNotFound, gin.H{"code": -1, "message": "no route"})
	})
	engi.MaxMultipartMemory = HTTP_MaxMultipartMemory

	LoadBuildInfo(buildInfo)
	LoadDebugApi(&engi.RouterGroup)
	LoadOpenApi(&engi.RouterGroup)
	LoadAuthApi(&engi.RouterGroup, basicAuth)
	LoadWebsocket(&engi.RouterGroup)
	LoadWebService(&engi.RouterGroup)
	// engi.Static("/web/static", "./static")

	if err = os.MkdirAll(filepath.Join("static", "user_uploads"), 0755); err != nil {
		return nil, err
	}

	// engi.Run(addr)
	server = &http.Server{
		Handler:        engi,
		ReadTimeout:    HTTP_ReadTimeout * time.Second,
		WriteTimeout:   HTTP_WriteTimeout * time.Second,
		MaxHeaderBytes: HTTP_MaxHeaderBytes,
		IdleTimeout:    HTTP_IdleTimeout * time.Second,
	}

	return server, nil
}

func ContextTimeout(t time.Duration) gin.HandlerFunc {
	return func(ctx *gin.Context) {
		c, cancel := context.WithTimeout(ctx.Request.Context(), t)
		defer func() {
			cancel()
			// deadline, _ := c.Deadline()
			if c.Err() == context.DeadlineExceeded {
				ctx.JSON(http.StatusRequestTimeout, nil) // the clients will receive nothing
				ctx.Abort()
			}
		}()

		ctx.Request = ctx.Request.WithContext(c)
		ctx.Next()
	}
}
