// File:        parser.h
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     01 Aug 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#pragma once

#include <string>
#include <vector>

#include "as/types.h"

namespace as {

class Parser {
private:
    // impl members
    std::vector<usize> words_;

public:
    // ctors
    Parser(const std::vector<std::vector<std::string>> &lexemes);

    // dtor
    ~Parser();

    // accessors
    const std::vector<usize> words() const;

private:
    // helpers
    void parse(const std::vector<std::vector<std::string>> &lexemes);
};

} // namespace as
