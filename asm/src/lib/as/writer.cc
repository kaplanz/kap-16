// File:        writer.cc
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     01 Aug 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#include "as/writer.h"

#include <fstream>
#include <string>
#include <vector>

namespace as {

// ctors
Writer::Writer(const std::string &file) : file(file) {
    // Open file
    this->ofs.exceptions(std::ofstream::failbit); // enable exceptions...
    this->ofs.open(file, std::ios_base::binary);  // ...while opening the file...
    this->ofs.exceptions(std::ofstream::goodbit); // ... then disable exceptions
}

// dtor
Writer::~Writer() {}

// methods
void Writer::write(const std::vector<usize> &words) {
    this->ofs.write((char *)words.data(), words.size() * sizeof(usize));
}

} // namespace as
