// File:        logger.h
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     01 Aug 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#pragma once

#include <ostream>
#include <string>

namespace log {

enum class Level {
    Debug,
    Info,
    Warning,
    Error,
};

class Logger {
public:
    // const members
    const std::string arg0;
    std::ostream &os;

    // pub members
    Level level;

public:
    // ctors
    Logger(char **argv);

    // dtor
    ~Logger();

    // methods
    void debug(const std::string &msg) const;
    void info(const std::string &msg) const;
    void warning(const std::string &msg) const;
    void error(const std::string &msg, int code = 0) const;

private:
    // helpers
    void log(const std::string &msg, Level level) const;
    const std::string filename() const;
    const std::string prefix(Level level) const;
};

} // namespace log
