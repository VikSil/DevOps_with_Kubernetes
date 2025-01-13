# Assignment

> *If you use JavaScript, notice that the [example app](https://github.com/kubernetes-hy/material-example/tree/master/app9) uses nats.js library version 1.5. The [current version](https://www.npmjs.com/package/nats) of the library has significant changes in the API, so copy-pasting the code from the example will not work.*
> 
> Create a new separate service for sending status messages of the todos to a popular chat application. Let's call the new service "broadcaster".
> 
> Requirements:
> 
> 1. The backend saving or updating todos should send a message to NATS
> 2. The broadcaster should subscribe to NATS messages
> 3. The broadcaster should send the message forward to **an external service** in a format they support
> 
> As the external service you can choose either:
> 
>  - Discord (you can use the course Full stack Discord, see [here](https://fullstackopen.com/en/part11/expanding_further#exercise-11-18) for the details)
>  - Telegram
>  - Slack
>
> or if you don't want to use them, use "Generic" where a URL is set as an Environment variable and the payload is e.g.
> 
>       {
>           "user": "bot",
>           "message": "A todo was created"
>       }
> The broadcaster should be able to be scaled without sending the message multiple times. Test that it can run with 6 replicas without issues. The messages only have to be sent to the external service if all of the services are working correctly. So a randomly missing message is not an issue but a duplicate is.
> 
> Example of a working broadcaster: 
>
> ![Example of a working broadcaster](https://devopswithkubernetes.com/373b4b99e844fb5340312e7460d81ccf/ex406-solution.gif)
> 
> You should not write the API key in plain text.

# Solution

*This exercise was skipped.*