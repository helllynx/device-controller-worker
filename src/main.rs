mod api;
mod hardware_info;

#[tokio::main]
async fn main() {
    let location = hardware_info::get_location().await.unwrap();
    println!("{}", location.text().await.ok().unwrap())

    // api::send_mmm(
    //     hardware_info::generate_uuid(),
    //     &hardware_info::mem_usage(),
    //     &hardware_info::cpu_avg(),
    //     location.unwrap().text()
    // )
}

#[cfg(test)]
mod tests {
    use crate::hardware_info;

    #[test]
    fn exploration() {
        // assert_eq!(2 + 2, 4);
        let res  = hardware_info::get_location();
        println!("{} {}",res.0, res.1)
    }
}