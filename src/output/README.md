# Output Publishers

Here we define structs to represent an output publisher (like Google Cloud Pub/Sub, RabbitMQ, JSON, etc).  Each one is represented by the same struct, however depending on which feature is activated, we implement the necessary methods to run the publisher.
