-- Your SQL goes here

ALTER TABLE contenttags
  ADD CONSTRAINT fk_tags_id
    FOREIGN KEY (tag_id)
        REFERENCES tags(id);
ALTER TABLE contenttags      
  ADD CONSTRAINT fk_articles_id
    FOREIGN KEY (content_id)
        REFERENCES articles(id);