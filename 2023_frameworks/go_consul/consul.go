package cloud

import (
	_ "embed"
	"fmt"
	"strings"
	"time"

	"github.com/d2jvkpn/gotk"
	// "github.com/google/uuid"
	"github.com/hashicorp/consul/api"
	// "github.com/hashicorp/consul/connect"
	"github.com/spf13/viper"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	healthV1 "google.golang.org/grpc/health/grpc_health_v1"
)

var (
	//go:embed consul.yaml
	_ConsulConfig string
)

/*
Configuration:

```yaml
consul:

	address: http://127.0.0.1:8500
	secret_id: xxxxyyyy

	# service_id could be empty string
	name: "my-service"
	tags: ["A", "B", "C"]
	registry: true
	# default values
	timeout: "5s"
	interval: "5s"
	dcsa: "15s" # deregister_critical_service_after

```
*/
type ConsulClient struct {
	Address  string `mapstructure:"address" json:"address"`              // consul address
	SecretId string `mapstructure:"secret_id" json:"secretId,omitempty"` // consul secretId

	Name     string   `mapstructure:"name" json:"name,omitempty"`         // name of service
	Registry bool     `mapstructure:"registry" json:"registry,omitempty"` // enable registry or not
	Tags     []string `mapstructure:"tags" json:"tags,omitempty"`         // service tags
	Timeout  string   `mapstructure:"timeout" json:"timeout,omitempty"`   // api.AgentServiceCheck.Timeout
	Interval string   `mapstructure:"interval" json:"interval,omitempty"` // api.AgentServiceCheck.Interval
	DCSA     string   `mapstructure:"dcsa" json:"dcsa,omitempty"`         // api.AgentServiceCheck.DeregisterCriticalServiceAfter

	*api.Client `mapstructure:"-"`
	serviceId   string
	ip          string
}

func ConsulConfigDemo() string {
	return _ConsulConfig
}

// fp: consul yaml config file
func NewConsulClient(fp, field string) (cc *ConsulClient, err error) {
	var (
		vc *viper.Viper
		ac *api.Config
		// svc *connect.Service
	)

	///
	//	if err = UnmarshalConfig(fp, map[string]any{"consul": &cc}); err != nil {
	//		return nil, err
	//	}
	if vc, err = gotk.LoadYamlConfig(fp, "..."); err != nil {
		return nil, err
	}
	vc = vc.Sub(field)
	vc.SetDefault("timeout", "5s")
	vc.SetDefault("interval", "5s")
	vc.SetDefault("dcsa", "30s")
	cc = new(ConsulClient)
	if err = vc.Unmarshal(cc); err != nil {
		return nil, err
	}

	if cc.Address == "" {
		return nil, fmt.Errorf("empty address")
	}
	if cc.ip, err = gotk.GetLocalIP(); err != nil {
		return nil, err
	}
	cc.Tags = append(cc.Tags, fmt.Sprintf("at:%q", time.Now().Format(time.RFC3339)))

	// Create a Consul API client
	ac = api.DefaultConfig()
	ac.Address = cc.Address
	ac.Token = cc.SecretId

	if cc.Client, err = api.NewClient(ac); err != nil {
		return nil, err
	}

	// Create an instance representing this service. "my-service" is the
	// name of _this_ service. The service should be cleaned up via Close.
	// if svc, err = connect.NewService(cc.Name, cc.Client); err != nil {
	// 	return nil, err
	// }
	// svc.Close()
	// fmt.Printf("~~~ %#v, %t\n", svc, svc.ServerTLSConfig() == nil)
	// http.Server.TLSConfig: svc.ServerTLSConfig()

	return cc, nil
}

func (cc *ConsulClient) SetServiceId(id string) {
	cc.serviceId = id
}

func (cc *ConsulClient) GetKV(key string) (vc *viper.Viper, err error) {
	var pair *api.KVPair

	// (k *KV) Get(key string, q *QueryOptions) (*KVPair, *QueryMeta, error)
	if pair, _, err = cc.KV().Get(key, nil); err != nil {
		return nil, err
	}
	if pair == nil {
		return nil, fmt.Errorf("key not found")
	}

	if vc, err = gotk.LoadYamlBytes(pair.Value); err != nil {
		return nil, err
	}

	return vc, nil
}

// p: http health path, port: service port
func (cc *ConsulClient) HTTPRegister(port int, useTLS bool, p string) (err error) {
	var (
		kind string
		// svc                  *connect.Service
	)

	// cc.Tags = append(cc.Tags, fmt.Sprintf("at:%s", time.Now().Format(time.RFC3339)))
	if cc.serviceId == "" {
		cc.serviceId = fmt.Sprintf("%s:%d", cc.ip, port) // uuid.NewString()
	}

	if kind = "HTTP"; useTLS {
		kind = "HTTPS"
	}

	registration := &api.AgentServiceRegistration{
		Kind:    api.ServiceKind(kind),
		ID:      cc.serviceId,
		Name:    cc.Name,
		Address: cc.ip, // fmt.Sprintf("%s://%s", scheme, ip),
		Port:    port,
		Tags:    cc.Tags,
	}

	if p != "" {
		registration.Check = &api.AgentServiceCheck{
			HTTP:     fmt.Sprintf("%s://%s:%d%s", strings.ToLower(kind), cc.ip, port, p),
			Timeout:  cc.Timeout,
			Interval: cc.Interval,

			DeregisterCriticalServiceAfter: cc.DCSA,
		}
	}

	if err = cc.Agent().ServiceRegister(registration); err != nil {
		return err
	}

	// Create an instance representing this service. "my-service" is the
	// name of _this_ service. The service should be cleaned up via Close.
	//	if svc, err = connect.NewService(serviceName, client); err != nil {
	//		log.Fatalln(err)
	//	}
	//	defer svc.Close()
	// http.Server.TLSConfig: svc.ServerTLSConfig()

	return nil
}

func (cc *ConsulClient) GRPCRegister(port int, useTLS bool, srv *grpc.Server) (err error) {
	// cc.Tags = append(cc.Tags, fmt.Sprintf("at:%s", time.Now().Format(time.RFC3339)))
	if cc.serviceId == "" {
		cc.serviceId = fmt.Sprintf("%s:%d", cc.ip, port) // uuid.NewString()
	}

	registration := &api.AgentServiceRegistration{
		Kind:    api.ServiceKind("GRPC"),
		ID:      cc.serviceId,
		Name:    cc.Name,
		Address: cc.ip,
		Port:    port,
		Tags:    cc.Tags,
	}

	if srv != nil {
		healthV1.RegisterHealthServer(srv, health.NewServer())

		registration.Check = &api.AgentServiceCheck{
			GRPC:       fmt.Sprintf("%s:%d", cc.ip, port),
			GRPCUseTLS: useTLS,
			Timeout:    cc.Timeout,
			Interval:   cc.Interval,

			DeregisterCriticalServiceAfter: cc.DCSA,
		}
	}

	if err = cc.Agent().ServiceRegister(registration); err != nil {
		return err
	}

	return nil
}

func (cc *ConsulClient) Deregister() (err error) {
	return cc.Agent().ServiceDeregister(cc.serviceId)
}

func (cc *ConsulClient) GetServices(name string) (services []*api.AgentService, err error) {
	var data map[string]*api.AgentService

	data, err = cc.Agent().ServicesWithFilter(fmt.Sprintf(`Service == "%s"`, name))
	if err != nil {
		return nil, err
	}

	for _, v := range data { // serviceId as key
		services = append(services, v)
	}

	return services, nil
}

func (cc *ConsulClient) GetHealthServices(name string, q *api.QueryOptions) (
	services []*api.AgentService, err error) {

	var checks []api.AgentServiceChecksInfo

	if _, checks, err = cc.Agent().AgentHealthServiceByNameOpts(name, q); err != nil {
		return nil, err
	}
	services = make([]*api.AgentService, 0, len(checks))

	for _, v := range checks {
		if v.AggregatedStatus == "passing" {
			services = append(services, v.Service)
		}
	}

	return services, nil
}
