#![allow(unused, dead_code)]

use core::num;
use std::collections::HashMap;
use std::env::{self, Args};

use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash)] //піддивилась
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug, Clone)]
struct CustomerOrder {
    ordered: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(ordered: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            ordered,
            quantity,
            shipped,
        }
    }
}
#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 2, true),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Fridge, 10, false),
        CustomerOrder::new(Product::Toaster, 4, false),
        CustomerOrder::new(Product::Microwave, 1, false),
        CustomerOrder::new(Product::Blender, 4, true),
    ];
    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];
    let input = io::stdin();

    let blenders = orders
        .iter()
        .filter(|order| order.ordered == Product::Blender)
        .collect::<Vec<&CustomerOrder>>();
    println!("{blenders:?}"); //абсолютно сама

    /*let microwaves = orders
            .iter()
            .filter(|order| order.ordered == Product::Microwave)
            .map(|order| order.quantity)
            .sum::<u32>();
    */
    let microwaves = orders
        .iter()
        .filter_map(|order| {
            if order.ordered == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("{microwaves:?}"); //чистила чатом; filter_map абсолютно сама

    //unshipped HashMap
    let unshipped = orders.iter().fold(HashMap::new(), |mut product, sum| {
        if !sum.shipped {
            product
                .entry(sum.ordered.clone())
                .and_modify(|q| *q += sum.quantity)
                .or_insert(sum.quantity);
        }
        product
    });

    if let Some(order) = orders.iter_mut().find(|order| order.shipped == false) {
        order.shipped = true;
    } // з чатом

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprint!("there was an error: cargo run -- <quantity>");
        return;
    }
    let quantity: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Enter valid number");
            return;
        }
    }; // чвт

    let boss_input: Vec<CustomerOrder> = orders
        .iter()
        .filter(|order| order.quantity >= quantity)
        .cloned()
        .collect();

    println!("{:#?}", boss_input); // з чатом

    //though one
    let mut customers = orders
        .into_iter()
        .zip(customer_ids_by_order)
        .fold(HashMap::new(), |mut ids_to_orders, (order, customer_id)| {
            let mut orders = { ids_to_orders.entry(customer_id).or_insert(vec![]) };
            orders.push(order);
            ids_to_orders
        })
        .into_iter()
        .map(|(id, orders)| Customer { id, orders })
        .collect::<Vec<Customer>>();

    customers.sort_by_key(|customer| customer.id);

    println!("{customers:#?}")
}
