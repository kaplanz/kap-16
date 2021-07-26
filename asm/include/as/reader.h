// File:        reader.h
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     25 Jul 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#pragma once

#include <fstream>
#include <vector>

namespace as {

class Reader {
public:
    // const members
    const std::string file;

private:
    // impl members
    std::ifstream ifs;
    std::vector<std::string> lines_;

public:
    // ctors
    Reader(const std::string &file);

    // dtor
    ~Reader();

    // accessors
    const std::vector<std::string> lines() const;

private:
    // helpers
    void readlines();
};

} // namespace as
