# Assignment

> Do a pros/cons comparison of the solutions [Database as a Service vs PersistentVolumeClaims] in terms of meaningful differences. This includes **at least** the required work and costs to initialize as well as the maintenance. Backup methods and their ease of usage should be considered as well.
> 
> Write your answer in the README of the project.

# Comparison

Criteria | DBaaS | PVC |
----------|-----------------|--------------|
Work to initialise | GUI wizzard. Few minutes to half an hour of work. [5] | Provision .yaml files and run them in manually or via kustomize. Several hours of work. |
Cost to initialise | None - $300 in free trial credit. [1] | None - $300 in free trial credit. [1] |
Work to maintain  | Minimal, managed by Google. | Ad-hoc, depending on the needs, fully managed by the user. |
Cost to maintain  | **Enterprise edition**:<br> - compute from $0.0413 per vCPU/hour <br> - up to 824 GB memory, from 0.0091 per GB/hour<br> **Enterprise Plus edition**: <br> - compute from $0.05369 per vCPU/hour <br> - up to 624 GB memory, from 0.007 per GB/hour <br> Storage: $0.17 per GB/month independent of service. <br> Charged as a separate service. <br> Per second billing. [1][4]<br> Commited use discount available. [2][4]| *In London (europe-west-2) for default pods*<br>**Reglar**:<br> - compute at $0.0573 per vCPU/hour <br> - memory at $0.0063421 per GB/hour <br> **Spot**:<br> - compute at $0.0172 per vCPU/hour <br> - memory at $0.0019026 per GB/hour <br> Storage: $0.0001789 per GB/hour independent of plan. <br>  Hourly, monthly, yearly and 3-year price plans available. <br> Price differs slightly based on the region. <br> Charge rolled into the GKE cluster usage bill. [6] |
Backup | Fully automated, out of the box solution. [1] | Backup has to be provisioned manually. |
Availability | Enterprise edition: 99.95% availability and < 60s of planned downtime. <br> Enterprise Plus edition: 99.99% availability and < 10 seconds of planned downtime. <br> Failover across zones. [1] | 99.5% to 99.99% availability, depending on the cluster type and region. [7] |
Scalability | Instant autoscaling. [1] | Scaling based on provisions in setup .yaml files |
Need for expertise | Minimal, GUI setup and management. | Advanced, write your own code and config. |
Control over setup | Within the limits of offered configuration options of the packaged solutions. Still fairly flexible. | Full control over your own code and setup. |
AI support | Integrated Gemini, Vector Search, LangChain [1], in addition to any 3rd party containerised solutions. | Integrated Gemini, in addition to any 3rd party containerised solutions. |
Database support | MySQL, PostgreSQL, SQL Server, Firebase, AlloyDB [1] and partner solutions. [5] | Any containerised database. |

# Sources

1. https://cloud.google.com/sql
2. https://cloud.google.com/blog/products/databases/cloud-sql-database-instances-now-discounted
3. https://cloud.google.com/blog/products/ai-machine-learning/rag-with-databases-on-google-cloud
4. https://cloud.google.com/sql/pricing
5. https://www.youtube.com/watch?v=q6noaMAnk5s
6. https://cloud.google.com/kubernetes-engine/pricing
7. https://cloud.google.com/kubernetes-engine/sla?hl=en