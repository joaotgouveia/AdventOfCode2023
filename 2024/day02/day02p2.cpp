#include <cstdlib>
#include <fstream>
#include <iostream>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

namespace {
bool is_safe(const std::vector<int>& report, bool removedlevel) {
    bool safe = true;
    const bool increasing = report[0] - report[1] < 0;
    std::vector<int> badlevels = {0};

    for (int i = 0; i < report.size() - 1; ++i) {
        const int distance = report[i] - report[i + 1];
        safe = std::abs(distance) > 0 && std::abs(distance) < 4 &&
               (distance < 0) == increasing;
        if (!safe) {
            badlevels.push_back(i);
            badlevels.push_back(i + 1);
            break;
        }
    }

    if (removedlevel || safe) {
        return safe;
    }

    for (int i = 0; i < badlevels.size() && !safe; ++i) {
        std::vector<int> cleanreport(report);
        cleanreport.erase(cleanreport.begin() + badlevels[i]);
        safe = is_safe(cleanreport, true);
    }
    return safe;
}
} // namespace

int main() {
    std::ifstream input("day02/day02.in");
    std::vector<std::vector<int>> reports;
    std::string line;

    while (std::getline(input, line).good()) {
        std::string data;
        std::vector<int> report;
        std::stringstream stream(line + ' ');
        while (std::getline(stream, data, ' ').good()) {
            report.push_back(std::stoi(data));
        }
        reports.push_back(report);
    }

    auto eval = [](int a, const std::vector<int>& b) {
        return is_safe(b, false) ? ++a : a;
    };
    const int safe = std::accumulate(reports.begin(), reports.end(), 0, eval);
    std::cout << safe << "\n";
}
