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
        bool domul = true;
        const std::regex regex(R"(mul\((\d+?),(\d+?)\)|don't\(\)|do\(\))",
                               std::regex_constants::optimize);

        while (std::getline(input, line).good()) {
            const auto end = std::sregex_iterator();
            for (auto i = std::sregex_iterator(line.begin(), line.end(), regex);
                 i != end; ++i) {
                const std::smatch& inst = *i;
                if (inst[0] == "do()") {
                    domul = true;
                } else if (inst[0] == "don't()") {
                    domul = false;
                } else if (domul) {
                    result += std::stoi(inst[1]) * std::stoi(inst[2]);
                }
            }
        }
        std::cout << result << "\n";
    } catch (const std::regex_error& e) {
        std::cout << "Regex failed: " << e.what() << "\n";
    }
}
