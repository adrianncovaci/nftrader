Checkpoint 1
===============

List of microservices
-----

* Chat
* Fetch User and Images
* Escrow
* Compress

API endpoints
-----

1. `GET /chat` fetch current message feed (includes all messages sent up until that moment)


2. `GET /images_and_users` fetch available images and users


3. `POST /message` post a message entry inside the database


4. `POST /compress` encode an image, compress it and store the hash of it

List of technologies to be used
-----

* [Rust](https://www.rust-lang.org)
* [PostgreSQL](https://www.postgresql.org)
* [solana-cli](https://docs.solana.com)
* [Node.js](https://nodejs.org/en/)

System Arhitecture
-----

<img width="1344" alt="Screenshot 2021-09-24 at 17 48 33" src="https://user-images.githubusercontent.com/51786337/134705010-e75a356e-84d3-4d1e-9a8e-c5415362ad76.png">
