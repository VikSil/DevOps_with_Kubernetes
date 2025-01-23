# Assignment

> Look at the CNCF Cloud Native Landscape [png](https://landscape.cncf.io/images/landscape.png) (also available as [interactive](https://landscape.cncf.io/))
> 
> Circle the logo of every product / project you've used. It does not have to be in this course. "Used" is defined here as something that you know you were using it. Next use different color to circle those that something we used was depending on, except those already circled. Then create a list with information where they were used. Anything outside of this course context can be labeled as "outside of the course"
> 
> For example:
> 
>  1. I used **HELM** to install Prometheus in part 2.
>  1. I indirectly used **Flannel** as k3d (through k3s) uses it. But I have no clue how it works.
>  1. I've used **Istio** outside of the course.
> You can follow the indirect use as deep as you want, like in the k3d -> k3s -> flannel example, but use common sense to make the final image meaningful.

# Analysis

![CNCF Cloud Native Landscape](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.08/landscape.png)

## Directly used

- I used **Helm** in to install charts for [Exercise 2.10](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part2/Exercise_2.10) and [Exercise 4.03](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part4/Exercise_4.03).
- I've used **Gradle** outside of this course.
- I've used **Cockroach Labs** outside of this course.
- I've used **SQL Server** outside of this course.
- I've used **mongoDB** outside of this course.
- I've used **MySQL** outside of this course.
- I've used **Oracle** outside of this course.
- I used **Postgres** as the database for applications that I developed as part of this course. It was introduced in [Exercise 2.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part2/Exercise_2.07) for Pingpong app and [Exercise 2.08](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part2/Exercise_2.08) for Todo app. 
- I used **redis** as an example when learning about StatefulSets in [Part 2](https://devopswithkubernetes.com/part-2/4-statefulsets-and-jobs#statefulsets).
- I used **Argo** for rollouts in [Exercise 4.04](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part4/Exercise_4.04), [Exercise 4.07](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part4/Exercise_4.07), [Exercise 4.08](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part4/Exercise_4.08) and [Exercise 5.03](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part5/Exercise_5.03).
- I used **GitHub Actions** to automate deployment in Part 3, starting [Exercise 3.03](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part3/Exercise_3.03) as well as [Exercise 4.07](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part4/Exercise_4.07) and [Exercise 4.08](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part4/Exercise_4.08).
- I've used **Jenkins** outside of this course.
- I learnt about **NATS** theoretically in [Part 4](https://devopswithkubernetes.com/part-4/2-messaging-systems) (the exercise was skipped).
- I've used **Kafka** outside of this course.
- I used **Kubernetes** throughout the course.
- I used **Knative** to run serverless deployments in [Exercise 5.06](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part5/Exercise_5.06) and [Exercise 5.07](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part5/Exercise_5.07).
- I used **Linkerd** as service mesh in [Exercise 5.02](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part5/Exercise_5.02) and [Exercise 5.03](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part5/Exercise_5.03).
- I've used **citrix** outside of this course.
- I've used **nginx** as a base image for DummySite resource in [Exercise 5.01](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part5/Exercise_5.01).
- I used **Prometheus** for monitoring the cluster in [Exercise 2.10](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part2/Exercise_2.10) and [Exercise 4.03](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part4/Exercise_4.03).
- I used **Grafana** to view log messages in [Exercise 2.10](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part2/Exercise_2.10)

## Dependencies

- **ArtifactHUB** is a non-mandatory dependency for Helm, where Helm charts can be stored. It was mentioned in the theory  in [Part 4](https://devopswithkubernetes.com/part-4/2-messaging-systems). 
- **Traefik** is the default ingress class that comes with k3s and was implicitly used for majority of the course, except where explicitly disabled in [Exercise 5.06](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.06) and [Exercise 5.07](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part5/Exercise_5.07).
- **etcd** is an underlying technology for Kubernetes that is used to store cluster data.
- **Zookeeper** is a dependancy of Kafka, that I have used outside of the course.
- **Flannel** is a dependency of k3s, as mentioned in the description of this exercise above.
- **Grafana Loki** was used as a front-end for querying Grafana in [Exercise 2.10](https://github.com/VikSil/DevOps_with_Kubernetes/blob/trunk/Part2/Exercise_2.10).
