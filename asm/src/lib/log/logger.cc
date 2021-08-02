// File:        logger.cc
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     01 Aug 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#include "log/logger.h"

#include <cstdlib>
#include <filesystem>
#include <iostream>
#include <ostream>
#include <string>

namespace log {

// ctors
Logger::Logger(char **argv) : arg0(argv[0]), os(std::cerr), level(Level::Info) {}

// dtor
Logger::~Logger() {}

// methods
void Logger::debug(const std::string &msg) const {
    this->log(msg, Level::Debug);
}

void Logger::info(const std::string &msg) const {
    this->log(msg, Level::Info);
}

void Logger::warning(const std::string &msg) const {
    this->log(msg, Level::Warning);
}

void Logger::error(const std::string &msg, int code) const {
    this->log(msg, Level::Error);
    if (code)
        std::exit(code);
}

// helpers
void Logger::log(const std::string &msg, Level level) const {
    if (level < this->level)
        return;
    this->os << this->prefix(level) << msg << std::endl;
}

const std::string Logger::filename() const {
    return std::filesystem::path(this->arg0).filename();
}

const std::string Logger::prefix(Level level) const {
    std::string p;
    switch (level) {
        case Level::Debug:
            p += "\033[1mdebug: \033[0m";
            break;
        case Level::Info:
            p += "\033[1;36minfo: \033[0m";
            break;
        case Level::Warning:
            p += "\033[1;33mwarning: \033[0m";
            break;
        case Level::Error:
            p += "\033[1;31merror: \033[0m";
            break;
    }
    return p;
}

} // namespace log
