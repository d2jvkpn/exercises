# include <iostream>
#include <algorithm> 
#include <cctype>
#include <locale>

using namespace std;

// trim from both ends (in place)
static inline int trim(string &s) {
	// static inline void ltrim(string &s)
    s.erase(s.begin(), find_if(s.begin(), s.end(), [](int ch) {
        return !isspace(ch);
    }));

	// static inline void rtrim(string &s)
    s.erase(find_if(s.rbegin(), s.rend(), [](int ch) {
        return !isspace(ch);
    }).base(), s.end());

	return s.length();
}
