Source directory for the performance benchmark test suite.

# Languages for Comparison
The intent is to use a set of computationally intensive algorithms to compare Rust with the following language:
* Clojure
* Go (1.6)
* Groovy
* Java (8)
* JavaScript/Node
* Julia
* PHP (7)
* Python (3.x)
* R 
* Scala

# Performance Benchmark Scenarios
* PB-001: Prime Number Generator
* (others to be added)


# Reporting Metrics Data Collection
* Each test (for a given language) should write out a JSON file to a well known directory name, that includes the following information:
** Test Name
** Language
** Duration (ms)
** Min Memory % Utilized
** Max Memory % Utilized
** Min Memory (MB) Utilized
** Max Memory (MB) Utilized
** Min CPU % Utilized
** Max CPU % Utilized

* TBD: Other data to include in the JSON output file

* An R program will be written to consume the various Language performance metric output files

* Each Language performance metrics output file will conform to the following naming convention:
** <Test ID>.<Language>.json
** For example: pb-001.rust.json
