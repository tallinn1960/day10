#include <algorithm>
#include <cassert>
#include <cctype>
#include <chrono>
#include <fstream>
#include <iterator>
#include <limits>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <span>
#include <stack>
#include <string>
#include <vector>

using std::chrono::high_resolution_clock;
using std::chrono::milliseconds;

std::vector<std::string> parse(std::span<const unsigned char> &input);

uint64_t solve(std::span<const unsigned char> input) {
    // assume that the input is text separated by newlines
    // turn this into a vector of strings
    std::vector<std::string> graph = parse(input);

    int ans = 0;

    int sr, sc;
    for (int r = 0; r < graph.size(); r++) {
        for (int c = 0; c < graph[r].size(); c++) {
            if (graph[r][c] == 'S') {
                sr = r;
                sc = c;
                goto out;
            }
        }
    }
out:

    std::set<std::pair<int, int>> loop; // coordinates in the loop
    loop.insert(std::make_pair(sr, sc));

    std::queue<std::pair<int, int>> q; // coordinates to visit
    q.push(std::make_pair(sr, sc));

    std::set<char> possibleS;
    possibleS.insert('|');
    possibleS.insert('-');
    possibleS.insert('J');
    possibleS.insert('L');
    possibleS.insert('7');
    possibleS.insert('F');

    while (!q.empty()) {
        std::pair<int, int> currPos = q.front();
        q.pop();

        int r = currPos.first;
        int c = currPos.second;

        char currCh = graph[r][c];

        // check up
        if (r > 0 &&
            (currCh == 'S' || currCh == '|' || currCh == 'J' ||
             currCh == 'L') &&
            (graph[r - 1][c] == '|' || graph[r - 1][c] == '7' ||
             graph[r - 1][c] == 'F') &&
            loop.find(std::make_pair(r - 1, c)) == loop.end()) {
            loop.insert(std::make_pair(r - 1, c));
            q.push(std::make_pair(r - 1, c));
            if (currCh == 'S') {
                std::set<char> copySet = possibleS;
                for (auto it = copySet.begin(); it != copySet.end(); ++it) {
                    if (*it != '|' && *it != 'J' && *it != 'L') {
                        possibleS.erase(*it);
                    }
                }
            }
        }

        // check down
        if (r < graph.size() &&
            (currCh == 'S' || currCh == '|' || currCh == '7' ||
             currCh == 'F') &&
            (graph[r + 1][c] == 'J' || graph[r + 1][c] == '|' ||
             graph[r + 1][c] == 'L') &&
            loop.find(std::make_pair(r + 1, c)) == loop.end()) {
            loop.insert(std::make_pair(r + 1, c));
            q.push(std::make_pair(r + 1, c));
            if (currCh == 'S') {
                std::set<char> copySet = possibleS;
                for (auto it = copySet.begin(); it != copySet.end(); ++it) {
                    if (*it != '|' && *it != '7' && *it != 'F') {
                        possibleS.erase(*it);
                    }
                }
            }
        }

        // check left
        if (c > 0 &&
            (currCh == 'S' || currCh == '-' || currCh == 'J' ||
             currCh == '7') &&
            (graph[r][c - 1] == '-' || graph[r][c - 1] == 'L' ||
             graph[r][c - 1] == 'F') &&
            loop.find(std::make_pair(r, c - 1)) == loop.end()) {
            loop.insert(std::make_pair(r, c - 1));
            q.push(std::make_pair(r, c - 1));
            if (currCh == 'S') {
                std::set<char> copySet = possibleS;
                for (auto it = copySet.begin(); it != copySet.end(); ++it) {
                    if (*it != '-' && *it != 'J' && *it != '7') {
                        possibleS.erase(*it);
                    }
                }
            }
        }

        // check right
        if (c < graph[r].size() &&
            (currCh == 'S' || currCh == '-' || currCh == 'L' ||
             currCh == 'F') &&
            (graph[r][c + 1] == '-' || graph[r][c + 1] == 'J' ||
             graph[r][c + 1] == '7') &&
            loop.find(std::make_pair(r, c + 1)) == loop.end()) {
            loop.insert(std::make_pair(r, c + 1));
            q.push(std::make_pair(r, c + 1));
            if (currCh == 'S') {
                std::set<char> copySet = possibleS;
                for (auto it = copySet.begin(); it != copySet.end(); ++it) {
                    if (*it != '-' && *it != 'L' && *it != 'F') {
                        possibleS.erase(*it);
                    }
                }
            }
        }
    }

    assert(possibleS.size() == 1);
    char S = *possibleS.begin();

    // replace S with the right character
    graph[sr] = graph[sr].substr(0, sc) + S +
                graph[sr].substr(sc + 1, graph[sr].size() - sc);

    std::vector<std::vector<char>> newGraph;
    for (int r = 0; r < graph.size(); r++) {
        std::vector<char> currRow;
        for (int c = 0; c < graph[r].size(); c++) {
            if (loop.find(std::make_pair(r, c)) == loop.end())
                currRow.push_back('.');
            else
                currRow.push_back(graph[r][c]);
        }
        newGraph.push_back(currRow);
    }

    std::set<std::pair<int, int>> outside;
    for (int r = 0; r < newGraph.size(); r++) {
        bool within = false;
        bool up = false;
        for (int c = 0; c < newGraph[r].size(); c++) {
            char ch = newGraph[r][c];
            if (ch == '|')
                within = !within;
            else if (ch == 'F' || ch == 'L')
                up = (ch == 'L');
            else if (ch == '7' || ch == 'J') {
                if (up) {
                    if (ch != 'J')
                        within = !within;
                } else {
                    if (ch != '7')
                        within = !within;
                }
                up = false;
            }

            if (!within)
                outside.insert(std::make_pair(r, c));
        }
    }
    std::set<std::pair<int, int>> outsideUnionLoop;
    std::set_union(outside.begin(), outside.end(), loop.begin(), loop.end(),
                   std::inserter(outsideUnionLoop, outsideUnionLoop.begin()));

    ans = newGraph.size() * newGraph[0].size() - outsideUnionLoop.size();
    return ans;
}

std::vector<std::string> parse(std::span<const unsigned char> &input) {
    std::vector<std::string> graph;
    std::string currLine = "";
    for (int i = 0; i < input.size(); i++) {
        if (input[i] == '\n') {
            graph.push_back(currLine);
            currLine = "";
        } else {
            currLine += input[i];
        }
    }
    graph.push_back(currLine);
    return graph;
}
extern "C" uint64_t solve_ffi(const unsigned char *bytes, size_t length) {
    std::span<const unsigned char> input(bytes, length);
    return solve(input);
}
