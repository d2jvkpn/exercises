package internal

import (
	// "fmt"

	"github.com/jackc/pgconn"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/schema"
)

/*
dsn format:
- {USERANME}:{PASSWORD}@tcp({IP})/{DATABASE}?sslmode=disable
- "host=%s user=%s password=%s dbname=%s port=5432 sslmode=disable TimeZone=Asia/Shanghai"
*/
func Connect(dsn string, debugMode bool) (db *gorm.DB, err error) {
	conf := &gorm.Config{
		NamingStrategy: schema.NamingStrategy{SingularTable: true},
	}
	// dsn: "host=%s user=%s password=%s dbname=%s port=5432 sslmode=disable TimeZone=Asia/Shanghai"
	if _DB, err = gorm.Open(postgres.Open(dsn), conf); err != nil {
		return nil, err
	}

	if debugMode {
		_DB = _DB.Debug()
	}
	/*
		sqlDB, err := db.DB() // Get generic database object sql.DB to use its functions
		sqlDB.SetMaxIdleConns(10)
		sqlDB.SetMaxOpenConns(100)
		sqlDB.SetConnMaxLifetime(time.Hour)
	*/

	return _DB, err
}

func IsUniqueViolation(err error) bool {
	e, ok := err.(*pgconn.PgError)
	return ok && e.Code == "23505"
}

func IsRecordNotFound(err error) bool {
	return err.Error() == "record not found"
}
