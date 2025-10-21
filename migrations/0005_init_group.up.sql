-- This alteration is extremely damaging,
-- and the best approach is a full revert first

create table rpghp_init_group
(
    rpghp_init_group_id UUID primary key,
    session_id UUID not null references rpghp_session,
    rank bigint not null
);

alter table
    rpghp_creature
add
    rpghp_init_group_id UUID
        not null,
add constraint
    fk_rpghp_init_group_id
        foreign key (rpghp_init_group_id)
        references rpghp_init_group (rpghp_init_group_id)
        on delete restrict;
