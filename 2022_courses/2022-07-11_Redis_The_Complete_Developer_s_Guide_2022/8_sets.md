### Set
---

#### chapter 1
sadd colors red
sadd colors green
sadd colors yellow
sadd colors purple
sadd colors pink

smembers colors
srem colors red
spop colors
scard colors # number of elements in set

sismember colors yellow
sismember colors xxxx

sadd nums red
sadd nums orange
sadd nums 12

sunion colors nums
sdiff colors nums # elements in colors only
sinter colors nums

smove  colors nums green
smembers colors
smembers nums


sscan colors 0 count 2 # cursor, return cursor is 0 means there is no more elements
