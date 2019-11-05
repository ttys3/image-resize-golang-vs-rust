# image-resize-golang-vs-rust

source image num: 163

source image avg size: 7MB

target thumbnail size: 128

go:

```bash
Total took 1m16.964716307s

real	1m16.967s
user	2m20.271s
sys	0m0.329s
```

cgo gmagick:

```bash
Total took 41.406062841s

real	0m41.431s
user	1m4.390s
sys	0m10.490s
```

rust:

```bash
count: 163
total took: 70 s

real	1m10.402s
user	2m36.688s
sys	0m7.960s
```