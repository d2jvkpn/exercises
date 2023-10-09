package settings

import (
	"strings"

	"github.com/redis/go-redis/v9"
	"github.com/spf13/viper"
)

var (
	_Project *viper.Viper
	RedisCli *redis.Client
)

func SetProject(str string) (err error) {
	_Project = viper.New()
	_Project.SetConfigType("yaml")
	return _Project.ReadConfig(strings.NewReader(str))
}

func Version() string {
	return _Project.GetString("version")
}

func Config() string {
	return _Project.GetString("config")
}

func DotEnv() string {
	return _Project.GetString(".env")
}
