#include <stdio.h>

float triangle(float base, float height) {
	float area;
	area = (base * height) / 2.0;

	return area;
}

int main() {
	float b, h, a;

	printf("Base: ");
	fflush(stdout);
	scanf("%f", &b);

	printf("Height: ");
	fflush(stdout);
	scanf("%f", &h);

	a = triangle(b, h);
	printf("The area is: %f\n", a);

	return 0;
}
