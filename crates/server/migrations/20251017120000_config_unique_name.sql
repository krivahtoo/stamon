-- Settings are looked up and saved by name, so keep one row per name.
DELETE FROM Configs
WHERE id NOT IN (SELECT MAX(id) FROM Configs GROUP BY name);

CREATE UNIQUE INDEX idx_configs_name ON Configs(name);
