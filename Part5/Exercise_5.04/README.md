# Assignment

> Write an app that serves Wikipedia pages. The app should contain
> 
> - the main container based on *nginx* image, that just serves whatever content it has in the public www location
> - init container that curls page https://en.wikipedia.org/wiki/Kubernetes and saves the page content to the public www directory for the main container
> - a sidecar container that waits for a random time between 5 and 15 minutes, curls for a random Wikipedia page in URL https://en.wikipedia.org/wiki/Special:Random and saves the page content to the public www directory for the main container
> 
> Hint: you might need to refresh your memory by reading [this](https://devopswithkubernetes.com/part-1/4-introduction-to-storage#volumes) from part 1 of the course.

# Solution

*This exercise was skipped.*