use crate::{
    schema::{FilterOptions},
    AppState,
};
use actix_web::{delete, get, put, post, web, HttpResponse, Responder};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::json;
use sqlx::postgres::{PgRow};
use sqlx::{Acquire, Row};
use slog::{error, info};
use crate::model::{PeopleModel, PeopleModelView};

#[utoipa::path(
    path = "/api/people",
    responses(
        (status = 200, description = "List of people", body = [PeopleModel])
    )
)]
#[get("/people")]
pub async fn people_list_handler(opts: web::Query<FilterOptions>,data: web::Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    info!(data.log, "Enter in get list people");

    let peoples: Vec<PeopleModel> = sqlx::query_as!(
        PeopleModel,
        r#"select * from people limit $1 offset $2"#,
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await
        .unwrap();

    let people_responses = peoples
        .into_iter()
        .map(|people| {
            PeopleModelView {
                id: people.id.to_owned(),
                name: people.name.to_owned(),
                surname: people.surname.to_owned(),
                age: people.age.to_owned()
            }
        })
        .collect::<Vec<PeopleModelView>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": people_responses.len(),
        "peoples": people_responses
    });

    info!(data.log, "Exit in get list people");
    HttpResponse::Ok().json(json_response)
}

#[utoipa::path(
    path = "/api/people/{id}",
    responses(
        (status = 200, description = "List of people", body = PeopleModel),
        (status = 404, description = "People was not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/people/{id}")]
pub async fn get_people_handler(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    info!(data.log, "Enter in get people");
    let people_id = path.into_inner().to_owned();
    let query_result = sqlx::query_as!(PeopleModel, r#"SELECT * FROM people WHERE id = $1"#, people_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(people) => {
            let people_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "people": PeopleModelView {
                    id: people.id.to_owned(),
                    name: people.name.to_owned(),
                    surname: people.surname.to_owned(),
                    age: people.age.to_owned()
                }
            })});
            info!(data.log, "Exit in get people");
            return HttpResponse::Ok().json(people_response);
        }
        Err(sqlx::Error::RowNotFound) => {
            error!(data.log, "People not found");
            return HttpResponse::NotFound().json(
                serde_json::json!({"status": "fail","message": format!("People with ID: {} not found", people_id)}),
            );
        }
        Err(e) => {
            error!(data.log, "Generic error");
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };
}

#[utoipa::path(
    path = "/api/people",
    request_body = PeopleModelView,
    responses(
        (status = 200, description = "Return the primary key created", body = i32),
        (status = 400, description = "Duplicate entry"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/people")]
async fn create_people_handler(body: web::Json<PeopleModelView>,data: web::Data<AppState>) -> impl Responder {
    info!(data.log, "Enter in create people");
    let dt: DateTime<Utc> = Utc::now();
    let ndt: NaiveDateTime = dt.naive_utc();

    let mut conn = data.db.acquire().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let query_result: Result<PgRow, String> =

        sqlx::query(r#"INSERT INTO people (name, surname, age, created_at) VALUES ($1, $2, $3, $4) RETURNING id"#)
            .bind( body.name.to_string())
            .bind(body.surname.to_string())
            .bind(body.age)
            .bind(ndt)
            //.fetch_one(&data.db)
            .fetch_one(&mut tx)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    tx.commit().await.unwrap();

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            error!(data.log, "People already exist");
            return HttpResponse::BadRequest().json(
                serde_json::json!({"status": "fail","message": "People already exists"}),
            );
        }

        error!(data.log, "Internal server error");
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    info!(data.log, "Exit in create people");
    let id: i32 = query_result.unwrap().get(0);
    HttpResponse::Ok().body(id.to_string())
}


#[utoipa::path(
    path = "/api/people/{id}",
    request_body = PeopleModelView,
    responses(
        (status = 200, description = "Success update"),
        (status = 404, description = "People not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id", description = "Unique storage id of people")
    ),
)]
#[put("/people/{id}")]
async fn edit_people_handler(path: web::Path<i32>, body: web::Json<PeopleModelView>, data: web::Data<AppState>) -> impl Responder {
    info!(data.log, "Enter in update people");
    let people_id = path.into_inner().to_owned();
    let query_result = sqlx::query_as!(PeopleModel, r#"SELECT * FROM people WHERE id = $1"#, people_id)
        .fetch_one(&data.db)
        .await;

    let _people = match query_result {
        Ok(people) => people,
        Err(sqlx::Error::RowNotFound) => {
            error!(data.log, "People not found");
            return HttpResponse::NotFound().json(
                serde_json::json!({"status": "fail","message": format!("People with ID: {} not found", people_id)}),
            );
        }
        Err(e) => {
            error!(data.log, "Internal server error");
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    let update_result = sqlx::query(
        r#"UPDATE people SET name = $1, surname = $2, age = $3 WHERE id = $4"#,
    )
        .bind(body.name.to_owned())
        .bind(body.surname.to_owned())
        .bind(body.age.to_owned())
        .bind(people_id.to_owned())
        .execute(&data.db)
        .await;

    match update_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("People with ID: {} not found", people_id);
                return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
            } else {
                return HttpResponse::Ok().json(json!({"status": "success"}));
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);

            error!(data.log, "Internal server error");
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": message}));
        }
    }
}

#[utoipa::path(
    path = "/api/people/{id}",
    responses(
        (status = 200, description = "People deleted successfully"),
        (status = 404, description = "People not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/people/{id}")]
async fn delete_people_handler(path: web::Path<i32>,data: web::Data<AppState>) -> impl Responder {
    info!(data.log, "Enter in delete people");
    let people_id = path.into_inner().to_owned();
    let query_result = sqlx::query!(r#"DELETE FROM people WHERE id = $1"#, people_id)
        .execute(&data.db)
        .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!(data.log, "People not found");
                let message = format!("People with ID: {} not found", people_id);
                HttpResponse::NotFound().json(json!({"status": "fail","message": message}))
            } else {
                info!(data.log, "Exit in delete people");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => {
            error!(data.log, "Internal server error");
            let message = format!("Internal server error: {}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error","message": message}))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(people_list_handler)
        .service(get_people_handler)
        .service(create_people_handler)
        .service(edit_people_handler)
        .service(delete_people_handler);
    conf.service(scope);
}