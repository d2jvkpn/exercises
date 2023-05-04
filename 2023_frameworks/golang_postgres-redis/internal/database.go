package internal

import (
	"database/sql"
	"errors"
	"fmt"

	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/database"
	"github.com/golang-migrate/migrate/v4/database/postgres"
	_ "github.com/golang-migrate/migrate/v4/source/file"
	// _ "github.com/lib/pq"
)

func migratePostgres(source, migrations string) (err error) {
	var (
		db     *sql.DB
		driver database.Driver
		migr   *migrate.Migrate
	)

	if db, err = sql.Open("postgres", source); err != nil {
		return fmt.Errorf("sql.Open: %w", err)
	}

	if driver, err = postgres.WithInstance(db, &postgres.Config{}); err != nil {
		return fmt.Errorf("postgres.WithInstance: %w", err)
	}

	migr, err = migrate.NewWithDatabaseInstance(
		fmt.Sprintf("file://%s", migrations),
		"postgres",
		driver,
	)
	if err != nil {
		return fmt.Errorf("migrate.NewWithDatabaseInstance: %w", err)
	}

	// or m.Step(2) if you want to explicitly set the number of migrations to run
	if err = migr.Up(); err != nil {
		if err.Error() != "no change" {
			return fmt.Errorf("migrate.Up: %w", err)
		}
	}

	e1, e2 := migr.Close()
	return errors.Join(e1, e2)
}
