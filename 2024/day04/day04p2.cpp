#include <algorithm>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

namespace {
bool mas(const std::vector<std::string>& puzzle, int l, int c, int h) {
    std::string word;
    const int linesize = std::min(l + 3, static_cast<int>(puzzle.size()));
    const int colsize = std::min(c + 3, static_cast<int>(puzzle[0].length()));

    for (int line = l, col = c; line < linesize && line > -1 && col < colsize;
         ++line, col += h) {
        word += puzzle[line][col];
    }
    return word == "MAS" || word == "SAM";
}
bool x_mas(const std::vector<std::string>& puzzle, int l, int c) {
    return mas(puzzle, l, c, 1) && mas(puzzle, l, c + 2, -1);
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
            if (x_mas(puzzle, l, c)) {
                ++count;
            }
        }
    }
    std::cout << count << "\n";
}
