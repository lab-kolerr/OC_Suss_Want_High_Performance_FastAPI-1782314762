use std::collections::HashMap;

fn main() {
    let users = vec![User { id: 1 }, User { id: 2 }];
    let orders = vec![Order { user_id: 1 }, Order { user_id: 2 }, Order { user_id: 1 }];
    let mut results = Vec::new();

    // Before (O(n²))
    // for user in &users {
    //     for order in &orders {
    //         if order.user_id == user.id {
    //             results.push(order.clone());
    //         }
    //     }
    // }

    // After (O(n) with hash map)
    let mut orders_by_user: HashMap<i32, Vec<&Order>> = HashMap::new();
    for order in &orders {
        orders_by_user.entry(order.user_id).or_insert(Vec::new()).push(order);
    }
    for user in &users {
        if let Some(user_orders) = orders_by_user.get(&user.id) {
            results.extend(user_orders.iter().cloned());
        }
    }

    println!("Results: {:?}", results);
}

#[derive(Debug, Clone)]
struct User {
    id: i32,
}

#[derive(Debug, Clone)]
struct Order {
    user_id: i32,
}