INSERT INTO Logs (service_id, status, message, time, duration) VALUES
(1, 1, 'Service started successfully', '2024-07-27 10:00:00', 120),
(2, 2, 'Service encountered an error', '2024-07-27 10:05:00', 45),
(1, 0, 'Service paused', '2024-07-27 10:10:00', 30),
(2, 1, NULL, '2024-07-27 10:15:00', 10),
(2, 1, NULL, '2024-07-27 10:20:00', 60);
