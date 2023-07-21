# actix-web-sqlx

The basic idea in the realization of this project stems from the idea of ​​using actix-web as the main development framework as a web solution for the performance it offers but also to enrich it with a method more similar to the current best known development frameworks such as Java and .NET Framework

Mainly the following crates were used:

* actix-web Core development framework
* actix-cors Implementation of the CORS mode in order to be easily recallable also from javascript framework
* sqlx To be able to communicate with the database (transactions were used for insert operations)
* slog To keep track of entry, exit and error logs within web methods
* utoipa To document the implemented rest methods and to be able to use Swagger UI

In addition to the configuration and development, the following have been configured:

* err_handler A file that globally configures general errors that occur in web methods
* security_headers File that enriches the responses with the most used security headers
* manager_logger File that manages the log configuration

To use it also and above all in a microservices perspective, the following functions have also been added

* actuator health To verify correct execution through the Kubernetes environment, in this case monitoring was also added to the database
* Dockerfile for creating an image from this project
* File .env containing the system variables used in the project

To run the project it is necessary to create the following table on the Postgress database, obviously the project can also be configured for other databases

``` 
CREATE TABLE public.people (
 id serial4 NOT NULL,
 name varchar NOT NULL,
 surname varchar NOT NULL,
 age int4 NOT NULL,
 created_at timestamp NULL,
 CONSTRAINT people_pk PRIMARY KEY (id)
);
```



## Runnig the Server

To startup the server

`cargo run`



## Docker

To buil a Docker image of the application:

`docker build -t actix-web-sqlx .`

Once the image is built, you can run the container in port 8000:

`docker run -it --rm --env-file=.env.docker -p 8000:8000 --name actix-web-sqlx actix-web-sqlx`




## License
This project is licensed under:

Apache 2




