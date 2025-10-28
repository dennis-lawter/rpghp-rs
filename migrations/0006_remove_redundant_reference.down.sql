alter table
    rpghp_creature
add column
    session_id
    uuid not null
    references rpghp_session(rpghp_session_id);
