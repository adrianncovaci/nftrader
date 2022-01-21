Seminar 2
----
```
Monitoring DS
Learn from Google how they monitor their services.
```

What is the paper about? 
----

This article is about focusing on system monitoring, analysis, how to use it and pitfalls


What is monitoring? 
----

It is about collecting and aggregating quantitative data about the system, such as query time, service load, etc. 


Why monitor a system in the first place? 
----

Monitoring is very critical since it permits us distinguish mistakes in framework communication and recognizing bottleneck administrations and permits understanding how to scale it legitimately, utilizing observational information.


Explain the 4 golden signals of monitoring. 
----

1. `Latency` - a bug might result in expanded time of reaction.
2. `Traffic` - recognizing either sum of demands or the sum of information exchanged in gushing associations.
3. `Errors` - abnormal rate of blunders might show a destitute quality of benefit.
4. `Saturation` - calculating the stack on the benefit may well be characteristic approximately scaling openings, especialy this would be valuable for informing approximately database running out of space.

According to the paper, how do you do monitoring? What is important? Exemplify. 
----

It’s critical to choose and assess information. A common entanglement, for case, would be the normal out the time per ask in a window of 5 seconds. Usually an opportunity for long handling time to cover up between numerous brief ones. distant better. An improved approach would be to classify the into time ranges and tally how numerous were between [0.1 and 0.3], [0.3 and 0.7] and so on.


What approach would you use for your lab: White-box or Black-box monitoring? Why? 
----

A good solution is to combinae the two of them. It's good to have summary of availability of the system for the client (black-box) and image of what's happening inside (white-box).


What happened with Bigtable SRE and how did they "fix" the situation?
----

Mail alarms were activated as the SLO drawn closer, and paging alarms were activated when the SLO was surpassed. Both sorts of alarms were terminating voluminously, expending unsatisfactory sums of building time: the group went through noteworthy sums of time triaging the cautions to discover the few that were truly noteworthy, and we frequently missed the issues that really influenced clients, since so few of them did. To cure the circumstance, the group utilized a three-pronged approach: whereas making extraordinary endeavors to move forward the execution of Bigtable, we moreover briefly dialed back our SLO target, utilizing the 75th percentile ask idleness. They moreover crippled e-mail cautions, as there were so numerous that investing time diagnosing them was infeasible.

