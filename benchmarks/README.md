# Performance Benchmark Test Suites

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
* Ruby
* Scala

# Performance Benchmark Scenarios
* PB-001: Prime Number Generator
* (others to be added)


# Reporting Metrics Data Collection
* Each test (for a given language) should write out a JSON file to a well known directory name, that includes the following information:
	* Test Name
	* Language
	* Duration (ms)	
	* Min Memory % Utilized
	* Max Memory % Utilized
	* Min Memory (MB) Utilized
	* Max Memory (MB) Utilized
	* Min CPU % Utilized
	* Max CPU % Utilized

* TBD: Other data to include in the JSON output file

* An R program will be written to consume the various Language performance metric output files

* Each Language performance metrics output file will conform to the following naming convention:
	* `<Test ID>`.`<Language>`.json
	* For example: pb-001.rust.json

* Possible way to simplify & automate the collection of system performance metrics:
	* Consider possibly leveraging one or more of the following to collect system performance / resource utilization:
		* [Nagios](https://www.nagios.org/)
		* [Cacti](http://www.cacti.net/)
		* [Icinga](https://www.icinga.org/)
		* [Zabbix](http://www.zabbix.com/)
		* [Ganglia](http://ganglia.info/)
	* Use a shell script to sequentially kick-off each test suite, with a call to some monitoring agent tooling to start/stop the monitoring - and output the results to a secondary data collection file (e.g. pb-001.java.system.json)?
	* Others?
		* https://en.wikipedia.org/wiki/List_of_performance_analysis_tools
