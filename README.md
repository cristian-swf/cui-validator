# CUI Validator
A basic API written in Rust using Rocket web framework version 5 RC 3, used to validate a Romanian Company Identification Number (CUI).

It provides 3 routes:

* **GET** /about - returns details about the API
* **GET** /uptime - returns the uptime in seconds
* **GET** /validate/_cui_ - validate CUI number

In Romania, a Company Identification Number (CUI), known as "Cod Unic de Înregistrare" in Romanian, is a unique identification number assigned to legal entities, including companies, registered in the country. The CUI is used for tax and legal purposes and is issued by the Romanian tax authorities.

The CUI is a combination of digits, and its format may change over time due to regulatory updates. At the moment of writing this API, the CUI consists of 7 to 10 digits. This unique number helps the government and other relevant authorities track and identify businesses for tax, legal, and administrative purposes.

Example of CUI: 33034700

To run:
```shell
cargo run
```

Build for production:
```shell
cargo build -r
```

To run tests:
```shell
cargo test
```

Load testing using _autocannon_:
```shell
autocannon -c 100 -d 5 -p 10 http://127.0.0.1:8000/validate/33034700
```