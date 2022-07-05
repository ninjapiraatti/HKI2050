create view rich_contenttags as
  select 
    row_number() OVER ()::integer AS idx,
    c.id as contenttag_id,
    c.tag_id as tag_id,
    c.content_id as content_id,
    t.title as tag_title
  from
    contenttags c,
    tags t
  where 
    c.tag_id = t.id