// File:        instr.h
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     22 Jul 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#pragma once

#include <iosfwd>
#include <vector>

#include "as/types.h"

namespace as {

class Instruction {
protected:
    // impl members
    const usize word;

public:
    // ctors
    Instruction(std::vector<std::string> tokens) = 0;
    // dtor
    virtual ~Instruction();
};

} // namespace as
