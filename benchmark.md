
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
