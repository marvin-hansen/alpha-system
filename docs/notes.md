# Next steps

Aug/11/2024

[ ] Update all spec config files with input parameter checks in constructor
[ ] Re-run all unit tests to double check no failure
[ ] Add test cases to trigger and test config error 
[ ] Update non-spec and DB config files with input parameter checks in constructor
[ ] Re-run all unit tests to double check no failure
[ ] Add test cases to trigger and test config error

Aug/7/2024

[x] Update ServiceConfig to generalize all endpoints in one singe format

[x] Update all existing service configs to the new format

[ ] Replace PG SQL utils with Diesel

- postgres_utils:	2470 LoC
- common_pg_queries: 775 LoC

[] Update DB to store new service config format...

[] Update ConfigManger to use the new format 

July/30/2024

* Add health-check endpoint auto configuration in ConfigManager
* Update remaining services with dedicated http health endpoint
* Update Service Util to start all services and use health check to determine when ready,. 

https://youtu.be/gbuWJ48T0bE?si=N9xZzG8AfR6S4DAL

Aug/3/2024

Prebuild the MDDB and all IMS services, clients, proto interfaces, and API's. The full end to end game for a complete IMS data, execution, and reporting system. 

Template and pre-integrating services. 
Template and pre-integrating components.
Very fast and powerful... 

DO NOT implement the API. Do NOT. Not yet,

Instead, design an API that replicates the COIN.API interface for data and execution. 

Return a dummy value from the actual API. 

Pre-integrate and iterate the API. 

When the API specification is stable, add tests for one service that will be implemented. 

All tests are suppose to fail. 
Iterate test design to cover more corner cases, add more tests so that more tests fail. 

Then, draft an implementation, iterate the draft, and then convert the draft into a done implementation that completes the moment all tests pass. 

