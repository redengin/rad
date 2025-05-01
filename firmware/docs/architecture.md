Rust Autonomous Drone Architecture
================================================================================
```mermaid
architecture-beta
    group fc(cloud)[Flight Controller]

    %% service db(database)[Database] in api
    %% service disk1(disk)[Storage] in api
    %% service disk2(disk)[Storage] in api
    %% service server(server)[Server] in api

    %% db:L -- R:server
    %% disk1:T -- B:server
    %% disk2:T -- B:db
```