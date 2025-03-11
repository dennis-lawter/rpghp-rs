create table rpghp_creature
(
    rpghp_creature_id UUID primary key,
    session_id UUID references rpghp_session,

    name text,
    max_hp int,
    curr_hp int
);
