create table rpghp_creature
(
    rpghp_creature_id UUID primary key,
    session_id UUID not null references rpghp_session,

    creature_name text not null,
    max_hp int not null,
    curr_hp int not null,
    hp_hidden boolean not null
);
