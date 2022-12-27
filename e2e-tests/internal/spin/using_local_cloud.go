package spin

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"os/exec"
	"sync"
	"time"

	"github.com/sirupsen/logrus"
)

const LocalCloud = "using-local-cloud"

type uselocalcloud struct {
	nomadController  *nomadController
	bindleController *bindleController
	hippoController  *hippoController

	sync.Mutex
}

func WithLocalCloud() (Controller, error) {
	nc, err := startNomad()
	if err != nil {
		return nil, err
	}

	bc, err := startBindle()
	if err != nil {
		return nil, err
	}

	hc, err := startHippo(bc.url)
	if err != nil {
		return nil, err
	}

	return &uselocalcloud{
		nomadController:  nc,
		bindleController: bc,
		hippoController:  hc,
	}, nil
}

type nomadController struct {
	cmd *exec.Cmd
}

type bindleController struct {
	cmd      *exec.Cmd
	url      string
	cacheDir string
}

type hippoController struct {
	url string
	cmd *exec.Cmd
}

func startNomad() (*nomadController, error) {
	cmd := exec.Command("nomad", "agent", "-dev")
	err := startBackgroundProcess(cmd)
	if err != nil {
		return nil, err
	}

	ctx, cancelFunc := context.WithTimeout(context.TODO(), 30*time.Second)
	defer cancelFunc()

	err = waitForTCP(ctx, "localhost:4646")
	if err != nil {
		return nil, err
	}

	return &nomadController{cmd: cmd}, nil
}

func startBindle() (*bindleController, error) {
	port, err := getFreePort()
	if err != nil {
		return nil, err
	}
	address := fmt.Sprintf("127.0.0.1:%d", port)
	url := fmt.Sprintf("http://%s/v1", address)
	cachedir, err := os.MkdirTemp("", "bindle-cache-")
	if err != nil {
		return nil, err
	}

	cmd := exec.Command("bindle-server", "-d", cachedir, "-i", address, "--unauthenticated")
	err = startBackgroundProcess(cmd)
	if err != nil {
		return nil, err
	}

	// ctx, cancelFunc := context.WithTimeout(context.TODO(), 30*time.Second)
	// defer cancelFunc()

	// err = waitForTCP(ctx, address)
	// if err != nil {
	// 	return nil, err
	// }

	return &bindleController{cmd: cmd, cacheDir: cachedir, url: url}, nil
}

func startHippo(bindleUrl string) (*hippoController, error) {
	port, err := getFreePort()
	if err != nil {
		return nil, err
	}
	address := fmt.Sprintf("127.0.0.1:%d", port)
	url := fmt.Sprintf("http://%s", address)

	cmd := exec.Command("Hippo.Web")
	cmd.Env = []string{
		fmt.Sprintf("%s=%s", "ASPNETCORE_URLS", url),
		fmt.Sprintf("%s=%s", "Nomad__Driver", "raw_exec"),
		fmt.Sprintf("%s=%s", "Nomad__Datacenters__0", "dc1"),
		fmt.Sprintf("%s=%s", "Database__Driver", "inmemory"),
		fmt.Sprintf("%s=%s", "ConnectionStrings__Bindle", fmt.Sprintf("Address=%s", bindleUrl)),
		fmt.Sprintf("%s=%s", "Jwt__Key", "ceci n'est pas une jeton"),
		fmt.Sprintf("%s=%s", "Jwt__Issuer", "localhost"),
		fmt.Sprintf("%s=%s", "Jwt__Audience", "localhost"),
	}

	err = startBackgroundProcess(cmd)
	if err != nil {
		return nil, err
	}

	ctx, cancelFunc := context.WithTimeout(context.TODO(), 30*time.Second)
	defer cancelFunc()

	err = waitForTCP(ctx, address)
	if err != nil {
		return nil, err
	}

	ctrl := &hippoController{cmd: cmd}
	httpclient := &http.Client{Timeout: 5 * time.Second}

	username := "rjindal"
	password := "password"

	//register account
	body := map[string]string{
		"userName": username,
		"password": password,
	}

	raw, err := json.Marshal(body)
	if err != nil {
		return ctrl, err
	}

	req, err := http.NewRequest(http.MethodPost, fmt.Sprintf("%s%s", url, "/api/accounts"), bytes.NewReader(raw))
	if err != nil {
		return ctrl, err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := httpclient.Do(req)
	if err != nil {
		return ctrl, err
	}
	defer resp.Body.Close()
	rawresp, err := io.ReadAll(resp.Body)
	if err != nil {
		return ctrl, err
	}

	if resp.StatusCode != http.StatusOK {
		return ctrl, fmt.Errorf("expected: %d, got: %d. rawresp: %s", http.StatusOK, resp.StatusCode, string(rawresp))
	}

	//get token
	return &hippoController{cmd: cmd, url: url}, nil
}

func (o *uselocalcloud) Name() string {
	return SpinUp
}

func (o *uselocalcloud) Login() error {
	args := []string{"login", "--url", o.hippoController.url, "--username", "rjindal", "--password", "password", "--bindle-server", o.bindleController.url}
	return runCmd(exec.Command("spin", args...))
}

func (o *uselocalcloud) TemplatesInstall(args ...string) error {
	return templatesInstall(args...)
}

func (o *uselocalcloud) New(template, appName string) error {
	return new(template, appName)
}

func (o *uselocalcloud) Build(appName string) error {
	return build(appName)
}

func (o *uselocalcloud) Deploy(name string, additionalArgs []string, metadataFetcher func(appname, logs string) (*Metadata, error)) (*Metadata, error) {
	port, err := getFreePort()
	if err != nil {
		return nil, err
	}

	args := []string{"up", "--listen", fmt.Sprintf("127.0.0.1:%d", port)}
	args = append(args, additionalArgs...)

	cmd := exec.Command("spin", args...)
	cmd.Dir = name
	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err = cmd.Start()
	if err != nil {
		return nil, fmt.Errorf("running: %s\nstdout:%s\nstderr:%s\n: %w", cmd.String(), stdout.String(), stderr.String(), err)
	}

	// TODO(rajat): make this dynamic instead of static sleep
	time.Sleep(20 * time.Second)
	return metadataFetcher(name, stdout.String())
}

func (o *uselocalcloud) StopApp(appname string) error {
	return nil
}

// with spin up, we always get latest version
func (o *uselocalcloud) PollForLatestVersion(ctx context.Context, metadata *Metadata) error {
	return nil
}

func (o *uselocalcloud) InstallPlugins(plugins []string) error {
	return installPlugins(plugins...)
}

func (o *uselocalcloud) Teardown() error {
	errs := []error{}
	if o.nomadController != nil {
		err := stopProcess(o.nomadController.cmd)
		if err != nil {
			errs = append(errs, err)
		}
	}

	if o.bindleController != nil {
		err := stopProcess(o.bindleController.cmd)
		if err != nil {
			errs = append(errs, err)
		}

		err = os.RemoveAll(o.bindleController.cacheDir)
		if err != nil {
			errs = append(errs, err)
		}

	}

	if o.hippoController != nil {
		err := stopProcess(o.hippoController.cmd)
		if err != nil {
			errs = append(errs, err)
		}
	}

	if len(errs) > 0 {
		msg := ""
		for _, e := range errs {
			msg += e.Error() + "\n"
		}

		return fmt.Errorf("%s", msg)
	}

	return nil
}

func waitForTCP(ctx context.Context, address string) error {
	ticker := time.NewTicker(1 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return fmt.Errorf("timeout waiting for tcp on %s", address)
		case <-ticker.C:
			_, err := net.Dial("tcp", address)
			if err != nil {
				logrus.Error(err)
				continue
			}

			return nil
		}
	}
}
