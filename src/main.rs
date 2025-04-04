use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use rand::{Rng};
use rand::distr::Alphanumeric;
use tinyurl_rust::models::*;
use diesel::expression_methods::*; // Import for arithmetic operations
use diesel::{connection, prelude::*};

use tinyurl_rust::schema::url_map::dsl::*;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Tiny Url Rust Server!")
}

#[post("/shorten")]
async fn shorten_url(conn_data: web::Data<std::sync::Mutex<PgConnection>>, req_body: String) -> impl Responder {
    let conn = &mut *conn_data.lock().unwrap();
    
    if let Ok(existing) = url_map
        .filter(original_url.eq(&req_body))
        .first::<UrlMap>(conn)
    {
        return HttpResponse::Ok().body(existing.tiny_url);
    }
    
    let tinyu : String = generate_short_url(conn);

    let new_urls = NewUrl{
        original_url: req_body,
        tiny_url: tinyu.clone(),
        fetch_count: 0
    };

   // Insert into the database with detailed error handling
   match diesel::insert_into(url_map)
   .values(&new_urls)
   .execute(conn)
    {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                println!("No rows inserted - possible schema mismatch or constraint issue");
                return HttpResponse::InternalServerError().body("No rows inserted");
            }
            println!("Successfully inserted {} row(s)", rows_affected);
        }
        Err(e) => {
            println!("Insert failed: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Failed to insert URL: {:?}", e));
        }
    }

    HttpResponse::Ok().body(tinyu)
}

#[get("/retrieve")]
async fn retrieve_url(conn_data: web::Data<std::sync::Mutex<PgConnection>>, req_body: String) -> impl Responder {
    let conn = &mut *conn_data.lock().unwrap();
    
    if let Ok(existing) = url_map
        .filter(tiny_url.eq(&req_body))
        .first::<UrlMap>(conn)
    {
        // match diesel::update(url_map.filter(tiny_url.eq(&req_body)))
        // .set(fetch_count.eq(fetch_count + 1)) // Should work with proper imports
        // .execute(conn)
        // {
        //         Ok(_) => println!("fetch_count incremented for {}", req_body),
        //         Err(e) => println!("Failed to increment fetch_count: {:?}", e),
        //     }

        return HttpResponse::Ok().body(existing.original_url);
    }

    HttpResponse::Ok().body("No Entry found")    
}

#[get("/count")]
async fn fetch_count(conn_data: web::Data<std::sync::Mutex<PgConnection>>, req_body: String) -> impl Responder {
    let conn = &mut *conn_data.lock().unwrap();

    if let Ok(existing) = url_map
        .filter(tiny_url.eq(&req_body))
        .filter(original_url.eq(&req_body))
        .first::<UrlMap>(conn)
    {
        return HttpResponse::Ok().body(existing.fetch_count.to_string());
    }
    
    HttpResponse::Ok().body("0")
}

// Utility functions
fn generate_short_url(conn: &mut PgConnection) -> String {
    loop {
        let random_string: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        
        let short_url = format!("https://rust.tiny.url/{}", random_string);
        
        // Check if the generated URL already exists in the database
        let exists = url_map
            .filter(tiny_url.eq(&short_url))
            .select(tiny_url)
            .first::<String>(conn)
            .is_ok(); // Returns true if found, false if not

        if !exists {
            return short_url; // Return the unique URL and exit the loop
        }
        // If exists is true, loop continues and generates a new random string
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let connection  = tinyurl_rust::establish_connection();
    let conn_data = web::Data::new(std::sync::Mutex::new(connection)); // wrap in Mutex
    // let state = web::Data::new(AppState { connection });


    HttpServer::new(move || {
        App::new()
            .app_data(conn_data.clone()) // clone Arc internally
            .service(hello)
            .service(shorten_url)
            .service(retrieve_url)
            .service(fetch_count)
            // .route("", shorten_url())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}