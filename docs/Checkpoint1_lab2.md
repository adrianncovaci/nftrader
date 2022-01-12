Lab 2 Checkpoint 1
===============


List of microservices
-----

* Proxy 
* Sales
* Billing
* Shipping
* Handler for each service provider

Features 
-----

Service 

      * Long-running saga transactions
      * Database redundancy/replication + failover. Service instances connect to different DB
      replicas
      * ELK stack

The Gateway 

      * Service high availability (if a request to a service fails or the service is otherwise unavail- able, route the request to a different one)
      * ELK stack


Technologies
-----
* [PostgreSQL](https://www.postgresql.org)
* [mongodb](https://www.mongodb.com)
* [Node.js](https://nodejs.org/en/)
* [NServiceBus](https://particular.net/nservicebus)
* [.NET Core](https://dotnet.microsoft.com/en-us/download)
* [ELK](https://www.elastic.co/what-is/elk-stack)


System Arhitecture
-----
![photo_2022-01-12_17-12-48](https://user-images.githubusercontent.com/51786337/149167425-7fdf368d-4575-494b-8d94-58d9b8e69118.jpg)


Description
-----
Lab1 will send messages to the Lab2 Proxy, which will cache the Buy orders, afterwards it will be sent to the Sale Microservice and the Billing Microservice which both will emit messages to the Shipping Microservice which will contain a running Saga handling the Shipping based on the response of the provider's and the messages sent by the Sale and the Billing department. Each provider will store data inside it's own database, messaging will be handled by NServiceBus and Logging via Elasticsearch.
