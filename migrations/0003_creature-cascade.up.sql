alter table
    rpghp_creature
drop constraint
    rpghp_creature_session_id_fkey;

alter table
    rpghp_creature
add constraint
    rpghp_creature_session_id_fkey
        foreign key (session_id)
        references rpghp_session(rpghp_session_id)
        on delete cascade;
