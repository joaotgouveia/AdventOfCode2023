#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

int main() {
    std::ifstream input("day05/day05.in");
    std::vector<std::string> rules;
    auto comparator = [&rules](const std::string& a, const std::string& b) {
        const std::string target_rule = a + "|" + b;
        const std::string opposite_rule = b + "|" + a;
        for (const auto& rule : rules) {
            if (rule == target_rule) {
                return true;
            }
            if (rule == opposite_rule) {
                return false;
            }
        }
        return true;
    };

    for (std::string line; input.peek() != '\n' && std::getline(input, line);) {
        rules.push_back(line);
    }

    int sum = 0;
    input.get(); // Consume trailling newline
    for (std::string line; std::getline(input, line);) {
        std::vector<std::string> pages;
        std::stringstream stream(line + ',');
        for (std::string page; std::getline(stream, page, ',');) {
            pages.push_back(page);
        }
        if (!std::is_sorted(pages.begin(), pages.end(), comparator)) {
            std::sort(pages.begin(), pages.end(), comparator);
            sum += std::stoi(pages[(int)std::floor(pages.size() / 2)]);
        }
    }
    std::cout << sum << "\n";
}
