package settings

import (
	// "fmt"
	"context"

	"github.com/redis/go-redis/v9"
	"github.com/spf13/viper"
)

func SetRedisClient(vp *viper.Viper) (err error) {
	RedisCli = redis.NewClient(&redis.Options{
		Addr:     vp.GetString("addr"),
		Password: vp.GetString("password"),
		DB:       vp.GetInt("db"),
	})

	if err = RedisCli.Ping(context.TODO()).Err(); err != nil {
		RedisCli = nil
		return err
	}

	/*
		if err = client.Close(); err != nil {
			log.Fatalln(err)
		}
	*/
	return nil
}
