#include <cctype>
#include <fstream>
#include <optional>
#include <span>
#include <string>
#include <tuple>
#include <vector>

class Location {
    size_t m_x;
    size_t m_y;

public:
    Location(size_t x, size_t y) : m_x(x), m_y(y) {}
    Location(const Location &other) : m_x(other.m_x), m_y(other.m_y) {}

    const size_t &x = m_x;
    const size_t &y = m_y;

    Location &operator=(const Location &other) {
        if (this == &other) {
            return *this;
        }
        m_x = other.m_x;
        m_y = other.m_y;
        return *this;
    }

    bool operator==(const Location &other) const {
        return m_x == other.m_x && m_y == other.m_y;
    }

    std::optional<Location> north() const {
        if (m_y == 0) {
            return std::nullopt;
        }
        return Location(m_x, m_y - 1);
    }

    std::optional<Location> south() const {
        return Location(m_x, m_y + 1);
    }

    std::optional<Location> west() const {
        if (m_x == 0) {
            return std::nullopt;
        }
        return Location(m_x - 1, m_y);
    }

    std::optional<Location> east() const {
        return Location(m_x + 1, m_y);
    }
};

enum Direction { NORTH, SOUTH, WEST, EAST };

class Map {
    const std::vector<std::span<const char>> m_lines;
    const Location m_start;
    const size_t m_width;
    const size_t m_height;

    // private constructor, Map will be parsed from input
    Map(std::vector<std::span<const char>> lines, size_t width, Location start)
        : m_lines(lines), m_width(width), m_start(start),
          m_height(lines.size()) {}

    // symbols on the tiles connect them to each other
    // | connects up and down
    // - connects left and right
    // L connects right and up
    // J connects left and up
    // 7 connects left and down
    // F connects right and down
    //
    // so given a location, look at the current tile and the direction
    // we came from and determine the next tile that can be visited
    // without going back to the previous tile. Return the location
    // and the direction we are coming from when entering that tile.
    // If there is no such tile, return std::nullopt.
    inline std::optional<std::tuple<Location, Direction>>
    next_location(const Location &current, const Direction &coming_from) {
        auto tile = get_tile(current);
        std::optional<std::tuple<std::optional<Location>, Direction>> next =
            std::nullopt;
        switch (tile) {
        case '|':
            if (coming_from == NORTH) {
                next = std::make_tuple(current.south(), NORTH);
            } else if (coming_from == SOUTH) {
                next = std::make_tuple(current.north(), SOUTH);
            }
            break;
        case '-':
            if (coming_from == WEST) {
                next = std::make_tuple(current.east(), WEST);
            } else if (coming_from == EAST) {
                next = std::make_tuple(current.west(), EAST);
            }
            break;
        case 'L':
            if (coming_from == NORTH) {
                next = std::make_tuple(current.east(), WEST);
            } else if (coming_from == EAST) {
                next = std::make_tuple(current.north(), SOUTH);
            }
            break;
        case 'J':
            if (coming_from == NORTH) {
                next = std::make_tuple(current.west(), EAST);
            } else if (coming_from == WEST) {
                next = std::make_tuple(current.north(), SOUTH);
            }
            break;
        case '7':
            if (coming_from == SOUTH) {
                next = std::make_tuple(current.west(), EAST);
            } else if (coming_from == WEST) {
                next = std::make_tuple(current.south(), NORTH);
            }
            break;
        case 'F':
            if (coming_from == SOUTH) {
                next = std::make_tuple(current.east(), WEST);
            } else if (coming_from == EAST) {
                next = std::make_tuple(current.south(), NORTH);
            }
            break;
        }

        // check wether next is beyond map borders, return None if it is
        if (!next.has_value()) {
            return std::nullopt;
        }
        auto [next_location, direction] = next.value();

        if (!next_location.has_value()) {
            return std::nullopt;
        }

        if (next_location.value().x >= m_width ||
                next_location.value().y >= m_height) {
            return std::nullopt;
        }

        // check wether next is a tile not connected to the direction we are
        // coming from, return None, if it is (S connects to everything)
        switch (direction) {
        case NORTH:
            if (memchr("S|LJ", get_tile(next_location.value()), 4) == nullptr) {
                return std::nullopt;
            }
            break;
        case SOUTH:
            if (memchr("S|7F", get_tile(next_location.value()), 4) == nullptr) {
                return std::nullopt;
            }
            break;
        case WEST:
            if (memchr("S-7J", get_tile(next_location.value()), 4) == nullptr) {
                return std::nullopt;
            }
            break;
        case EAST:
            if (memchr("S-LF", get_tile(next_location.value()), 4) == nullptr) {
                return std::nullopt;
            }
            break;
        }

        return std::make_tuple(next_location.value(), direction);
    }

    inline const char &get_tile(const Location &next_location) {
        return m_lines[next_location.y][next_location.x];
    }

    std::vector<std::tuple<Location, Direction>> connected_to() {
        std::vector<std::tuple<Location, Direction>> connected;
        auto north = m_start.north();
        if (north.has_value()) {
            if (memchr("|F7", get_tile(north.value()), 4) != nullptr) {
                connected.push_back(std::make_tuple(north.value(), SOUTH));
            }
        }
        auto south = m_start.south();
        if (south.has_value()) {
            if (memchr("|LJ", get_tile(south.value()), 4) != nullptr) {
                connected.push_back(std::make_tuple(south.value(), NORTH));
            }
        }
        auto west = m_start.west();
        if (west.has_value()) {
            if (memchr("-LF", get_tile(west.value()), 4) != nullptr) {
                connected.push_back(std::make_tuple(west.value(), EAST));
            }
        }
        auto east = m_start.east();
        if (east.has_value()) {
            if (memchr("-7J", get_tile(east.value()), 4) != nullptr) {
                connected.push_back(std::make_tuple(east.value(), WEST));
            }
        }
        return connected;
    }

public:
    static Map parse(const std::span<const char> &input) {
        // It is really hard in c++ to structure the input
        // without copying it. Let's ask Copilot for help.
        // Create a vector of spans for each line.
        auto start_of_current_line = 0;
        size_t running_width = SIZE_T_MAX;
        std::vector<std::span<const char>> lines;
        std::optional<Location> start_location = std::nullopt;
        for (size_t i = 0; i < input.size(); i++) {
            if (input[i] == '\n') {
                auto line = input.subspan(start_of_current_line,
                                          i - start_of_current_line);

                if (!start_location.has_value()) {
                    look_for_startposition(line, start_location, lines);
                }

                lines.push_back(line);
                start_of_current_line = i + 1;
                running_width = std::min(
                                    running_width, line.size()); // don't count the newline
            }
        }
        if (start_of_current_line < input.size()) {
            auto line = input.subspan(start_of_current_line,
                                      input.size() - start_of_current_line - 1);
            if (!start_location.has_value()) {
                look_for_startposition(line, start_location, lines);
            }
            lines.push_back(line);
        }
        return Map(lines, running_width,
                   start_location.value_or(Location(0, 0)));
    }

    static void look_for_startposition(
        std::span<const char> &line,
        std::optional<Location> &start_location,
        std::vector<std::span<const char>> &lines) {
        char *start_position = (char *) memchr(line.data(), 'S', line.size());
        if (start_position != nullptr) {
            start_location = Location(start_position - line.data(),
                                      lines.size());
        }
    }

    std::optional<std::vector<Location>> find_loop() {
        std::vector<Location> loop;
        auto connected = connected_to();
        for (auto [location, direction] : connected) {
            auto next = next_location(location, direction);
            std::vector<Location> path;
            path.push_back(m_start);
            path.push_back(location);
            while (next.has_value()) {
                auto [next_tile, next_direction] = next.value();
                if (next_tile == m_start) {
                    return path;
                }
                path.push_back(next_tile);
                next = next_location(next_tile, next_direction);
            }
            printf("Pathlen: %d\n", path.size());
        }
        return std::nullopt;
    }
};

size_t p1(const std::span<const char> &input) {
    Map map = Map::parse(input);
    auto loop = map.find_loop();
    if (loop.has_value()) {
        return loop.value().size() / 2;
    }
    return 0;
}

extern "C" {
    size_t run_p1(const char *input, size_t input_len) {
        auto span = std::span(input, input_len);
        return p1(span);
    }
}
