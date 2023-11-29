#include <iostream>
#include <cstring>
#include <climits>

using namespace std;

int main() {
	//
	char sentence[10];

	cout << "Enter sentence: ";
	cin.getline(sentence, 5);
	cout << "==> Sentence: " << strlen(sentence) << " <<EOF\n" << sentence << "\nEOF" << endl;
	cin.clear();
	cin.ignore(INT_MAX, '\n');

	//
	char paragraph[140];

	cout << "Enter paragraph:" << endl;
	cin.getline(paragraph, 140, '.');
	cout << "==> Paragraph: " << strlen(paragraph) << " <<EOF\n" << paragraph << "\nEOF" << endl;

	return 0;
}
