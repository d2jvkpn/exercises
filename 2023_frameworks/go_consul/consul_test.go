package cloud

import (
	"fmt"
	"log"
	"net/url"
	"testing"

	"github.com/hashicorp/consul/api"
)

var (
	_ConsulKey       string
	_ConsulApiClient *api.Client
)

func TestConsulUrl(t *testing.T) {
	u, err := url.Parse("http://127.0.0.1:8081")
	if err != nil {
		t.Fatal(err)
	}

	fmt.Printf("%#v\n", u)
}

func _TestConsultClient(t *testing.T) {
	var (
		err    error
		config *api.Config
	)

	config = api.DefaultConfig()
	config.Address, _ConsulKey = "http://127.0.0.1:8500", "my-service"
	config.Token = "7a1798a3-4e9b-d98d-6186-59bbddd1f851" // k/v: mys-service, policy: mys-service

	if _ConsulApiClient, err = api.NewClient(config); err != nil {
		log.Fatalln(err)
	}
}

// $ go test -run TestConsulClient -- http://127.0.0.1:8500
func TestConsulDiscovery_t1(t *testing.T) {
	var (
		err    error
		data   map[string]*api.AgentService
		checks []api.AgentServiceChecksInfo
	)

	_TestConsultClient(t)

	data, err = _ConsulApiClient.
		Agent().
		ServicesWithFilter(fmt.Sprintf(`Service == "%s"`, _ConsulKey))
	if err != nil {
		t.Fatal(err)
	}

	for k, v := range data {
		// fmt.Printf(">>> %#v\n", v)
		fmt.Printf(
			"~~~ key=%q, address=%q, port=%d, ns=%q, kind=%q\n",
			k, v.Address, v.Port, v.Namespace, v.Kind,
		)
	}

	_, checks, err = _ConsulApiClient.
		Agent().
		AgentHealthServiceByNameOpts(_ConsulKey, nil)
	if err != nil {
		t.Fatal(err)
	}

	for _, v := range checks {
		fmt.Printf(">>> %s, %#v\n", v.AggregatedStatus, v.Service) // critical, passing
	}
}

func TestConsulDiscovery_t2(t *testing.T) {
	var (
		err    error
		data   map[string]*api.AgentService
		checks []api.AgentServiceChecksInfo
	)

	service := "greet"

	data, err = _ConsulApiClient.
		Agent().
		ServicesWithFilter(fmt.Sprintf(`Service == "%s"`, service))
	if err != nil {
		t.Fatal(err)
	}

	for k, v := range data {
		// fmt.Printf(">>> %#v\n", v)
		fmt.Printf(
			"~~~ key=%q, address=%q, port=%d, ns=%q, kind=%q\n",
			k, v.Address, v.Port, v.Namespace, v.Kind,
		)
	}

	_, checks, err = _ConsulApiClient.
		Agent().
		AgentHealthServiceByNameOpts(service, nil)
	if err != nil {
		t.Fatal(err)
	}

	for _, v := range checks {
		fmt.Printf(">>> %s, %#v\n", v.AggregatedStatus, v.Service) // critical, passing
	}
}

func TestConsulKV(t *testing.T) {
	pair, meta, err := _ConsulApiClient.KV().Get(_ConsulKey, nil)
	if err != nil {
		t.Fatal(err)
	}
	fmt.Printf("~~~ KVPair: %#v\n    QueryMeta: %#v\n", pair, meta)

	if pair != nil {
		fmt.Printf("    key=%q, value=%q\n", pair.Key, pair.Value)
	}
}
