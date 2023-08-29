select name
  from track
  where albumid = 193
  order by trackid;

select track.name as track, genre.name as genre
  from track
  join genre using(genreid)
  where albumid = 193
  order by trackid;
-- join: inner join

select name,
    milliseconds * interval '1 ms' as duration,
    pg_size_pretty(bytes) as bytes
  from track
  where albumid = 193
  order by trackid;

select album.title as album,
    sum(milliseconds) * interval '1 ms' as duration
  from album
  join artist using(artistid)
  left join track using(albumid)
  where artist.name = 'Red Hot Chili Peppers'
  group by album
  order by album;

-- default isolation level: read commited
-- online backup isolation level: repeatable read
