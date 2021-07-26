// File:        lexer.cc
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     22 Jul 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#include "as/lexer.h"

#include <sstream>
#include <string>
#include <vector>

namespace as {

// ctors
Lexer::Lexer(const std::vector<std::string> &lines) {
    // Tokenize lines
    this->tokenize(lines);
}

// dtor
Lexer::~Lexer() {}

// accessors
const std::vector<std::vector<std::string>> Lexer::lexemes() const {
    return this->lexemes_;
}

// helpers
void Lexer::tokenize(const std::vector<std::string> &lines) {
    // Loop through each line
    for (const auto &line : lines) {
        std::istringstream iss(line);
        std::vector<std::string> tokens;
        // Iterate over tokens
        for (std::string token; iss >> token;) {
            // Ignore comments
            if (token.find(";") != std::string::npos)
                break;
            // Save token
            tokens.push_back(token);
        }
        // Record tokenized line
        this->lexemes_.push_back(tokens);
    }
}

} // namespace as
