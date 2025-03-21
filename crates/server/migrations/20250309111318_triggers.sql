-- Update service status
CREATE TRIGGER update_service_status
AFTER INSERT ON Logs
FOR EACH ROW
BEGIN
    UPDATE Services
    SET last_status = NEW.status
    WHERE id = NEW.service_id;
END;

-- Update last_update
CREATE TRIGGER update_config_timestamp
AFTER UPDATE ON Configs
FOR EACH ROW
BEGIN
    UPDATE Configs
    SET last_updated = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
