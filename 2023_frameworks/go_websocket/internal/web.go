package internal

import (
	"github.com/gin-gonic/gin"
)

func LoadWebService(rg *gin.RouterGroup, handlers ...gin.HandlerFunc) {
	web := rg.Group("/web", handlers...)
	web.Static("/static", "./static") // not list files in dir
}
