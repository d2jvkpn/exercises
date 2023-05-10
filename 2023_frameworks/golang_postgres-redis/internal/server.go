package internal

import (
	"context"
	// "fmt"

	"golang_postgres-redis/internal/settings"
)

func onExit() {
	_ = closeDatabase()
	settings.RedisCli.ShutdownSave(context.TODO())
	// settings.Logger.Down()
}
