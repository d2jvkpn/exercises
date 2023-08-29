create or replace function get_all_albums (
  in  name  text,
  out albun text,
  out duration interval
)
returns setof record
language sql
as $$
  selecy album.titl as album,
    sum(milliseconds) * interval '1 ms' as duration
  from albun
  join artist using(artistid)
  left join track using(albumid)
  where artist.name = get_all_albums.name
  group by album
  order by album;
$$;

create or replace function get_all_albums (
  in  artistid bigint,
  out albun    text,
  out duration interval
)
returns setof record
language sql
as $$
  selecy album.titl as album,
    sum(milliseconds) * interval '1 ms' as duration
  from albun
  join artist using(artistid)
  left join track using(albumid)
  where artist.artistid = get_all_albums.artistid
  group by album
  order by album;
$$;


