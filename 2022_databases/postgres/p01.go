package main

import (
	"fmt"
	"log"
	"time"

	"github.com/d2jvkpn/go-web/pkg/misc"
	"github.com/d2jvkpn/go-web/pkg/orm"
	"github.com/d2jvkpn/go-web/pkg/wrap"
	"github.com/spf13/viper"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/schema"
)

var (
	_DB *gorm.DB
)

func init() {
	misc.RegisterLogPrinter()
}

type User struct {
	ID        int64     `gorm:"column:id;"`
	CreatedAt time.Time `gorm:"column:created_at;autoCreateTime"`
	UpdatedAt time.Time `gorm:"column:updated_at;autoUpdateTime"`
	Status    string    `gorm:"column:status;type:enum('ok','blocked','deleted');default:'ok'"`

	Name         string          `gorm:"column:name"`
	Email        string          `gorm:"column:email"`
	Phone        string          `gorm:"column:phone"`
	Birthday     string          `gorm:"column:birthday"`
	Attributions orm.Map[string] `gorm:"column:attributions;type:json"`
}

func main() {
	var (
		err error
		vc  *viper.Viper
	)

	defer func() {
		if err != nil {
			log.Fatalln(err)
		}
	}()

	if vc, err = wrap.OpenConfig("a01_connect.auth"); err != nil {
		return
	}

	conf := &gorm.Config{
		NamingStrategy: schema.NamingStrategy{SingularTable: true},
	}
	dsn := fmt.Sprintf(
		"host=%s user=%s password=%s dbname=%s port=5432 sslmode=disable TimeZone=Asia/Shanghai",
		vc.GetString("host"), vc.GetString("user"),
		vc.GetString("password"), vc.GetString("database"),
	)
	if _DB, err = gorm.Open(postgres.Open(dsn), conf); err != nil {
		return
	}
	_DB = _DB.Debug()

	jane := User{
		Name:         "Jone",
		Email:        "jone@gmail.com",
		Birthday:     "2000-01-01",
		Attributions: map[string]string{"gender": "female"},
	}

	if err = _DB.Table("users").Create(&jane).Error; err != nil {
		return
	}

	///
	users := []User{}
	if err = _DB.Table("users").Find(&users).Error; err != nil {
		return
	}
	fmt.Println("~~~", users)
}
