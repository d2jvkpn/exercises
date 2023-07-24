#include <stdio.h>
#include <string.h>

int main() {
	char str[32];
	char *p;

	strncpy(str, "I like apples.", 31);
	p = str;
	printf("%c, %s\n", *p, p);

	p += 1;
	printf("%s\n", p);
}
