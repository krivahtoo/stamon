-- Docker checks name a container instead of a URL or host.
ALTER TABLE Services DROP COLUMN target;

ALTER TABLE Services
ADD target TEXT GENERATED ALWAYS AS (
    COALESCE(
        json_extract(config, '$.url'),
        json_extract(config, '$.host'),
        json_extract(config, '$.container')
    )
) VIRTUAL;
