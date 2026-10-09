INSERT INTO Services (user_id, active, name, interval, config, timeout, last_status, retry, retry_interval) VALUES
(1, 1, 'Service One', 5, '{"type":"http","url":"https://service-one.com/api","method":"POST","body":"{\"key\":\"value\"}"}', 10, 0, 3, 5),
(2, 1, 'Service Two', 10, '{"type":"ping","host":"service-two.com"}', 15, 1, 3, 10),
(3, 0, 'Service Three', 15, '{"type":"http","url":"https://service-three.com/api","method":"POST","body":"{\"data\":\"test\"}"}', 20, 0, 5, 15),
(4, 1, 'Service Four', 20, '{"type":"ping","host":"service-four.com"}', 25, 0, 2, 20),
(1, 1, 'Service Five', 25, '{"type":"http","url":"https://service-five.com/api"}', 30, 2, 4, 25);
