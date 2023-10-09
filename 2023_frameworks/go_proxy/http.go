package proxy

import (
	"encoding/base64"
	"fmt"
	"net/http"
	"net/url"

	"golang.org/x/net/proxy"
)

func Http(client *http.Client, proxyAddr string, insecureSkipVerify bool, auths ...proxy.Auth) (err error) {
	var (
		proxyURL  *url.URL
		dialer    proxy.Dialer
		transport *http.Transport
		auth      *proxy.Auth
	)

	if proxyURL, err = url.Parse(proxyAddr); err != nil {
		return err
	}

	if len(auths) > 0 {
		auth = &auths[0]
	}

	switch proxyURL.Scheme {
	case "socks5":
		if dialer, err = proxy.SOCKS5("tcp", proxyURL.Host, auth, nil); err != nil {
			return err
		}
		transport = &http.Transport{Dial: dialer.Dial}
	case "http", "https":
		transport = &http.Transport{Proxy: http.ProxyURL(proxyURL)}
		if auth != nil {
			bts := []byte(auth.User + ":" + auth.Password)
			basicAuth := "Basic " + base64.StdEncoding.EncodeToString(bts)
			transport.ProxyConnectHeader.Add("Authorization", basicAuth) // ?? "Proxy-Authorization"
		}
	default:
		return fmt.Errorf("unknow proxy schema")
	}

	transport.TLSClientConfig.InsecureSkipVerify = insecureSkipVerify
	client.Transport = transport
	return nil
}

/*
	Author: ChatGPT

```golang
import (

	"fmt"
	"net/http"

)

	func main() {
	    // Set up a handler for our protected endpoint
	    http.HandleFunc("/protected", ProtectedEndpoint)

	    // Start the server
	    http.ListenAndServe(":8080", nil)
	}

```
*/
func protectedEndpoint(w http.ResponseWriter, r *http.Request) {
	// Get the user credentials from the Authorization header
	username, password, ok := r.BasicAuth()

	// Check that the credentials are valid
	if !ok || username != "admin" || password != "password" {
		w.Header().Set("WWW-Authenticate", `Basic realm="Restricted"`)
		w.WriteHeader(http.StatusUnauthorized)
		fmt.Fprint(w, "Unauthorized access to this resource.")
		return
	}

	// If the credentials are valid, allow access to the resource
	fmt.Fprint(w, "You have access to this protected resource.")
}
