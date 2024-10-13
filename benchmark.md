
## 2024-10-13 - 001

Requests/sec:    206.72

```txt
wrk -t12 -c400 -d30s -R200 http://localhost:7878/
Running 30s test @ http://localhost:7878/
  12 threads and 400 connections
  Thread calibration: mean lat.: 1.260ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.125ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.140ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.093ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.102ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.069ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 5.537ms, rate sampling interval: 17ms
  Thread calibration: mean lat.: 1.076ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 5.565ms, rate sampling interval: 16ms
  Thread calibration: mean lat.: 1.224ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.184ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 4.695ms, rate sampling interval: 13ms
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.90ms    1.95ms  11.69ms   85.25%
    Req/Sec    17.33     89.76     2.75k    93.30%
  6237 requests in 30.17s, 565.58KB read
  Socket errors: connect 0, read 387, write 0, timeout 0
Requests/sec:    206.72
Transfer/sec:     18.75KB
```

httperf

```txt
httperf --server localhost --port 7878 --num-conns 1000 --rate 10
httperf --client=0/1 --server=localhost --port=7878 --uri=/ --rate=10 --send-buffer=4096 --recv-buffer=16384 --num-conns=1000 --num-calls=1
httperf: warning: open file limit > FD_SETSIZE; limiting max. # of open files to FD_SETSIZE
Maximum connect burst length: 1

Total: connections 1000 requests 1000 replies 1000 test-duration 99.901 s

Connection rate: 10.0 conn/s (99.9 ms/conn, <=1 concurrent connections)
Connection time [ms]: min 0.3 avg 0.5 max 1.7 median 0.5 stddev 0.1
Connection time [ms]: connect 0.0
Connection length [replies/conn]: 1.000

Request rate: 10.0 req/s (99.9 ms/req)
Request size [B]: 62.0

Reply rate [replies/s]: min 10.0 avg 10.0 max 10.0 stddev 0.0 (19 samples)
Reply time [ms]: response 0.5 transfer 0.0
Reply size [B]: header 97.0 content 2.0 footer 0.0 (total 99.0)
Reply status: 1xx=0 2xx=1000 3xx=0 4xx=0 5xx=0

CPU time [s]: user 8.13 system 91.76 (user 8.1% system 91.9% total 100.0%)
Net I/O: 1.6 KB/s (0.0*10^6 bps)

Errors: total 0 client-timo 0 socket-timo 0 connrefused 0 connreset 0
Errors: fd-unavail 0 addrunavail 0 ftab-full 0 other 0
```
