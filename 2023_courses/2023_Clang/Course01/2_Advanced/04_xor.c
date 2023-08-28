#include <stdio.h>

#define KEY 'X'
#define CLEAR_TEXT 'A'

/*
0 0 = 0
0 1 = 1
1 0 = 1
1 1 = 0
*/

void printchar(char c, char *text) {
	printf("%s = '%c' (0x%x)\n", text, c, c);
	return;
}

int main() {
	char clear_text, key, cipher_text, new_text;

	clear_text = CLEAR_TEXT;
	printchar(CLEAR_TEXT, "clear_text");

	key = KEY;
	printchar(key, "key");

	cipher_text = clear_text ^ key; /* XOR operation*/
	printchar(cipher_text, "cipher_text");

	new_text = cipher_text ^ key;
	printchar(new_text, "new_text");

	return 0;
}
