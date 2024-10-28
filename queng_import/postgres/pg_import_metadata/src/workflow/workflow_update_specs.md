
Implement update_assets_metadata to update the asset metadata in the database.


1. Extract the stats from the meta_data parameter
2. Extract the expected assets count using the .number_assets() methods from stats
3. Load all assets from the database
4. Extract the database asset count by calling len on the assets list from the database
5. Compare the expected and actual asset counts
6. If the database asset count is greater than the expected asset count, iterate through the database assets, find the
assets that are in the database but not in the meta_data parameter, and delete them from the database.
7. If the database asset count is less than the expected asset count, iterate through the meta_data parameter, find the assets that 
are in the meta_data parameter but not in the database, and insert them into the database.
8. Load all assets from the database again
9. Compare the expected and actual asset counts again and ensure they are equal; if not, raise a panic.


Next, iterate through all database assets, match each asset code with the asset code in the meta_data parameter, 
compare the asset hash of the database asset to the asset hash in the meta_data parameter, and if the hashes don't match,
update the database asset in the database with the asset from the meta_data. If the hashes match, just proceed to the next iteration.

