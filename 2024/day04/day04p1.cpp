#include <algorithm>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

namespace {
bool xmas(const std::vector<std::string>& puzzle, int l, int c, int h, int v) {
    std::string word;
    const int linesize = std::min(l + 4, static_cast<int>(puzzle.size()));
    const int colsize = std::min(c + 4, static_cast<int>(puzzle[0].length()));

    for (int line = l, col = c; line < linesize && line > -1 && col < colsize;
         line += v, col += h) {
        word += puzzle[line][col];
    }
    return word == "XMAS" || word == "SAMX";
}
} // namespace

int main() {
    std::ifstream input("day04/day04.in");
    std::vector<std::string> puzzle;

    for (std::string line; std::getline(input, line);) {
        puzzle.push_back(std::move(line));
    }

    int count = 0;
    for (int l = 0; l < puzzle.size(); ++l) {
        for (int c = 0; c < puzzle[0].length(); ++c) {
            // Forward and Backward
            if (xmas(puzzle, l, c, 1, 0)) {
                ++count;
            }
            // Upward and Downward
            if (xmas(puzzle, l, c, 0, 1)) {
                ++count;
            }
            // Forward + Downward and Backward + Upward diagonals
            if (xmas(puzzle, l, c, 1, 1)) {
                ++count;
            }
            // Forward + Upward and Backward + Downward diagonals
            if (xmas(puzzle, l, c, 1, -1)) {
                ++count;
            }
        }
    }
    std::cout << count << "\n";
}
