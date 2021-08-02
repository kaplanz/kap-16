// File:        writer.h
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     01 Aug 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#pragma once

#include <fstream>
#include <string>
#include <vector>

#include "as/types.h"

namespace as {

class Writer {
public:
    // const members
    const std::string file;

private:
    // impl members
    std::ofstream ofs;

public:
    // ctors
    Writer(const std::string &file);

    // dtor
    ~Writer();

    // methods
    void write(const std::vector<usize> &words);
};

} // namespace as
