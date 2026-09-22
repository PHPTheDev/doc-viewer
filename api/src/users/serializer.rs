



// async fn get_user(Path(user_id) : Path<Uuid>) -> Json<CreateUser> {
//     let user = find_user(user_id).await;
//     Json(user)
// }

// async fn find_user(user_id: usize) -> CreateUser {
//     let connection = sqlite::open("teste.db").unwrap();
//     let query = format!("SELECT * FROM teste WHERE id =");
// }
