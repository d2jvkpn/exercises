package proxy

type BasicAuth struct {
	User     string
	Password string
}

// $ export http_proxy=socks5://hello:world@localhost:8080
func (ba *BasicAuth) Valid(user, password string) bool {
	return user == ba.User && password == ba.Password
}
