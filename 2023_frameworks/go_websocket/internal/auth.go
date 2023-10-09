package internal

import (
	"encoding/base64"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
)

var basicAuth gin.HandlerFunc = func(ctx *gin.Context) {
	authorization := ctx.GetHeader("Authorization")
	var (
		found          bool
		bts            []byte
		user, password string
		err            error
	)

	if !strings.HasPrefix(authorization, "Basic ") {
		ctx.Header("Www-Authenticate", `Basic realm="Authorization Required"`)
		ctx.JSON(http.StatusUnauthorized, nil)
		ctx.Abort()
		return
	}

	if bts, err = base64.StdEncoding.DecodeString(authorization[6:]); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"code": -1, "message": "decode failed"})
		ctx.Abort()
		return
	}

	if user, password, found = strings.Cut(string(bts), ":"); !found {
		ctx.JSON(http.StatusBadRequest, gin.H{"code": -2, "message": "invalid realm"})
		ctx.Abort()
		return
	}

	if err = _UsersData.Verify(user, password); err != nil {
		ctx.JSON(
			http.StatusUnauthorized,
			gin.H{"code": -3, "message": "username or password is invalid"},
		)
		ctx.Abort()
		return
	}

	ctx.Set("User", user)
	ctx.Next()
}
