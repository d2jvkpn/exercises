# include <iostream>
# include <cstring>

using namespace std;

int main() {
	char a[100] = "apple";
	char b[100];

	cout << "==> a: " << a << endl;
	cout << "strlen(a): " << strlen(a) << ", strlen(b): " << strlen(b) << ".\n";
	cout << "size of a: " << sizeof(a) << ", length of a: " << sizeof(a)/sizeof(char) << ".\n";
	cout << "==> strcmp(a, b): " << strcmp(a, b) << ".\n";

	cout << "==> b: " << b << endl;
	cout << "strlen(b): " << strlen(b) << ".\n";

	strcpy(b, a);
	cout << "==> b: " << b << endl;
	cout << "strlen(b): " << strlen(b) << ".\n";

	cout << "==> strcmp(a, b): " << strcmp(a, b) << ".\n";

	char web[] = "www.";
	char domain[] = "codingminutes.com";
	cout << strcat(web, domain) << endl;

	strcpy(b, strcat(web, "."));
	cout << strcmp(a, b) << endl;

	return 0;
}
