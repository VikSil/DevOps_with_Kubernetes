# Assignment

> Install Knative Serving component to your k3d cluster.
> 
> For Knative to work locally in k3d you need to create it a cluster without Traefik:
> 
>       $ k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2 --k3s-arg  "--disable=traefik@server:0"
> 
> Follow then [this](https://knative.dev/docs/install/yaml-install/serving/install-serving-with-yaml/) guide.
> 
> You might end up in a situation like this in the step *verify the installation*:
> 
>       $ get pods -n knative-serving
>       NAME                                      READY   STATUS             RESTARTS      AGE
>       activator-67855958d-w2ws8                 0/1     Running            0             64s
>       autoscaler-5ff4c5d679-54l28               0/1     Running            0             64s
>       webhook-5446675b97-2ngh6                  0/1     CrashLoopBackOff   3 (12s ago)   64s
>       net-kourier-controller-58b6bf4fbc-g7dlp   0/1     CrashLoopBackOff   3 (10s ago)   55s
>       controller-6d8b579f9-p42dx                0/1     CrashLoopBackOff   3 (6s ago)    64s
> 
> See the logs of a crashing pod to see how to fix the problem.
> 
> Next, try out the examples in [Deploying a Knative Service](https://knative.dev/docs/getting-started/first-service/), [Autoscaling](https://knative.dev/docs/getting-started/first-autoscale/) and [Traffic splitting](https://knative.dev/docs/getting-started/first-traffic-split/).
> 
> Note you can access the service from the host machine as follows:
> 
>       curl -H "Host: hello.default.192.168.240.3.sslip.io" http://localhost:8081
>
> Where *Host* is the URL you get with the following command:
> 
>       kubectl get ksvc


# Solution

## Commands

### Installing Knative Serving

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands2.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands3.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands4.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands5.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands6.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands7.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands8.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands9.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands10.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands11.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands12.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands17.png)

### Installing Knative CLI

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands14.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands15.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands16.png)


### Deploying a Knative Service

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands13.png)

### Autoscaling

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands18.png)

### Traffic splitting

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands19.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands20.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands21.png)

![Commands for Exercise 5.06](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part5/Exercise_5.06/Exercise_5.06_commands22.png)