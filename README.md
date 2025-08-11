# Welcome to Loco :train:

[Loco](https://loco.rs) is a web and API framework running on Rust.

This is the **SaaS starter** which includes a `User` model and authentication based on JWT.
It also include configuration sections that help you pick either a frontend or a server-side template set up for your fullstack server.

## Quick Start

```sh
cargo loco start
```

```sh
$ cargo loco start --server-and-worker
Finished dev [unoptimized + debuginfo] target(s) in 21.63s
    Running `target/debug/myapp start`

    :
    :
    :

controller/app_routes.rs:203: [Middleware] Adding log trace id

environment: development
   database: automigrate
     logger: debug
compilation: debug
      modes: server

listening on http://localhost:5150
```

## Full Stack Serving

You can check your [configuration](config/development.yaml) to pick either frontend setup or server-side rendered template, and activate the relevant configuration sections.

# Check CI

cargo fmt --all -- --check

cargo clippy --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery -W rust-2018-idioms

cargo clippy --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery -W rust-2024-compatibility

cargo clippy --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery -W rust-2024-compatibility &> clippy_note.txt

cargo test &> test_note.txt

cargo test

cargo loco db migrate

INSTA_ACCEPT=yes cargo test test_insert_and_find_hair_color

cargo llvm-cov --html

UPDATE auth.users SET api_key=md5(random()::text) WHERE api_key IS NULL;
UPDATE auth.users SET pid=uuid_generate_v7() WHERE pid = '00000000-0000-0000-0000-000000000000';

## Getting help

Check out [a quick tour](https://loco.rs/docs/getting-started/tour/) or [the complete guide](https://loco.rs/docs/getting-started/guide/).

## run task

```sh
cargo loco task RegenerateAllStudentDetailActivities "student_register_academic_year_id:8591fbfd-d173-4bfc-9d0f-896a24d2cd77" "curriculum_id:b7d9f9d9-1bd1-4c92-a928-58720cf0a214" "unit_activity_id:74f1aef3-406e-435c-b095-1802e4d278f1" "semester_id:4a3826de-0415-43ea-aa87-861281e49a94"
```

```sh
cargo loco task RegenerateAllStudentDetailActivities "student_register_academic_year_id:8591fbfd-d173-4bfc-9d0f-896a24d2cd77" "curriculum_id:b7d9f9d9-1bd1-4c92-a928-58720cf0a214" "unit_activity_id:74f1aef3-406e-435c-b095-1802e4d278f1" "semester_id:4a3826de-0415-43ea-aa87-861281e49a94"
```

```sh
cargo loco task GenerateUnitStudentCampaignActivities "unit_id:8c13311c-5d4b-4840-a71f-ed0dd732e170" "academic_year_id:7fb68ca8-3376-4aef-b158-4c43f1b0f177" "student_academic_year_id:5884b8d6-bab3-4e5e-99bc-739a114596f2"
"fee:3250000.0"
```

```sh
cargo loco task GenerateStudentPaymentMidtransTransaction "unit_id:8c13311c-5d4b-4840-a71f-ed0dd732e170" "academic_year_id:7fb68ca8-3376-4aef-b158-4c43f1b0f177" "student_academic_year_id:5884b8d6-bab3-4e5e-99bc-739a114596f2" "account_id:0196e1f8-5a47-728f-bb16-5f9bda31299b"
```

```sh
cargo loco task GenerateHashPassword "input_password:SayaNurhidayati"
```

## Broadcasting client example

```js
const { io } = require("socket.io-client");
const socket = io("ws://localhost:5150");

// client-side
socket.emit("message", "hello");
socket.on("message-back", (arg) => {
  console.log(arg);
});

socket.on("broadcasting", (arg) => {
  console.log(arg);
});
```

[test pdf](http://localhost:5150/api/academic/student/campaign/activities/print_activity_plan/9cda3fdb-9f5e-4dc0-8449-f1bc90a42b81)

example successful_payment

https://8d38-104-28-217-187.ngrok-free.app/api/payment/midtrans/student/successful_payment?order_id=019732d7-05e0-7584-850d-f2f13d4ef5c7&status_code=200&transaction_status=settlement
