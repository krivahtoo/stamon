INSERT INTO services (user_id, active, name, interval, url, payload, timeout, last_status, service_type, retry, retry_interval) VALUES
(1, 1, 'Service One', 5, 'https://service-one.com/api', '{"key":"value"}', 10, 0, 'http', 3, 5),
(2, 1, 'Service Two', 10, 'https://service-two.com/api', NULL, 15, 1, 'ping', 3, 10),
(3, 0, 'Service Three', 15, 'https://service-three.com/api', '{"data":"test"}', 20, 0, 'http', 5, 15),
(4, 1, 'Service Four', 20, 'https://service-four.com/api', '{"username":"user","password":"pass"}', 25, 0, 'ping', 2, 20),
(1, 1, 'Service Five', 25, 'https://service-five.com/api', NULL, 30, 2, 'http', 4, 25);