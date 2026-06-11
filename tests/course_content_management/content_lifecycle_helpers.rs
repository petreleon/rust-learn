struct CourseContentFixture {
    pool: DbPool,
    course: Course,
    other_chapter: Chapter,
    teacher_token: String,
    student_token: String,
    outsider_token: String,
}

async fn setup_course_content_fixture() -> CourseContentFixture {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let teacher = create_test_user(&mut conn, "teacher_content").await;
    let student = create_test_user(&mut conn, "student_content").await;
    let outsider = create_test_user(&mut conn, "outsider_content").await;

    let new_course = NewCourse {
        title: unique_string("CourseWithContent"),
        description: None,
        topics: None,
        prerequisites: None,
    };
    let course = diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result::<Course>(&mut conn)
        .await
        .unwrap();
    let other_course = diesel::insert_into(courses::table)
        .values(&NewCourse {
            title: unique_string("OtherContentCourse"),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result::<Course>(&mut conn)
        .await
        .unwrap();
    let other_chapter = diesel::insert_into(chapters::table)
        .values(NewChapter {
            course_id: other_course.id,
            title: "Other course chapter".to_string(),
            order: 1,
        })
        .get_result::<Chapter>(&mut conn)
        .await
        .unwrap();

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;

    CourseContentFixture {
        pool,
        course,
        other_chapter,
        teacher_token: generate_token(teacher.id()),
        student_token: generate_token(student.id()),
        outsider_token: generate_token(outsider.id()),
    }
}

fn assert_forbidden_response<B>(
    resp: Result<actix_web::dev::ServiceResponse<B>, actix_web::Error>,
    success_message: &str,
) {
    match resp {
        Ok(r) => {
            if r.status().is_success() {
                panic!("{}", success_message);
            }
            assert_eq!(r.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
        Err(e) => {
            let r = e.error_response();
            assert_eq!(r.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }
}
