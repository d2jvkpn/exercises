#include <stdio.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <unistd.h>
#include <string.h>

#define IP "127.0.0.1"
#define PORT 8000

int main() {
	int s;
	struct sockaddr_in sock;
	char buf[1024];
	char *data;

	data = "GET / HTTP/1.0\r\n\r\n";

	// man 2 socket
	s = socket(AF_INET, SOCK_STREAM, 0);
	if (s < 0) {
		printf("no.1: socket() error.\n");
		return -1;
	}

	sock.sin_addr.s_addr = inet_addr(IP);
	sock.sin_port = htons(PORT);
	sock.sin_family = AF_INET;

	if (connect(s, (struct sockaddr *)&sock, sizeof(struct sockaddr_in)) != 0) {
		printf("no.2: connect() error.\n");
		close(s);
		return -1;
	}

	ssize_t size;
	size = write(s, data, strlen(data));
	if (size == 0) {
		printf("no.3: write() error.\n");
		close(s);
		return -1;
	};
	printf("==> write: %ld, %ld\n", strlen(data), size);

	memset(buf, 0, sizeof(buf));
	size = read(s, buf, sizeof(buf) - 1);
	if (size == 0) {
		printf("no.4: read() 0.\n");
		close(s);
		return 0;
	}

	close(s);
	printf("==> read: %ld\n\n%s\n", size, buf);
	return 0;
}
