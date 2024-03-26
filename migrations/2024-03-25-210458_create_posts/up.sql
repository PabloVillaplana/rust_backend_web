-- up.sql
Create table posts (
    id int primary key auto_increment,
    title varchar(25) not null,
    slug varchar(50) not null,
    body text not null
);
