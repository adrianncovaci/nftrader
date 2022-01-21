Seminar 4
----

```
Sharding Pinterest
An article about how the engineers at Pinterest have scaled their MYSQL fleet.
```

What is the article about? 
----

The article is about a technique for data storage scaling.


What requirements and design philosophies influenced the final solution and What is the relation between MySQL instance, database and shard?
----

The prerequisites were that the framework ought to be adaptable, steady, without working overhead, profoundly open, best exertion upgrade. Their arrangement, consdiering that fundamentally MySQL occurrence may be a single database and it was a scaling bottleneck - they begun a few db servers running one SQL occasion each, which would have information conveyed over with record extending. They would part up tables in shards, which would be stored on distinctive servers. They have an awfully flawless execution of UUID so that the id takes after the IP address - a combination of information sort id, id in nearby table and shard ID.


What was ZooKeeper used for? 
----

ZooKeeper is used for mapping shard ID to a database, where it can see up the table recorded in information sort id and the push from local id.
