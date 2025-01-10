# Assignment

> Ok, we started up Prometheus in part 2, but we have barely scratched the surface. Let's do a single hands-on query to learn more.
> 
> Start now Prometheus with Helm and use port-forward to access the GUI website. Port 9090 is the default for Prometheus:
> 
>      $ kubectl -n prometheus get pods
>       NAME                                                              READY   STATUS    RESTARTS   AGE
>        alertmanager-kube-prometheus-stack-1714-alertmanager-0            2/2     Running   0          3h19m
>        kube-prometheus-stack-1714-operator-94c596dbd-n5pcl               1/1     Running   0          3h19m
>        kube-prometheus-stack-1714644114-grafana-54cbbc4c46-m26f6         3/3     Running   0          3h19m
>        kube-prometheus-stack-1714644114-kube-state-metrics-7cb796tpjbd   1/1     Running   0          3h19m
>        kube-prometheus-stack-1714644114-prometheus-node-exporter-kdpln   1/1     Running   0          3h19m
>        kube-prometheus-stack-1714644114-prometheus-node-exporter-sp9pg   1/1     Running   0          3h19m
>        kube-prometheus-stack-1714644114-prometheus-node-exporter-vbbjk   1/1     Running   0          3h19m
>        prometheus-kube-prometheus-stack-1714-prometheus-0                2/2     Running   0          3h19m
>        
>        $ kubectl -n prometheus port-forward prometheus-kube-prometheus-stack-1714-prometheus-0 9090:9090
>        Forwarding from 127.0.0.1:9090 -> 9090
>        Forwarding from [::1]:9090 -> 9090
>
> And now accessing http://localhost:9090 will allow us to write queries.
> 
> **Write a query** that shows the number of pods created by StatefulSets in *prometheus* namespace. For the above setup the *Value* should be 3 different pods:
> 
> ![prometheus p4](https://devopswithkubernetes.com/static/9295ed50207655f31d8c229e0e61375b/1a867/prometheus-p4.png)
>
> Query for "kube_pod_info" should have the required fields to filter through. See [documentation](https://prometheus.io/docs/prometheus/latest/querying/basics/) for help with querying.

# Solution

![Commands for Exercise 4.03](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.03/Exercise_4.03_commands.png)

![Commands for Exercise 4.03](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.03/Exercise_4.03_commands2.png)

![Query for Exercise 4.03](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part4/Exercise_4.03/Exercise_4.03_frontend.png)
