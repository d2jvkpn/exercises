#include <iostream>
#include <filesystem>
#include <fstream>
#include <cassert>
#include <stdio.h>

using namespace std;
using namespace filesystem;

void helloToFile(path p) {
	// fstream fs;
	ofstream fs;

	if (!fs) {
		cerr << "!!! bad file: " << p << endl;
		return;
	}

	fs.open(p, fstream::in | fstream::out | fstream::app);

	fs << "Hello, world\n";

	fs.close();
}

void print_directory(path p) {
	try {
		if (is_directory(p)) {
			cout << "==> contents in directory " << p << ":\n";
		}

		for (const directory_entry& x : directory_iterator{p}) {
			cout << "-- " << x.path() << endl;
			const path& f = x;

			if (f.extension() == ".exe") {
				cout << f.stem() << " is a Windows executable\n";
			} else {
				string n = f.extension().string();
				if (n == ".cpp" || n == ".C" || n == ".cxx") {
					cout << f.stem() << " is a C++ source file\n"; // "a01" is a C++ source file
				}
			}
		}
	} catch (const filesystem_error& e) {
		cerr << e.what() << '\n';
	}
}

/*
exists(path p)
copy(path p1, path p2)
copy(path p1, path p2, filesystem_error e)
e = copy(path p1, path p2)
e = create_directory(path p)
e = create_directories(path p)
p = current_path()
s = file_size(path p)
b = remove(path p)
is_block_file(p)
is_character_file(p)
is_directory(p)
is_file(p)
is_empty(f)
is_fifo(f)
is_regular_file(f)
is_socket(f)
is_symlink(f)
status_known
*/

int main(int argc, char* argv[]) {
	// cout << "Hello, world!\n";

	if (argc < 2) {
		cerr << "!!! arguments expected\n";
		return 1;
	}

	// path f = "./c1104_file.cpp";
	path fh {argv[1]};
	// assert(exists(fh));

	if (!exists(fh)) {
		cerr << "!!! file not exists: " << argv[1] << endl;
	} else if (is_regular_file(fh)) {
		cout << "==> " << fh << " is a file, it's size is " << file_size(fh) << ".\n";
	} else if ((is_directory(fh))) {
		cout << "==> " << fh << " is a dirtory.\n";
	}

	helloToFile("target/hello.txt");

	print_directory("target");

	return 0;
}
