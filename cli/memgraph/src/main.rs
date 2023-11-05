use rsmgclient::{ConnectParams, Connection, ConnectionStatus, SSLMode, Value};

const DELETE_ONLY: bool = false;

fn main() {
    // Connect to Memgraph
    let connect_params = ConnectParams {
        host: Some(String::from("localhost")),
        port: 7687,
        sslmode: SSLMode::Disable,
        ..Default::default()
    };
    let mut connection = Connection::connect(&connect_params).unwrap();

    // Check if connection is established.
    let status = connection.status();

    if status != ConnectionStatus::Ready {
        println!("Connection failed with status: {:?}", status);
        return;
    } else {
        println!("Connection established with status: {:?}", status);
    }

    connection
        .execute_without_results("MATCH (n) DETACH DELETE n;")
        .unwrap();
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }
    println!("Graph has been cleared!");

    connection
        .execute_without_results("DROP INDEX ON :Developer(id);")
        .unwrap();
    connection
        .execute_without_results("DROP INDEX ON :Developer(name);")
        .unwrap();
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }

    connection
        .execute_without_results("DROP INDEX ON :Technology(id);")
        .unwrap();
    connection
        .execute_without_results("DROP INDEX ON :Technology(name);")
        .unwrap();
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }
    println!("Indexes have been cleared!");

    if DELETE_ONLY {
        // Close the connection.
        connection.close();
        return;
    }

    for index in get_indexes() {
        connection.execute_without_results(index).unwrap();
    }
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }
    println!("Indexes have been created!");

    for developer_node in get_developer_nodes() {
        connection.execute_without_results(developer_node).unwrap();
    }
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }
    println!("Developer nodes have been created!");

    for technology_node in get_technology_nodes() {
        connection.execute_without_results(technology_node).unwrap();
    }
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }
    println!("Technology nodes have been created!");

    for relationship in get_relationships() {
        connection.execute_without_results(relationship).unwrap();
    }
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }
    println!("Relationships have been created!");

    // Fetch the graph.
    let columns = connection.execute("MATCH (n)-[r]->(m) RETURN n, r, m;", None);
    println!("Columns: {}", columns.unwrap().join(", "));

    while let Ok(result) = connection.fetchall() {
        for record in result {
            for value in record.values {
                match value {
                    Value::Node(node) => println!("Node: {}", node),
                    Value::Relationship(edge) => println!("Edge: {}", edge),
                    value => println!("Value: {}", value),
                }
            }
        }

        println!();
    }

    // Close the connection.
    connection.close();
}

fn get_indexes<'l>() -> Vec<&'l str> {
    vec![
        "CREATE INDEX ON :Developer(id);",
        "CREATE INDEX ON :Technology(id);",
        "CREATE INDEX ON :Developer(name);",
        "CREATE INDEX ON :Technology(name);",
    ]
}

fn get_developer_nodes<'l>() -> Vec<&'l str> {
    vec![
        "CREATE (n:Developer {id: 1, name:'Andy'});",
        "CREATE (n:Developer {id: 2, name:'John'});",
        "CREATE (n:Developer {id: 3, name:'Michael'});",
    ]
}

fn get_technology_nodes<'l>() -> Vec<&'l str> {
    vec![
        "CREATE (n:Technology {id: 1, name:'Memgraph', description: 'Fastest graph DB in the world!', createdAt: Date()})",
        "CREATE (n:Technology {id: 2, name:'Rust', description: 'Rust programming language ', createdAt: Date()})",
        "CREATE (n:Technology {id: 3, name:'Docker', description: 'Docker containerization engine', createdAt: Date()})",
        "CREATE (n:Technology {id: 4, name:'Kubernetes', description: 'Kubernetes container orchestration engine', createdAt: Date()})",
        "CREATE (n:Technology {id: 5, name:'Python', description: 'Python programming language', createdAt: Date()})",
    ]
}

fn get_relationships<'l>() -> Vec<&'l str> {
    vec![
        "MATCH (a:Developer {id: 1}),(b:Technology {id: 1}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 2}),(b:Technology {id: 3}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 3}),(b:Technology {id: 1}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 1}),(b:Technology {id: 5}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 2}),(b:Technology {id: 2}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 3}),(b:Technology {id: 4}) CREATE (a)-[r:LOVES]->(b);",
    ]
}
