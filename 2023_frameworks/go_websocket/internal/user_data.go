package internal

import (
	"fmt"
	"sync"
)

type UsersData struct {
	m     *sync.RWMutex
	users map[string]string
}

func NewUsersData() UsersData {
	ud := UsersData{
		m:     new(sync.RWMutex),
		users: make(map[string]string, 100),
	}

	// ud.users["admin"] = "admin"
	return ud
}

func (ud UsersData) UserExists(name string) (ok bool) {
	ud.m.RLock()
	defer ud.m.RUnlock()

	_, ok = ud.users[name]
	return ok
}

func (ud UsersData) Register(user, password string) (code int, err error) {
	println(user, password)
	if !_UserRE.Match([]byte(user)) {
		return -1, fmt.Errorf("user(%s) is invalid", _UserStr)
	}
	if !_PasswordRE.Match([]byte(password)) {
		return -1, fmt.Errorf("password(%s) is invalid", _PasswordStr)
	}

	ud.m.Lock()
	defer ud.m.Unlock()

	if _, ok := ud.users[user]; ok {
		return -2, fmt.Errorf("username already exists")
	}

	ud.users[user] = password
	return 0, nil
}

// Verify password before unregister
func (ud UsersData) Unregister(name string) {
	delete(ud.users, name)
}

func (ud UsersData) Verify(name, password string) (err error) {
	if !_UserRE.Match([]byte(name)) || !_PasswordRE.Match([]byte(password)) {
		return fmt.Errorf("usernamae or password is invalid")
	}

	ud.m.RLock()
	defer ud.m.RUnlock()

	if pw, ok := ud.users[name]; !ok || pw != password {
		return fmt.Errorf("username or password is invalid")
	}

	return nil
}
