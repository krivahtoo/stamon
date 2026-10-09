-- Type-specific check settings move into a JSON `config` column, tagged by `type`.
ALTER TABLE Services
ADD config TEXT NOT NULL DEFAULT '{}';

UPDATE Services
SET config = CASE service_type
    WHEN 'http' THEN json_object(
        'type', 'http',
        'url', url,
        -- A payload used to be sent as a POST body.
        'method', CASE WHEN TRIM(COALESCE(payload, '')) = '' THEN 'GET' ELSE 'POST' END,
        'body', CASE WHEN TRIM(COALESCE(payload, '')) = '' THEN NULL ELSE payload END,
        'expected_code', NULLIF(expected_code, 0),
        'expected_payload', CASE
            WHEN TRIM(COALESCE(expected_payload, '')) = '' THEN NULL
            ELSE expected_payload
        END
    )
    ELSE json_object('type', 'ping', 'host', url)
END;

ALTER TABLE Services DROP COLUMN url;
ALTER TABLE Services DROP COLUMN payload;
ALTER TABLE Services DROP COLUMN expected_code;
ALTER TABLE Services DROP COLUMN expected_payload;
ALTER TABLE Services DROP COLUMN service_type;

-- Derived from `config` so they can't drift from it; used for listing and filtering.
ALTER TABLE Services
ADD service_type TEXT GENERATED ALWAYS AS (json_extract(config, '$.type')) VIRTUAL;

ALTER TABLE Services
ADD target TEXT GENERATED ALWAYS AS (
    COALESCE(json_extract(config, '$.url'), json_extract(config, '$.host'))
) VIRTUAL;

-- Check jobs now carry only the service id; queued jobs in the old format can't be read.
DELETE FROM Jobs
WHERE job_type = 'server::models::service::Service';
