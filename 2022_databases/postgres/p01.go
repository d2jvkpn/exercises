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
	ID        int64     `gorm:"column:id" json:"id,omitempty"`
	CreatedAt time.Time `gorm:"column:created_at;autoCreateTime" json:"createdAt,omitempty"`
	UpdatedAt time.Time `gorm:"column:updated_at;autoUpdateTime" json:"updatedAt,omitempty"`
	Status    string    `gorm:"column:status;type:enum('ok','blocked','deleted');default:'ok'" json:"status,omitempty"`

	Name         string          `gorm:"column:name" json:"name,omitempty"`
	Email        string          `gorm:"column:email" json:"email,omitempty"`
	Phone        string          `gorm:"column:phone" json:"phone,omitempty"`
	Birthday     string          `gorm:"column:birthday" json:"birthday,omitempty"`
	Attributions orm.Map[string] `gorm:"column:attributions;type:json" json:"attributions,omitempty"`
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
	// dsn: "host=%s user=%s password=%s dbname=%s port=5432 sslmode=disable TimeZone=Asia/Shanghai"
	if _DB, err = gorm.Open(postgres.Open(vc.GetString("dsn")), conf); err != nil {
		return
	}
	_DB = _DB.Debug()

	jane := User{
		Name: "Jone", Email: "jone@gmail.com", Birthday: "2000-01-01",
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
