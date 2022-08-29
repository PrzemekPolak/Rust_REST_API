use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize};
use rand::Rng;
use chrono::Datelike;


#[derive(Deserialize)]
struct CarData {
    distance: u64,
    year_of_production: u64,
    fuel_usage_per_100km: u64,
}

#[get("/calculateDisselUsageForDistance/{distance}/{year_of_production}/{fuel_usage_per_100km}")]
async fn calculate_dissel_usage_for_distance(car_data: web::Path<CarData>) -> impl Responder {
    // Check if date in request is correct
    let current_date = chrono::Utc::now().date();
    if car_data.year_of_production < 2005 || car_data.year_of_production > 2010 {
        return HttpResponse::BadRequest().json("Incorrect date in request. This car was produced from 2005 to 2010.");
    }

    // Calculate fuel usage
    let overtime_lost_in_efficiency = 1.0 + (current_date.year() as f64 - car_data.year_of_production as f64) / 100.0;
    let fuel_usage_per_1km = car_data.fuel_usage_per_100km as f64 / 100.0 * overtime_lost_in_efficiency;
    let fuel_usage = car_data.distance as f64 * fuel_usage_per_1km;

    // Send response as json
    HttpResponse::Ok().json(fuel_usage)
}

#[get("/probabilityOfUnitInjectorFail/{vin}")]
async fn probability_of_unit_injector_fail(path_data: web::Path<String>) -> impl Responder {
    // Unneded variable
    let _vin = path_data.into_inner();

    // Calculate fail probability
    let mut rng = rand::thread_rng();
    let fail_probability = rng.gen_range(0..101) as f64 / 100.0;

    // Send response as json
    HttpResponse::Ok().json(fail_probability)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculate_dissel_usage_for_distance)
            .service(probability_of_unit_injector_fail)
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
