-- Free-form labels for grouping and finding services, as a JSON array of strings
ALTER TABLE Services
ADD tags TEXT NOT NULL DEFAULT '[]';
