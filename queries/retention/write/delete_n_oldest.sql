DELETE FROM retention_publish
WHERE (t_millis, counter) IN (
    SELECT t_millis, counter
    FROM retention_publish
    -- should alreay be ordered by t_millis, counter
    -- ORDER BY t_millis ASC, counter ASC
    LIMIT ?
);
