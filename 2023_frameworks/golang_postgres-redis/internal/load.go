package internal

import (
	// "fmt"

	"golang_postgres-redis/internal/settings"

	"github.com/spf13/viper"
)

func Load(config string, release bool) (err error) {
	var (
		vp *viper.Viper
	)

	//
	vp = viper.New()
	vp.SetConfigType("yaml")
	vp.SetConfigFile(config)
	if err = vp.ReadInConfig(); err != nil {
		return err
	}

	//
	if err = settings.SetRedisClient(vp.Sub("redis")); err != nil {
		return err
	}

	return nil
}
