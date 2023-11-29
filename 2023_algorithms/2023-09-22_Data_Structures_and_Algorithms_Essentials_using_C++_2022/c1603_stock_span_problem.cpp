// author: ChatGPT
#include <iostream>
#include <stack>
#include <vector>

using namespace std;

// 定义一个结构体来存储股票价格和其对应的跨度
struct StockSpan {
	int price;
	int span;
	StockSpan(int p, int s) : price(p), span(s) {}
};

vector<int> calculateStockSpan(const vector<int>& prices) {
	stack<StockSpan> stockStack; // 用于维护价格和跨度信息
	vector<int> spans; // 用于存储每天的股票跨度

	for (int price : prices) {
		int span = 1;
		// 弹出所有比当天价格小的元素，累计它们的跨度
		while (!stockStack.empty() && stockStack.top().price <= price) {
			span += stockStack.top().span;
			stockStack.pop();
		}
		// 将当天价格和计算的跨度入栈
		stockStack.push(StockSpan(price, span));
		spans.push_back(span);
	}

	return spans;
}

int main() {
	vector<int> prices = {100, 80, 60, 70, 60, 75, 85};
	vector<int> spans = calculateStockSpan(prices);

	cout << "Prices: ";
	for (int price : prices) {
		cout << price << ", ";
	}
	cout << endl;

	cout << "Stock Spans: ";
	for (int span : spans) {
		cout << span << ", ";
	}
	cout << endl;

	return 0;
}
