## Assignment

> 
> **Exercises can be done with any language and framework you want.**
> 
> Create an application that generates a random string on startup, stores this string into memory, and outputs it every 5 seconds with a timestamp. e.g.
> 
>       2020-03-30T12:15:17.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43   
>       2020-03-30T12:15:22.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43
> 
> Deploy it into your Kubernetes cluster and confirm that it's running with `kubectl logs ...`
> 
> You will keep building this application in the future exercises. This application will be called "Log output".

## Solution

- Application was built in Rust. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.01/app).
- Image was pushed to Docker Hub repo [viksil/log_output](https://hub.docker.com/r/viksil/log_output/tags?name=1.01).
- The following commands were used to create and test Kubernetes deployment:

![Deployment for Exercise 1.01](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.01/Exercise_1.01.png)
