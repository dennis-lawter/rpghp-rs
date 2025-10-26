alter table
    rpghp_creature
drop constraint
    rpghp_creature_session_id_fkey;

alter table
    rpghp_creature
drop column
    session_id;
