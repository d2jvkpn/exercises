#include <stdio.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>

#define PORT 8000

int main() {
	int s, c;
	socklen_t addrlen;
	struct sockaddr_in srv, cli;
	char buf[512];
	char *data;

	addrlen = 0;
	memset(&srv, 0, sizeof(srv));
	memset(&cli, 0, sizeof(cli));

	s = socket(AF_INET, SOCK_STREAM, 0);
	if (s < 0) {
		printf("no.1: socket() error\n");
		return -1;
	}

	srv.sin_family = AF_INET;
	srv.sin_addr.s_addr = 0;
	srv.sin_port = htons(PORT);

	if (bind(s, (struct sockaddr *)&srv, sizeof(srv))) {
		printf("no.2: binid() error\n");
		close(s);
		return -1;
	}

	if (listen(s, 5)) {
		printf("no.3: listen() error\n");
		close(s);
		return -1;
	};
	printf("==> listening on 0.0.0.0:%d\n", PORT);

	c = accept(s, (struct sockaddr *)&srv, &addrlen);
	if (c < 0) {
		printf("no.4: accept()\n");
		return -1;
	}
	printf("==> client connected: %d\n", c);

	read(c, buf, sizeof(buf) - 1);
	data = "httpd v1.0\n";
	write(c, data, strlen(data));
	close(s);

	return 0;
}
