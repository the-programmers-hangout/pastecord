-- Add migration script here
alter table pastes
    add user_id integer NULL;

alter table pastes
    add constraint pastes_users_id_fk
        foreign key (user_id) references users;
