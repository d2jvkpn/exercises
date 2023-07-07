#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <sys/types.h>
#include <unistd.h>
#include <string.h>

char *tr1(int timeout) {
	static char buf[512];
	fd_set rfds;

	struct timeval tv;
	int ret;

	FD_ZERO(&rfds); // zero out rfds
	FD_SET(0, &rfds); // bind standard input

	tv.tv_sec = timeout;
	tv.tv_usec = 0;

	// parameters: nfds, readfds, writefds, exceptfds, timeout
	ret = select(1, &rfds, 0, 0, &tv);

	// FD_ISSET(0, &rfds) read any data from keyboard
	if (ret && FD_ISSET(0, &rfds)) {
		memset(buf, 0, 512);
		ret = read(0, buf, 512);

		if (ret < 1) { // didn't succeed in reading any data
			return 0;
		}

		ret -= 1;
		buf[ret] = 0;

		return buf;
	} else {
		return 0;
	}
}

int main() {
	char *name;
	printf("What's your name? Think fast!\n");
	name = tr1(3);

	if (name) {
		printf("Hello, %s!\n", name);
		return 0;
	} else {
		printf("Too slow!\n");
		return 1;
	}
}
