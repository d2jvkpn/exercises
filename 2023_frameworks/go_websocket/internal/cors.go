package internal

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func CORS(ctx *gin.Context) {
	method := ctx.Request.Method
	ctx.Header("Access-Control-Allow-Origin", "*")

	ctx.Header(
		"Access-Control-Allow-Headers",
		"Content-Type, Access-Token, X-CSRF-Token, Authorization",
	)

	ctx.Header("Access-Control-Allow-Methods", "PUT, GET, POST, OPTIONS")

	ctx.Header(
		"Access-Control-Expose-Headers",
		"Content-Length, Access-Control-Allow-Origin, Access-Control-Allow-Headers, Content-Type",
	)

	ctx.Header("Access-Control-Allow-Credentials", "true")

	if method == "OPTIONS" {
		ctx.AbortWithStatus(http.StatusNoContent)
		return
	}

	ctx.Next()
}
