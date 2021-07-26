// File:        reader.cc
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     25 Jul 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#include "as/reader.h"

#include <fstream>
#include <vector>

namespace as {

// ctors
Reader::Reader(const std::string &file) : file(file) {
    // Open file
    this->ifs.exceptions(std::ifstream::failbit); // enable exceptions...
    this->ifs.open(file);                         // ...while opening the file...
    this->ifs.exceptions(std::ifstream::goodbit); // ... then disable exceptions
    // Read lines from file
    this->readlines();
}

// dtor
Reader::~Reader() {}

// accessors
const std::vector<std::string> Reader::lines() const {
    return this->lines_;
}

// helpers
void Reader::readlines() {
    for (std::string line; std::getline(this->ifs, line);)
        this->lines_.push_back(line);
}

} // namespace as
