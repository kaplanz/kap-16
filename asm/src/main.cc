// File:        main.cc
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     19 Jul 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>

#include "as/lexer.h"
#include "as/reader.h"
#include "clip/clip.h"

int main(int argc, char **argv) {
    // Initialize parser
    clip::Parser parser(argc,
                        argv,
                        clip::App("asm")
                            .about("LANv1 assembler")
                            .author("Zakhary Kaplan <zakharykaplan@gmail.com>")
                            .version("0.1.0"));
    // Add arguments
    parser.add(clip::Arg<std::string>("input").help("input file to assemble"));
    // Add options
    parser.add(clip::Opt<std::string>("output")
                   .shortname('o')
                   .help("output file to write")
                   .value("a.out"));
    // Parse args
    parser.parse();

    // Retrieve args
    const auto &input = parser.getArg<std::string>("input");

    // Initialize reader
    std::vector<std::string> lines;
    try {
        as::Reader reader(input.value());
        lines = reader.lines();
    } catch (const std::ifstream::failure &e) {
        std::cerr << "Could not open file..." << std::endl;
    }
    // Initialize lexer
    as::Lexer lexer(lines);
    // Extract lexemes
    auto lexemes = lexer.lexemes();
}
