#include <stdio.h>
#include <assert.h>

#define F fflush(stdout);

void multi(int t, int v) {
	int result;

	result = t * v;
	printf("%d x %d = %d\n", t, v, result);

	return;
}

int main() {
	// while loop
	int num;
	num = 0;

	// while (1) infinity loop
	while (num < 10) {
		printf("num: %d\n", num);
		num+=1;
	}
	
	// for loop
	int x, table;

	printf("Select multiplication table: ");
	F;
	scanf("%d", &table);
	assert((table < 13) && (table > 0));

	for (x=0; x < 13; x+=1) {
		multi(table, x);
	}
}
