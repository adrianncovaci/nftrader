Seminar 5
-----

```
Vector Clocks
An article discussing a way of tracking time in DS.
```

What is the article about? 
-----

This article is about 2 examples of using vclocks and how it works. It also provides some info about pruning and why vclocks could be complicated.


What do vclocks guarantee? 
-----

Integrity of the data.


How do vclocks solve conflicts? 
-----

Makes a vector clock that will be a successor to all previously-seen vector clocks. So taking everything the past two vector clocks have.


Explain the concept of "pruning" a vclock and why we would do that. 
-----

Pruning is utilized within the circumstance where vector clocks will develop and develop as more clients utilize a system over time. This can be done by including a timestamp to each field and upgrading it to the current neighborhood time at whatever point that field is incremented. This timestamp is never utilized for vclovk comparison – that's as it were of pruning purpose.


What other timekeeping tools are used besides vclocks? Describe at least 2 shortly.
-----

Tools for capturing chronological and causal relationships in a distributed system (logic clocks) are:
-	Version vectors, order replicas, according to updates, in an optimistic replicated system.
-	Matrix clocks, an extension of vector clocks that also contains information about other processes’ views of the system.

