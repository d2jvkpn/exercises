package internal

import (
	"fmt"
	"log"
	"mime/multipart"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/d2jvkpn/pieces/pkg/go/misc"
	"github.com/gin-gonic/gin"
)

func HandleUploadFile(ctx *gin.Context) {
	var (
		err        error
		fn, target string
		user, dir  string
		fileHeader *multipart.FileHeader
		files      []*multipart.FileHeader
		form       *multipart.Form
		now        time.Time
	)

	// fileHeader, err = ctx.FormFile("file")
	if form, err = ctx.MultipartForm(); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"code": -1, "message": "bad request"})
		return
	}
	files = form.File["files"]
	user = ctx.GetString("User")
	now = time.Now()

	for _, fileHeader = range files {
		fn = fileHeader.Filename
		if fileHeader.Size == 0 {
			ctx.JSON(
				http.StatusBadRequest,
				gin.H{"code": -2, "message": "file is empty: " + fn},
			)
			return
		}

		if !_FilenameRE.Match([]byte(fn)) {
			ctx.JSON(http.StatusBadRequest, gin.H{
				"code": -3, "message": "file name not match ^[0-9a-zA-Z-_][0-9a-zA-Z-_.]{1,31}$",
			})
			return
		}

		if fileHeader.Size > HTTP_MaxFileHeaderSize {
			ctx.JSON(
				http.StatusRequestEntityTooLarge,
				gin.H{"code": -4, "message": "file size is too large: " + fn},
			)
			return
		}
	}

	dir = filepath.Join("static", "user_uploads", user, now.Format("2006-01-02"))
	if err = os.MkdirAll(dir, 0755); err != nil {
		ctx.JSON(
			http.StatusInternalServerError,
			gin.H{"code": 1, "message": "create user upload dir failed"},
		)
	}

	for _, fileHeader = range files {
		fn = fileHeader.Filename
		ext := filepath.Ext(fn)
		fn = fileHeader.Filename[0 : len(fn)-len(ext)]

		target = filepath.Join(dir, fmt.Sprintf(
			"%s.%d_%s%s", fn, now.UnixMilli(), RandStringRunes(16), ext,
		))

		log.Printf(
			"receiving file: source=%q, size=%s\n",
			fileHeader.Filename, misc.FileSize2Str(fileHeader.Size),
		)

		if err = ctx.SaveUploadedFile(fileHeader, target); err != nil {
			log.Printf("save file error: %q, %v\n", target, err)
			ctx.JSON(
				http.StatusInternalServerError,
				gin.H{"code": 1, "message": "failed to save file: " + fn},
			)
			return
		}
	}

	ctx.JSON(http.StatusOK, gin.H{
		"code": 0, "message": "ok",
		"data": gin.H{"url": filepath.Join("/web", target)},
	})
}
