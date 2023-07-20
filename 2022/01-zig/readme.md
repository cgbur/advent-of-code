I think the performance difference is mostly due to Rust not have a default nice way of
dealing with bytes and not caring about the utf8 encoding.

streaming: zig implementation, collect all input elements, sort them and get the answer

streaming_three: just keep a 3 element array of the current 3 largest elements

rust-streaming: rust implementation, collect all input elements, sort them and get the answer

rust-streaming-three: rust implementation, just keep a 3 element array of the current 3 largest elements

```sh
❯ poop "./rust-streaming-three src/huge" "./streaming_three src/huge" "./rust-streaming src/huge" "./streaming src/huge" 
Benchmark 1 (18 runs): ./rust-streaming-three src/huge
  measurement          mean ± σ            min … max           outliers         delta
  wall_time           292ms ± 4.48ms     285ms …  298ms          0 ( 0%)        0%
  peak_rss           1.73MB ±  127KB    1.57MB … 1.97MB          0 ( 0%)        0%
  cpu_cycles         1.32G  ± 14.5M     1.31G  … 1.36G           0 ( 0%)        0%
  instructions       5.26G  ±  234      5.26G  … 5.26G           0 ( 0%)        0%
  cache_references    355K  ±  805K      127K  … 3.56M           2 (11%)        0%
  cache_misses       22.1K  ± 1.72K     20.1K  … 27.1K           2 (11%)        0%
  branch_misses      1.91M  ± 19.6K     1.89M  … 1.95M           0 ( 0%)        0%
Benchmark 2 (42 runs): ./streaming_three src/huge
  measurement          mean ± σ            min … max           outliers         delta
  wall_time           120ms ±  824us     118ms …  123ms          3 ( 7%)        ⚡- 58.7% ±  0.5%
  peak_rss            737KB ±    0       737KB …  737KB          0 ( 0%)        ⚡- 57.3% ±  2.3%
  cpu_cycles          533M  ± 2.28M      532M  …  545M           6 (14%)        ⚡- 59.8% ±  0.3%
  instructions       2.33G  ± 4.35      2.33G  … 2.33G           0 ( 0%)        ⚡- 55.7% ±  0.0%
  cache_references   19.0K  ± 13.1K     10.9K  … 83.7K           7 (17%)        ⚡- 94.6% ± 69.5%
  cache_misses       4.06K  ±  803      2.80K  … 6.58K           5 (12%)        ⚡- 81.6% ±  2.9%
  branch_misses      1.85M  ± 3.63K     1.84M  … 1.85M           0 ( 0%)        ⚡-  3.6% ±  0.3%
Benchmark 3 (17 runs): ./rust-streaming src/huge
  measurement          mean ± σ            min … max           outliers         delta
  wall_time           310ms ± 4.07ms     298ms …  316ms          1 ( 6%)        💩+  6.2% ±  1.0%
  peak_rss           6.51MB ±  797KB    6.09MB … 8.18MB          3 (18%)        💩+277.1% ± 22.5%
  cpu_cycles         1.40G  ± 14.2M     1.38G  … 1.42G           0 ( 0%)        💩+  5.4% ±  0.7%
  instructions       5.31G  ±  241      5.31G  … 5.31G           0 ( 0%)          +  1.0% ±  0.0%
  cache_references   1.34M  ± 14.0K     1.32M  … 1.36M           0 ( 0%)        💩+276.5% ± 112.5%
  cache_misses       57.4K  ± 2.97K     52.3K  … 64.4K           0 ( 0%)        💩+159.6% ±  7.5%
  branch_misses      2.27M  ± 11.8K     2.24M  … 2.28M           0 ( 0%)        💩+ 18.5% ±  0.6%
Benchmark 4 (34 runs): ./streaming src/huge
  measurement          mean ± σ            min … max           outliers         delta
  wall_time           149ms ±  344us     149ms …  150ms          0 ( 0%)        ⚡- 48.7% ±  0.5%
  peak_rss           8.96MB ±  825KB    7.91MB … 10.2MB          0 ( 0%)        💩+418.9% ± 22.9%
  cpu_cycles          665M  ±  348K      664M  …  666M           0 ( 0%)        ⚡- 49.8% ±  0.4%
  instructions       2.54G  ±  281      2.54G  … 2.54G           0 ( 0%)        ⚡- 51.7% ±  0.0%
  cache_references   1.45M  ± 23.7K     1.41M  … 1.50M           0 ( 0%)        💩+308.7% ± 77.5%
  cache_misses       82.6K  ± 8.94K     64.9K  …  102K           0 ( 0%)        💩+273.6% ± 19.4%
  branch_misses      5.29M  ± 5.49K     5.28M  … 5.31M           0 ( 0%)        💩+176.5% ±  0.4%

```

Binary sizes after stripping:

```sh
.rwxr-xr-x  346k cgbur 19 Jul 21:36  rust-streaming
.rwxr-xr-x  345k cgbur 19 Jul 21:36  rust-streaming-three
.rwxr-xr-x   80k cgbur 19 Jul 21:36  streaming
.rwxr-xr-x   79k cgbur 19 Jul 21:36  streaming_three
```
