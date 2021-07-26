// File:        lexer.h
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     22 Jul 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#pragma once

#include <fstream>
#include <vector>

namespace as {

class Lexer {
private:
    // impl members
    std::vector<std::vector<std::string>> lexemes_;

public:
    // ctors
    Lexer(const std::vector<std::string> &lines);

    // dtor
    ~Lexer();

    // accessors
    const std::vector<std::vector<std::string>> lexemes() const;

private:
    // helpers
    void tokenize(const std::vector<std::string> &lines);
};

} // namespace as
