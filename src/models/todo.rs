use crate::schema::todos;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub todo_text: String,
    pub time_added: String,
    pub time_finished: String,
    pub finished: bool,
}

impl Todo {
    pub fn get_by_id(id: &i32) -> Result<Todo, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::db_handler::connect;

        let connection = connect();

        todos::table
            .find(id)
            .first(&connection)
    }

    pub fn delete_by_id(id: &i32) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::todos::dsl;
        use crate::db_handler::connect;

        let connection = connect();

        diesel::delete(
            dsl::todos.find(id))
                .execute(&connection)?;
        Ok(())
    }

    pub fn update_by_id(id: &i32, new_todo: &NewTodo) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::todos::dsl;
        use crate::db_handler::connect;

        let connection = connect();
        diesel::update(dsl::todos.find(id))
            .set(new_todo)
            .execute(&connection)?;
        Ok(())
    }
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "todos"]
pub struct NewTodo {
    pub todo_text: String,
    pub time_added: String,
    pub time_finished: Option<String>,
    pub finished: Option<bool>,
}

impl NewTodo {
    pub fn create(&self) -> Result<Todo, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::db_handler::connect;

        let connection = connect();
        diesel::insert_into(todos::table)
            .values(self)
            .get_result(&connection)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList(pub Vec<Todo>);

impl TodoList {
    pub fn get_all() -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::todos::dsl::*;
        use crate::db_handler::connect;

        let connection = connect();

        let result = todos
            .limit(100)
            .load::<Todo>(&connection)
            .expect("Error loading todos");

        TodoList(result)
    }
}