#include <cstdlib>
#include <fstream>
#include <iostream>
#include <numeric>
#include <set>
#include <string>

int main() {
    std::ifstream input("day01/day01.in");
    std::multiset<size_t> left;
    std::multiset<size_t> right;

    while (input.good()) {
        std::string sleft;
        input >> sleft;
        left.insert(std::stoi(sleft));

        std::string sright;
        input >> sright;
        right.insert(std::stoi(sright));
    }

    auto score = [right](int a, int b) { return a + (b * right.count(b)); };
    const int similarity = std::accumulate(left.begin(), left.end(), 0, score);
    std::cout << similarity << "\n";
}
