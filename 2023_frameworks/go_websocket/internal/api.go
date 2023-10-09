package internal

import (
	"expvar"
	"fmt"
	"log"
	"net/http"
	"runtime"
	"runtime/debug"
	"time"

	"github.com/d2jvkpn/pieces/pkg/go/misc"
	"github.com/gin-gonic/gin"
)

func LoadDebugApi(rg *gin.RouterGroup, handlers ...gin.HandlerFunc) {
	expvar.Publish("goroutines", expvar.Func(func() interface{} {
		return runtime.NumGoroutine()
	}))

	expvar.Publish("timestamp", expvar.Func(func() interface{} {
		// return time.Now().Unix()
		return time.Now().Format(time.RFC3339)
	}))

	debugRG := rg.Group("/api/debug", handlers...)
	debugRG.GET("/vars", WrapH(expvar.Handler()))

	buildInfo, _ := debug.ReadBuildInfo()
	debugRG.GET("/build_info", func(ctx *gin.Context) {
		ctx.JSON(http.StatusOK, gin.H{
			"code":    0,
			"message": "ok",
			"data":    gin.H{"buildInfo": buildInfo},
		})
	})
}

func LoadOpenApi(rg *gin.RouterGroup, handlers ...gin.HandlerFunc) {
	open := rg.Group("/api/open", handlers...)

	open.GET("/hello", func(ctx *gin.Context) {
		var (
			name string
			now  string
		)

		if name = ctx.DefaultQuery("name", ""); name == "" { // ctx.Param
			name = "world"
		}
		now = time.Now().Format(time.RFC3339)

		ctx.JSON(http.StatusOK, gin.H{
			"code":    0,
			"message": fmt.Sprintf("Hello, %s, it's %s.", name, now),
			"data":    gin.H{"time": now},
		})
	})

	open.GET("/ping", func(ctx *gin.Context) {
		ip := ctx.ClientIP()
		log.Printf("GET %q request: %s\n", "ping", ctx.Request.RemoteAddr)

		data := make(map[string]interface{}, 3)
		data["ip"], data["build"] = ip, _BuildData

		if h := ctx.Query("header"); h != "" {
			data[h] = ctx.GetHeader(h)
		}

		ctx.JSON(http.StatusOK, gin.H{"code": 0, "message": "ok", "data": data})
		return
	})

	open.GET("/timeout", func(ctx *gin.Context) {
		time.Sleep(20 * time.Second)
		ctx.JSON(http.StatusOK, gin.H{"code": 1, "message": "ok"})
	})

	open.GET("/ntp", WrapF(misc.DelayFunc(1000)))

	open.POST("/register", func(ctx *gin.Context) {
		data := struct {
			User     string `json:"user"`
			Password string `json:"password"`
		}{}

		var (
			code int
			err  error
		)

		if err = ctx.BindJSON(&data); err != nil {
			ctx.JSON(http.StatusBadRequest, gin.H{"code": -1, "message": "unmarshal data failed"})
			return
		}

		if code, err = _UsersData.Register(data.User, data.Password); err != nil {
			ctx.JSON(http.StatusBadRequest, gin.H{"code": code, "message": err.Error()})
			return
		}

		ctx.JSON(http.StatusOK, gin.H{"code": 0, "message": "ok"})
	})
}

func LoadAuthApi(rg *gin.RouterGroup, handlers ...gin.HandlerFunc) {
	// auth := rg.Group("/basic_auth", gin.BasicAuth(ReadAccounts())) // clone map
	// auth := rg.Group("/auth", basic_auth)
	auth := rg.Group("/api/auth", handlers...)

	user := auth.Group("/user")
	user.PUT("/upload", HandleUploadFile)

	user.POST("/unregister", func(ctx *gin.Context) {
		user := ctx.GetString("User")
		_UsersData.Unregister(user)
		ctx.JSON(http.StatusOK, gin.H{"code": 0, "message": "ok"})
	})
}
