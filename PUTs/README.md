# Programs under test

To perform effective fuzzing strategies we will use three programs: a simple `nothing` program that allows us to test the performance bottlenecks of our fuzzer, a reader that reads from inputs and performs a few actions if some characters are detected and a parser that reads and parses from inputs. 
