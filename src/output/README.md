# Output Publishers

Here we define structs to represent an output publisher (like BigQuerry, RabbitMQ, etc).  Each one is represnted by the same struct, however depending on which feature is activated, we implement the necessary methods to run the publisher.