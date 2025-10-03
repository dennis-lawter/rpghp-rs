create table rpghp_init_group
(
    rpghp_init_group_id UUID primary key,
    rank bigint not null
);

insert into rpghp_init_group
(rpghp_init_group_id, rank)
values
('00000000-0000-0000-0000-000000000000', 0);

alter table
    rpghp_creature
add
    rpghp_init_group_id UUID
        not null default '00000000-0000-0000-0000-000000000000',
add constraint
    fk_rpghp_init_group_id
        foreign key (rpghp_init_group_id)
        references rpghp_init_group (rpghp_init_group_id)
        on delete restrict;
