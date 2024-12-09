## Assignment

> 
> **Project can be done with any language and framework you want**
> 
> The project will be a simple todo application with the familiar features of create, read, update, and delete (CRUD). We'll develop it during all parts of this course. Check the title of the exercise for "Project vX.Y" to know it is about building the project.
> 
> Todo is a text like "I need to clean the house" that can be in state of not-done or done.
> 
> ![Project evolution](https://devopswithkubernetes.com/static/42cd00819d1db5789826b38bb7f95328/2430e/project.webp)
> 
> Dashed lines separate major differences across the course. Some exercises are not included in the picture. The connections between most pods are not included as well. You are free to do them however you want.
> 
> Keep this in mind if you want to avoid doing more work than necessary.
> 
> Let's get started!
> 
> Create a web server that outputs "Server started in port NNNN" when it is started and deploy it into your Kubernetes cluster. Please make it so that an environment variable PORT can be used to choose that port. You will not have access to the port when it is running in Kubernetes yet. We will configure the access when we get to networking.

## Solution

- Application was built in Rust. Source code can be found [here](https://github.com/VikSil/DevOps_with_Kubernetes/tree/trunk/Part1/Exercise_1.02/app).
- Image was pushed to Docker Hub repo [viksil/todo_app:1.02](https://hub.docker.com/repository/docker/viksil/todo_app/tags?name=1.02).
- The following commands were used to create and test Kubernetes deployment:

![Deployment for Exercise 1.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.02/Exercise_1.02_deployment.png)

![Landing Page for Exercise 1.02](https://raw.githubusercontent.com/VikSil/DevOps_with_Kubernetes/refs/heads/trunk/Part1/Exercise_1.02/Exercise_1.02_landing_page.png)
