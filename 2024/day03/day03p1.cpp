#include <cstdlib>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>

int main() {
    try {
        std::ifstream input("day03/day03.in");
        std::string line;
        int result = 0;
        const std::regex regex(R"(mul\((\d+?),(\d+?)\))",
                               std::regex_constants::optimize);

        while (std::getline(input, line).good()) {
            const auto end = std::sregex_iterator();
            for (auto i = std::sregex_iterator(line.begin(), line.end(), regex);
                 i != end; ++i) {
                const std::smatch& mul = *i;
                result += std::stoi(mul[1]) * std::stoi(mul[2]);
            }
        }
        std::cout << result << "\n";
    } catch (const std::regex_error& e) {
        std::cout << "Regex failed: " << e.what() << "\n";
    }
}
