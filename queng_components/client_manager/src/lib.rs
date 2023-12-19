use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClientManager {
    clients: HashMap<u16, String>,
}

impl ClientManager {
    //! ClientManager stores client information in a hashmap.
    //!
    //! # Usage
    //!
    //! ```
    //! use client_manager::ClientManager;
    //!
    //! let mut manager = ClientManager::new();
    //!
    //! // Add a new client
    //! manager.add_client(1, "Client 1".to_string()).expect("Failed to add client");
    //!
    //! // Get a client
    //! let client = manager.get_client(1);
    //!
    //! // Update a client
    //! manager.update_client(1, "Updated Client".to_string());
    //!
    //! // Check if a client id exists
    //! let exists = manager.check_client(1);
    //!
    //! // Remove a client
    //! manager.remove_client(1);
    //! ```
    //!
    //! # Methods
    //!
    //! - `add_client` - Adds a new client to the manager. Takes a `u16` id and a `String` name.
    //! Returns a `Result`.
    //!
    //! - `get_client` - Gets a client by id. Takes a `u16` id and returns a `Result` with the client `String`.
    //!
    //! - `update_client` - Updates a client. Takes a `u16` id and a `String` name.
    //!
    //! - `check_client` - Checks if a client id exists. Takes a `u16` id and returns a `bool`.
    //!
    //! - `remove_client` - Removes a client by id. Takes a `u16` id.
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
}

impl ClientManager {
    /// Adds a new client to the manager.
    ///
    /// Takes a `u16` id and a `String` name.  
    /// Returns a `Result`.
    ///
    /// If the id already exists, returns an Err with
    /// "Client id already exists" message.
    /// Otherwise inserts the id and name into the hashmap
    /// and returns Ok.
    pub fn add_client(&mut self, id: u16, name: String) -> Result<(), &'static str> {
        if self.clients.contains_key(&id) {
            return Err("Client id already exists");
        }

        self.clients.insert(id, name);

        Ok(())
    }

    /// Gets a client by id.
    ///
    /// Takes a `u16` id and returns a `Result` with the client `String`.
    ///
    /// If the id does not exist, returns an Err with
    /// "Client id does not exist" message.
    /// Otherwise returns the client String in an Ok.
    pub fn get_client(&self, id: u16) -> Result<&String, &'static str> {
        if !self.clients.contains_key(&id) {
            return Err("Client id does not exist");
        }

        self.clients.get(&id).ok_or("Client not found")
    }

    /// Updates a client.
    ///
    /// Takes a `u16` id and a `String` name.
    /// Inserts the id and name into the hashmap,
    /// overwriting any existing entry with the same id.
    /// If no entry exists, adds a new one.
    pub fn update_client(&mut self, id: u16, name: String) {
        self.clients.insert(id, name);
    }

    /// Checks if a client id exists.
    ///
    /// Takes a `u16` id and returns a `bool`.
    /// Returns true if the id exists in the hashmap.
    /// Returns false if the id does not exist.
    pub fn check_client(&self, id: u16) -> bool {
        self.clients.contains_key(&id)
    }

    /// Removes a client by id.
    ///
    /// Takes a `u16` id.
    /// If the id exists, removes it from the hashmap.
    /// If it does not exist, does nothing.
    pub fn remove_client(&mut self, id: u16) {
        if self.clients.contains_key(&id) {
            self.clients.remove(&id);
        }
    }
}
