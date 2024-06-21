# echoserver-rs
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High performance echoserver for load testing

Note: This is a simple echoserver written in Rust. It is not a full-fledged web server. It's still under development.

## Usage
```bash
cargo run --release
```

## Docker image

### Build
```bash
docker build -t echoserver-rs . 
```

### Run
```bash
docker run -d -p 8080:8080 echoserver-rs
```

## Simple Benchmark

```
$ ab -k -c 32 -n 200000 127.0.0.1:8080/test
This is ApacheBench, Version 2.3 <$Revision: 1879490 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking 127.0.0.1 (be patient)
Completed 20000 requests
Completed 40000 requests
Completed 60000 requests
Completed 80000 requests
Completed 100000 requests
Completed 120000 requests
Completed 140000 requests
Completed 160000 requests
Completed 180000 requests
Completed 200000 requests
Finished 200000 requests


Server Software:        
Server Hostname:        127.0.0.1
Server Port:            8080

Document Path:          /test
Document Length:        38 bytes

Concurrency Level:      32
Time taken for tests:   14.848 seconds
Complete requests:      200000
Failed requests:        0
Keep-Alive requests:    200000
Total transferred:      35200000 bytes
HTML transferred:       7600000 bytes
Requests per second:    13469.74 [#/sec] (mean)
Time per request:       2.376 [ms] (mean)
Time per request:       0.074 [ms] (mean, across all concurrent requests)
Transfer rate:          2315.11 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.0      0       1
Processing:     0    2  15.3      0     422
Waiting:        0    2  15.3      0     422
Total:          0    2  15.3      0     422

Percentage of the requests served within a certain time (ms)
  50%      0
  66%      0
  75%      0
  80%      0
  90%      1
  95%      1
  98%     64
  99%     68
 100%    422 (longest request)
 ```
