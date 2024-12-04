#include <cstdlib>
#include <fstream>
#include <iostream>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

namespace {
bool is_safe(const std::vector<int>& report) {
    bool safe = true;
    const bool increasing = report[0] - report[1] < 0;

    for (size_t i = 0; i < report.size() - 1 && safe; ++i) {
        const int distance = report[i] - report[i + 1];
        safe = std::abs(distance) > 0 && std::abs(distance) < 4 &&
               (distance < 0) == increasing;
    }
    return safe;
}
} // namespace

int main() {
    std::ifstream input("day02/day02.in");
    std::vector<std::vector<int>> reports;

    for (std::string line; std::getline(input, line);) {
        std::vector<int> report;
        std::stringstream stream(line + ' ');
        for (std::string data; std::getline(stream, data, ' ');) {
            report.push_back(std::stoi(data));
        }
        reports.push_back(report);
    }

    auto eval = [](int a, const std::vector<int>& b) {
        return is_safe(b) ? ++a : a;
    };
    const int safe = std::accumulate(reports.begin(), reports.end(), 0, eval);
    std::cout << safe << "\n";
}
