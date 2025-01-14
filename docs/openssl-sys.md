## Remove openssl-sys dependency

1) Find the package introducing the dependency

> cargo tree -i --target all -p openssl

should show which dependencies are depending on that crate.

2) Reconfigure the crate to use rust-tls or remove it altogether