# include <vector>
# include <tuple>
# include <stack>
# include <queue>
# include <list>
# include <set>
# include <unordered_map>

using namespace std;

namespace ns01 {
	template<typename T>
	using Vector = vector<T>;

	template <typename... Types>
	using Tuple = tuple<Types...>;

	template<typename T>
	using Stack = stack<T>;

	template<typename T>
	using Queue = queue<T>;

	template<typename T>
	using Set = set<T>;

	template<typename K, typename V>
	using UnorderedMap = unordered_map<K, V, hash<K>, equal_to<K>, allocator<pair<const K, V>>>;
}
