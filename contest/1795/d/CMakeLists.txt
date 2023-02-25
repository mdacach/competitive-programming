cmake_minimum_required(VERSION 3.24)
project(d)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "-std=c++17 -g -O2 -Wall -Wshadow -Wextra \
		-Wformat=2 -fsanitize=address -fsanitize=undefined \
		-fno-sanitize-recover -Wfatal-errors \
		-Wno-unused-variable")

add_executable(d
        d.cpp)
