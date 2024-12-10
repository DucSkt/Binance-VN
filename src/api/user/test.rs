// #[tokio::test]
// async fn test_create_user_api_unit_test() {
//     // Thiết lập database test
//     let pool = setup_test_db().await;
//
//     // Tạo router với route để test
//     let app = Router::new()
//         .route("/api/users", axum::routing::post(create_user_handler))
//         .layer(axum::AddExtensionLayer::new(pool));
//
//     // Tạo request POST giả
//     let response = app
//         .oneshot(
//             Request::builder()
//                 .method("POST")
//                 .uri("/api/users")
//                 .header("content-type", "application/json")
//                 .body(Body::from(
//                     r#"{"name": "John Doe", "email": "john@example.com"}"#,
//                 ))
//                 .unwrap(),
//         )
//         .await
//         .unwrap();
//
//     // Kiểm tra kết quả
//     assert_eq!(response.status(), StatusCode::CREATED);
// }
//
// #[tokio::test]
// async fn test_create_user_api_integration() {
//     // Thiết lập database test
//     let pool = setup_test_db().await;
//
//     // Tạo router với route để test
//     let app = Router::new()
//         .route("/api/users", axum::routing::post(create_user_handler))
//         .layer(axum::AddExtensionLayer::new(pool));
//
//     // Tạo request POST giả
//     let response = app
//         .oneshot(
//             Request::builder()
//                 .method("POST")
//                 .uri("/api/users")
//                 .header("content-type", "application/json")
//                 .body(Body::from(
//                     r#"{"name": "John Doe", "email": "john@example.com"}"#,
//                 ))
//                 .unwrap(),
//         )
//         .await
//         .unwrap();
//
//     // Kiểm tra kết quả
//     assert_eq!(response.status(), StatusCode::CREATED);
// }
