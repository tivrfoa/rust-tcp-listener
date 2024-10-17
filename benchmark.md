

## 004 - 2024-10-17

Enable keep-alive by default. Only close if Connection: close is received.

```txt
wrk -t12 -c400 -d30s -R1000 http://localhost:7878/
Running 30s test @ http://localhost:7878/
  12 threads and 400 connections
  Thread calibration: mean lat.: 0.985ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.983ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.804ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.992ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.968ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.960ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 2.050ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.003ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.940ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.951ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.937ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.970ms, rate sampling interval: 10ms
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.10ms  559.72us   4.12ms   69.86%
    Req/Sec    85.97    230.62     3.67k    99.03%
  29974 requests in 30.17s, 3.26MB read
Requests/sec:    993.57
Transfer/sec:    110.61KB
```

## 2024-10-14

### Each thread with its own channel

```txt
wrk -t12 -c400 -d30s -R1000 http://localhost:7878/
Running 30s test @ http://localhost:7878/
  12 threads and 400 connections
  Thread calibration: mean lat.: 48.588ms, rate sampling interval: 556ms
  Thread calibration: mean lat.: 221.939ms, rate sampling interval: 633ms
  Thread calibration: mean lat.: 28.605ms, rate sampling interval: 72ms
  Thread calibration: mean lat.: 33.447ms, rate sampling interval: 485ms
  Thread calibration: mean lat.: 218.854ms, rate sampling interval: 629ms
  Thread calibration: mean lat.: 229.369ms, rate sampling interval: 649ms
  Thread calibration: mean lat.: 491.732ms, rate sampling interval: 2746ms
  Thread calibration: mean lat.: 64.674ms, rate sampling interval: 615ms
  Thread calibration: mean lat.: 47.378ms, rate sampling interval: 539ms
  Thread calibration: mean lat.: 55.957ms, rate sampling interval: 585ms
  Thread calibration: mean lat.: 395.852ms, rate sampling interval: 2375ms
  Thread calibration: mean lat.: 227.524ms, rate sampling interval: 621ms
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    50.19ms  182.98ms   1.20s    96.17%
    Req/Sec    83.15     60.65   211.00     64.25%
  29674 requests in 30.17s, 2.78MB read
  Socket errors: connect 0, read 182, write 0, timeout 0
Requests/sec:    983.58
Transfer/sec:     94.51KB
```

thread_pool3
Passing TcpStream and handle_request function pointer to channel.

```
wrk -t12 -c400 -d30s -R1000 http://localhost:7878/
Running 30s test @ http://localhost:7878/
  12 threads and 400 connections
  Thread calibration: mean lat.: 269.183ms, rate sampling interval: 733ms
  Thread calibration: mean lat.: 251.589ms, rate sampling interval: 518ms
  Thread calibration: mean lat.: 269.919ms, rate sampling interval: 732ms
  Thread calibration: mean lat.: 271.118ms, rate sampling interval: 733ms
  Thread calibration: mean lat.: 269.903ms, rate sampling interval: 732ms
  Thread calibration: mean lat.: 252.128ms, rate sampling interval: 519ms
  Thread calibration: mean lat.: 272.885ms, rate sampling interval: 733ms
  Thread calibration: mean lat.: 275.242ms, rate sampling interval: 733ms
  Thread calibration: mean lat.: 275.145ms, rate sampling interval: 734ms
  Thread calibration: mean lat.: 273.282ms, rate sampling interval: 733ms
  Thread calibration: mean lat.: 275.610ms, rate sampling interval: 734ms
  Thread calibration: mean lat.: 275.419ms, rate sampling interval: 734ms
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   111.68ms  125.08ms 334.59ms   72.94%
    Req/Sec    83.81     15.82   127.00     71.39%
  29913 requests in 30.10s, 2.82MB read
  Socket errors: connect 0, read 28, write 0, timeout 0
Requests/sec:    993.83
Transfer/sec:     95.99KB
```

### 001 - 2024-10-13

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

### 002

Requests/sec: 993.61

```txt
wrk -t12 -c400 -d30s -R1000 http://localhost:7878/
Running 30s test @ http://localhost:7878/
  12 threads and 400 connections
  Thread calibration: mean lat.: 0.987ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.026ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.959ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.025ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.994ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.001ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.970ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.971ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 3.977ms, rate sampling interval: 12ms
  Thread calibration: mean lat.: 3.966ms, rate sampling interval: 13ms
  Thread calibration: mean lat.: 0.973ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 0.933ms, rate sampling interval: 10ms
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.49ms    1.43ms  12.43ms   88.87%
    Req/Sec    86.89    184.88     3.00k    95.58%
  29974 requests in 30.17s, 2.66MB read
  Socket errors: connect 0, read 1784, write 0, timeout 0
Requests/sec:    993.61
Transfer/sec:     90.34KB
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

### Thread Per Request

```txt
wrk -t12 -c400 -d30s -R1000 http://localhost:7878/
Running 30s test @ http://localhost:7878/
  12 threads and 400 connections
  Thread calibration: mean lat.: 146.903ms, rate sampling interval: 677ms
  Thread calibration: mean lat.: 148.502ms, rate sampling interval: 677ms
  Thread calibration: mean lat.: 223.592ms, rate sampling interval: 489ms
  Thread calibration: mean lat.: 225.972ms, rate sampling interval: 495ms
  Thread calibration: mean lat.: 148.617ms, rate sampling interval: 676ms
  Thread calibration: mean lat.: 224.491ms, rate sampling interval: 491ms
  Thread calibration: mean lat.: 150.391ms, rate sampling interval: 677ms
  Thread calibration: mean lat.: 151.370ms, rate sampling interval: 677ms
  Thread calibration: mean lat.: 148.554ms, rate sampling interval: 676ms
  Thread calibration: mean lat.: 150.835ms, rate sampling interval: 676ms
  Thread calibration: mean lat.: 152.045ms, rate sampling interval: 677ms
  Thread calibration: mean lat.: 156.264ms, rate sampling interval: 677ms
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    46.49ms  153.92ms 992.77ms   92.49%
    Req/Sec    83.28     30.03   202.00     87.66%
  29907 requests in 30.17s, 2.78MB read
  Socket errors: connect 0, read 435, write 0, timeout 0
Requests/sec:    991.39
Transfer/sec:     94.45KB
```

## Asking ChatGPT: The server implementation using your thread pool was slower than creating a thread every time for each request. Why do you think so?

The server implementation using a **thread pool** being slower than creating a new thread for each request could be caused by several factors related to the thread pool design, request handling, or resource contention. Here are some possible reasons:

### 1. **Thread Pool Starvation or Overhead**:
   - **Thread pool size**: If your thread pool has too few threads (e.g., fewer than the number of available CPU cores or incoming requests), tasks may be queued, causing delays. This creates a backlog of requests waiting for an available thread, which could lead to lower throughput compared to spawning a new thread for every request.
   - **Task scheduling overhead**: When you submit tasks to a thread pool, there's overhead in queuing and scheduling jobs for worker threads. If the tasks are lightweight (e.g., small, short-lived HTTP requests), this scheduling overhead may outweigh the benefits of reusing threads.

   **Solution**: Try increasing the number of threads in the pool to see if performance improves. Adjust the size of the pool to better match the workload.

### 2. **Lock Contention on the Job Queue**:
   - The thread pool implementation uses a **shared job queue** guarded by a `Mutex` to distribute work to worker threads. If multiple worker threads try to lock the job queue at the same time, they could block each other, causing delays in fetching tasks.
   - If too many threads try to acquire the lock to pick up tasks from the queue, this contention can slow down the overall performance of the thread pool.

   **Solution**: Reduce lock contention by exploring more efficient job distribution mechanisms, or if your workload is I/O bound, consider using **asynchronous programming** (via `async`/`await`) to avoid locking.

### 3. **Overhead of Task Submission**:
   - Submitting tasks to the pool involves sending jobs over a channel and locking the receiver (protected by `Mutex`). This overhead could become significant if you're handling very lightweight tasks like quick HTTP requests, where the cost of enqueuing the job could outweigh the benefit of reusing threads.
   - By contrast, creating new threads every time incurs system-level overhead, but for small tasks, this might not be as costly compared to the thread pool's overhead in task management.

   **Solution**: For very lightweight tasks, a thread-per-request model might perform better due to lower task submission overhead. Alternatively, optimizing the task submission mechanism (e.g., using lock-free queues) could help.

### 4. **Thread Pool Warm-up Time**:
   - If the thread pool starts with idle threads (or no pre-warmed threads), there might be an initial delay as threads are spawned and initialized to handle requests.
   - In contrast, spawning a thread for every request might be faster initially, as there’s no need to manage a pool.

   **Solution**: You could pre-warm the thread pool by initializing threads ahead of time or keeping them active to reduce any startup latency.

### 5. **Resource Contention**:
   - **CPU cores**: If your server runs on a multi-core machine, having too few threads (less than the number of CPU cores) in the pool can underutilize the available hardware. On the other hand, having too many threads can lead to context-switching overhead, especially if tasks are CPU-bound.
   - **I/O-bound tasks**: If your tasks are I/O-bound (e.g., reading from or writing to the network), the pool size should typically be higher to ensure more threads can handle I/O requests concurrently while waiting for external resources.

   **Solution**: Profile your server to understand whether it’s CPU-bound or I/O-bound and adjust the thread pool size accordingly. For I/O-bound workloads, a larger pool may help improve concurrency.

### 6. **Inefficient Workload Distribution**:
   - Depending on the workload, the pool might not be distributing jobs evenly among worker threads. Some threads could be over-utilized, while others remain idle, which leads to inefficient resource usage and lower performance.

   **Solution**: Ensure that the workload is balanced across all threads in the pool, and profile the distribution of tasks among threads.

### 7. **Thread Reuse Penalties**:
   - The thread pool reuses threads to handle multiple tasks, but if tasks involve heavy I/O operations or blocking, keeping the thread alive may lead to resource contention (e.g., the thread holding onto system resources like file descriptors, memory, etc., for too long).
   - Conversely, creating a new thread for each request ensures that the system releases resources as soon as the thread completes, which can sometimes be more efficient in certain situations.

   **Solution**: Profile how threads behave with your workload and check if keeping threads alive in a pool causes resource contention or delays.

---

### **How to Diagnose and Fix the Issue**:
1. **Profile the Server**: Use profiling tools (e.g., `perf`, `strace`, or the Rust `cargo flamegraph` tool) to analyze where the bottlenecks are. Check for things like:
   - Lock contention.
   - Idle threads in the pool.
   - Queue wait times.

2. **Experiment with Pool Size**: Increase the thread pool size and see how performance changes. Start by matching the number of threads to the number of CPU cores and then experiment with higher values if your workload is more I/O-bound.

3. **Consider Asynchronous Programming**: If your workload is I/O-heavy, Rust’s `async` and `await` features might perform better than using a thread pool. Asynchronous programming models are more scalable for handling many concurrent I/O-bound requests, as they avoid blocking threads while waiting for I/O operations to complete.

Let me know if you'd like help adjusting the thread pool design or trying asynchronous programming for the server!
