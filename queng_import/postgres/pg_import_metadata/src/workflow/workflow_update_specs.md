
Implement update_instruments_metadata to update the instruments metadata in the database.

Extract the stats from the meta_data parameter 

Extract the expected instruments count using the .number_instruments() methods from stats 

Load all instruments from the database 

Extract the database instruments count by calling len on the instruments list from the database 

Iterate through the database instruments, find the
instruments that are in the database but not in the meta_data parameter, 
check if it exists in database, and if so, delete it from the database.

Iterate through the meta_data parameter, find the instruments that 
are in the meta_data parameter but not in the database, 
check if it exists in database, and if not, insert it into the database.

Load all instruments from the database again

Compare the expected and actual instruments counts again and ensure they are equal; if not, raise a panic.

Iterate through all database instruments, match each instruments code with the instruments code in the meta_data parameter, 
compare the instruments hash of the database instruments to the instruments hash in the meta_data parameter;  
if the hashes don't match, update the database instruments in the database with the instruments from the meta_data. 
If the hashes match, just proceed to the next iteration.

