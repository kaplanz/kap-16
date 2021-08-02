// File:        main.cc
// Author:      Zakhary Kaplan <https://zakharykaplan.ca>
// Created:     19 Jul 2021
// Version:     0.1.0
// SPDX-License-Identifier: MIT

#include <clip/clip.h>

#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>

#include "as/lexer.h"
#include "as/parser.h"
#include "as/reader.h"
#include "as/writer.h"
#include "log/logger.h"

int main(int argc, char **argv) {
    // Initialize logger
    log::Logger logger(argv);

    // Initialize parser
    clip::Parser clip(argc,
                      argv,
                      clip::App("asm")
                          .about("LANv1 assembler")
                          .author("Zakhary Kaplan <zakharykaplan@gmail.com>")
                          .version("0.1.0"));
    // Add arguments
    clip.add(clip::Arg<std::string>("input").help("input file to assemble"));
    // Add options
    clip.add(clip::Opt<std::string>("output")
                 .shortname('o')
                 .help("output file to write")
                 .value("a.out"));
    // Parse args
    clip.parse();

    // Retrieve args
    const auto &input = clip.getArg<std::string>("input");
    const auto &output = clip.getOpt<std::string>("output");

    // Initialize reader
    std::vector<std::string> lines;
    try {
        as::Reader reader(input.value());
        lines = reader.lines();
    } catch (const std::ifstream::failure &e) { logger.error("could not open file.", 1); }
    // Initialize lexer
    as::Lexer lexer(lines);
    // Extract lexemes
    auto lexemes = lexer.lexemes();
    // Parse lexemes
    as::Parser parser(lexemes);
    // Extract instructions
    auto words = parser.words();
    // Initialize writer
    try {
        as::Writer writer(output.value());
        // Write instructions
        writer.write(words);
    } catch (const std::ofstream::failure &e) { logger.error("could not open file.", 1); }
}
