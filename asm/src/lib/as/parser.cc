// File:        parser.cc
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     01 Aug 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#include "as/parser.h"

#include <vector>

#include "as/types.h"

namespace as {

// ctors
Parser::Parser(const std::vector<std::vector<std::string>> &lexemes) {
    // Parse lexemes
    this->parse(lexemes);
}

// dtor
Parser::~Parser() {}

// accessors
const std::vector<usize> Parser::words() const {
    return this->words_;
}

// helpers
void Parser::parse(const std::vector<std::vector<std::string>> &lexemes) {}

} // namespace as
