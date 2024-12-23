#include <cstdlib>
#include <fstream>
#include <iostream>
#include <set>
#include <string>

int main() {
    std::ifstream input("day01/day01.in");
    std::multiset<int> left;
    std::multiset<int> right;

    while (input.good()) {
        std::string sleft;
        input >> sleft;
        left.insert(std::stoi(sleft));

        std::string sright;
        input >> sright;
        right.insert(std::stoi(sright));
    }

    int distance = 0;
    for (auto l = left.begin(), r = right.begin();
         l != left.end() && r != right.end(); ++l, ++r) {
        distance += std::abs(*l - *r);
    }
    std::cout << distance << "\n";
}
