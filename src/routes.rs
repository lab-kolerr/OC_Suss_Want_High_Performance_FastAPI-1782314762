use warp::Filter;

/// Defines the API routes for the Auto application.
pub fn api_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Define a route for getting all items
    warp::path("items")
        .and(warp::get())
        .and_then(get_items)
}

/// Handler for getting all items.
async fn get_items() -> Result<impl warp::Reply, warp::Rejection> {
    // Simulate fetching items from a database
    let items = vec!["Item1", "Item2", "Item3"];
    Ok(warp::reply::json(&items))
}