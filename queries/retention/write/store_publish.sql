INSERT OR FAIL INTO retention_publish (
    t_millis,
    counter,
    interface,
    path,
    expiry_t_secs,
    sent,
    payload
) VALUES (?, ?, ?, ?, ?, ?, ?);
