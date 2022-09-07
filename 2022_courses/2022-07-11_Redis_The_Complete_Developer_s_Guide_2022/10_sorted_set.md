### Sorted set
---

#### chapter 1
zadd students 1 rover
zadd students 2 joe
zadd students 3 doe

zcard students
zcount students 1 2

zscore students joe
zadd students  incr 0.1 joe
zscore students joe
zrem students rover


del products

zadd products 45 cpu
zadd products 10 keyboard
zadd products 55 power

zcard products
zcount products  10 45
zcount products  10 45
zcount products  10 (45
zcount products  -inf +inf

zpopmin products 1
zpopmax products 2

zincrby products 1.1 cpu
zscore products cpu
zrange products 10 48

zrange products 0 10

zrange products 0 2 withscores # index 0, 1, 2

zrange products 0 0 withscores

zrange products -1 -1 withscores


zrange products 0 50 byscore withscores
zrange products 0 (55 byscore withscores

zrange products 0 3 rev
zrange products 0 50 withscores

// select score range 50 to 0 and pick the first element only [0]
zrange products 50 0 byscore rev withscores limit 0 1

// the first two elements
zrange products 0 1 
// the last two elements
zrange products 0 1 rev
