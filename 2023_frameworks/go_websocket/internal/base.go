package internal

import (
	"math/rand"
	"net/http"
	"regexp"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/gorilla/websocket"
)

const (
	TimeFormat              = "2006-01-02T15:04:05.000-07:00"
	HTTP_MaxHeaderBytes     = 2 << 20 // 2M
	HTTP_MaxFileHeaderSize  = 8 << 20 // 8M
	HTTP_MaxMultipartMemory = 8 << 20 // 8M
	HTTP_ReadTimeout        = 5
	HTTP_WriteTimeout       = 5
	HTTP_IdleTimeout        = 60
)

var (
	_BuildData   = map[string]string{}
	_Rand        = rand.New(rand.NewSource(time.Now().UnixNano()))
	_LetterRunes = []rune("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
	_Upgrader    = websocket.Upgrader{EnableCompression: true}

	_UserStr     = "^[0-9a-zA-Z-_]{5,32}$"
	_UserRE      = regexp.MustCompile(_UserStr)
	_PasswordStr = "^[a-zA-Z0-9-_.]{8,20}$"
	_PasswordRE  = regexp.MustCompile(_PasswordStr)
	_FilenameStr = "^[0-9a-zA-Z-_][0-9a-zA-Z-_.]{0,31}$"
	_FilenameRE  = regexp.MustCompile(_FilenameStr)
	_UsersData   = NewUsersData()
)

func LoadBuildInfo(info [][2]string) {
	for i := range info {
		_BuildData[info[i][0]] = info[i][1]
	}
}

func RandStringRunes(n int) string {
	b := make([]rune, n)
	for i := range b {
		b[i] = _LetterRunes[_Rand.Intn(len(_LetterRunes))]
	}
	return string(b)
}

func ReadAccounts() map[string]string {
	return _UsersData.users
}

func WrapF(fn http.HandlerFunc) gin.HandlerFunc {
	return func(ctx *gin.Context) {
		fn(ctx.Writer, ctx.Request)
	}
}

func WrapH(hd http.Handler) gin.HandlerFunc {
	return func(ctx *gin.Context) {
		hd.ServeHTTP(ctx.Writer, ctx.Request)
	}
}
