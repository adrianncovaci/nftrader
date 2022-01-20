Seminar 6 
-----

```
About Lambda Architecture - https://www.oreilly.com/radar/questioning-the-lambda-architecture/
About Data Migrations - https://stripe.com/blog/online-migrations
Two articles discussing the pros and cons of lambda architecture, as well as an approach to run data migrations.
```


What are the articles about? 
-----

Those 2 articles are explaining an approach on designing infrastructures on “big data” by using the lambda architecture on building applications around complex asynchronous transformations that need to run with low latency and how safely can be done one large migration of hundreds of millions of objects. There is also explained how to handle the migrations and how to  perform migrations at scale.


What Lambda Architecture tries to solve? 
-----
It is solving data processing that organizations use to combine a traditional batch pipeline with a fast real-time stream pipeline for data access. The Lambda Architecture attempts to balance concerns around latency, data consistency, scalability, fault tolerance, and human fault tolerance, but this architecture has sometimes been criticized as being overly complex. By having each pipeline that requires its own code base, and the code bases must be kept in sync to ensure consistent, accurate results when queries touch both pipelines. Also this architecture can perform re-computation and precomputation. The biggest advantages of the Lambda Arhitecture solves the human error tolerance and tolerance in case of a hardware damage.


What is the critique given against Lambda Architecture? 
-----
If no specific considerations are made, an organization's choice of lambda architecture to prepare for a data lake can have some drawbacks:

1. Big Data Frameworks are hard to code.

2. Different layers of this architecture may make it complex an synchronization between the layers can be an expensive and it has to be handled in a cautious manner.

3. Support and maintenance become difficult because of distinct and distributed layers namely batch and speed, also maintenance of the code of the architecture can be difficult as it has to produce the same results in the distributed system.


Explain the keywords.
-----

1. `CAP theorem` 

CAP stands for Consistency, Availability, and Partition tolerance. 
The theorem states that a distributed system cannot guarantee all three: consistency, availability, and partition tolerance all the time. When things go wrong, it is needed to decide on a trade-off and prioritize at most two characteristics of distributed systems to keep.

2. `MapReduce` 

MapReduce is a software framework and programming model used for processing huge amounts of data.

3. `Migration`

A Migration is the shifting of data or software from one system to another. 


Explain the 4 steps of data migration. 
-----

1. `Dual writing:` writing all the data from the first table to the 2nd one until everything was cloned to the 2nd table, take care to update both the tables.
2. `Changing all read paths:` read from first table, comparing results with the one from 2nd table, read data from the 2 table.
3. `Changing all write paths:` writing all the data to both tables by using reverse order into writing buy keeping both the tables consistent.
4. `Removing old data:` ignore no more used table and moving to new table(the most recently used).


What can we achieve if we follow the pattern for data migration provided in the article?
-----

By following this pattern we can easily achieve the balance concerns around latency, data consistency, scalability, fault tolerance, and human fault tolerance on building an application which uses “big data”.
