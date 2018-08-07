/*
Copyright (c) 2018 Pierre Marijon <pierre.marijon@inria.fr>
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/


/* standard include */
#include <fstream>
#include <sstream>
#include <limits>
#include <vector>
#include <string>
#include <unordered_map>
#include <chrono>
#include <iostream>

#include "csv.h"

// type definition
using name_len = std::pair<std::string, std::uint64_t>;
using interval = std::pair<std::uint64_t, std::uint64_t>;
using interval_vector = std::vector<interval>;

struct Read2MappingHashEq
{
    bool operator()(const name_len& x, const name_len& y) const {
        return x.first == y.first;
    }
    std::size_t operator()(const name_len& k) const
    {
	return std::hash<std::string>()(k.first);
    }
};

using read2mapping_type = std::unordered_map<name_len, std::vector<interval>, Read2MappingHashEq, Read2MappingHashEq>;


struct alignment_span {
  std::string name;
  size_t beg, end, len;
};

using alignment = std::pair<alignment_span, alignment_span>;

template< typename T >
inline T absdiff( const T& lhs, const T& rhs ) {
  return lhs>rhs ? lhs-rhs : rhs-lhs;
}

void file(const std::string& filename, read2mapping_type* read2mapping);

void file(const std::string& filename, read2mapping_type* read2mapping)
{
    std::uint64_t switch_val;
    
    io::CSVReader<12, io::trim_chars<' '>, io::no_quote_escape<'\t'> > in(filename);
    
    std::string name_a, name_b, strand;
    std::uint64_t len_a, beg_a, end_a, len_b, beg_b, end_b, nb_match, nb_base, qual;
    
    while(in.read_row(name_a, len_a, beg_a, end_a, strand, name_b, len_b, beg_b, end_b, nb_match, nb_base, qual))
    {
        if(beg_a > end_a)
        {
            switch_val = beg_a;
            beg_a = end_a;
            end_a = switch_val;
        }

        if(beg_b > end_b)
        {
            switch_val = beg_b;
            beg_b = end_b;
            end_b = switch_val;
        }

        if(read2mapping->count(std::make_pair(name_a, len_a)) == 0)
        {
            read2mapping->emplace(std::make_pair(name_a, len_a), std::vector<interval>());
        }
        read2mapping->at(std::make_pair(name_a, len_a)).push_back(std::make_pair(beg_a, end_a));

        if(read2mapping->count(std::make_pair(name_b, len_b)) == 0)
        {
            read2mapping->emplace(std::make_pair(name_b, len_b), std::vector<interval>());
        }
        read2mapping->at(std::make_pair(name_b, len_b)).push_back(std::make_pair(beg_b, end_b));
    }

}

int main(int argc, char* argv[])
{
    std::string filename = argv[argc-1];

    int iteration;

    for(std::string line; std::getline(std::cin, line), iteration = stoi(line);)
    {
	auto start = std::chrono::high_resolution_clock::now();
	
	for(int i = 0; i != iteration; i++)
	{
	    read2mapping_type a;

	    file(filename, &a);
	}

	auto finish = std::chrono::high_resolution_clock::now();

	std::cout<<std::chrono::duration_cast<std::chrono::nanoseconds>(finish-start).count()<<std::endl;
    }

    return 0;
}
