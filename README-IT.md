# actix-web-sqlx

L'idea di base nella realizzazione di questo progetto nasce dall'idea di utilizzare actix-web come framework di sviluppo principale come soluzione web per le performance che esso offre ma anche quello di arrichirlo con una modalità più simile agli attuali framework di sviluppo più conosciuti quali Java e .NET Framework

Principalmente sono stati usati i seguenti crate:

* actix-web Framework principale di sviluppo
* actix-cors Implementazione della modalità CORS in modo da essere facilmente richiamabile anche da framework javascript
* sqlx Per poter dialogare con il database (sono state usate le transazioni per le operazioni di insert)
* slog Per tenere traccia su dei log dell'entrata, uscita e errori all'interno dei web method
* utoipa Per documentare i metodi rest implementati e per poter usare Swagger UI

Oltre alla configurazione e sviluppo dei create sopra menzionati si sono configurati:

* err_handler Un file che configura in maniera globale gli errori genrali che si verificano nei web method
* security_headers File che arrichisce le risposte con gli header di sicurezza più usati
* manager_logger File che gestiscela configurazione del log

Per utilizzarlo anche e sopratutto in un ottica microservizi si sono aggiunte anche le seguneti funzionalità

* actuator health Per verificare tramite ambiente kubernetes la corretta esecuzione, in questo caso è stato aggiunto il monitoraggio anche al database
* Dockerfile per la creazione di un immagine da questo progetto
* File .env contenente le variabili di sistema usate nel progetto

Per eseguire il progetto è necessario creare la seguente tabella su database Postgress, ovviamente il progetto si può configurare anche per altri database

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




