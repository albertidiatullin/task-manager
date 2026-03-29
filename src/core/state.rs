use std::sync::Arc;
use crate::databace::db_connect::db_connect;
use crate::repository::user::UserRepository;
use crate::repository::session::SessionRepository;
use crate::repository::tasks::TaskRepository;
use crate::services::session::SessionService;
use crate::services::tasks::TaskService;
use crate::services::user::UserService;
#[derive(Clone)]
pub struct AppState{
    pub user_service:UserService,
    pub session_repo:SessionRepository,
    pub task_service:TaskService,
    pub session_service:SessionService,
}

pub async  fn get_state() -> anyhow::Result<Arc<AppState>>{

    let db = db_connect().await?;
    let user_repo = UserRepository{
        sqlite_pool:db.clone()
    };
    let session_repo = SessionRepository 
    { db_pool:db.clone()
    };

    let task_repo = TaskRepository{
        sqlite_pool:db.clone()
    };

    let task_service = TaskService{
        task_repo,
        user_repo:user_repo.clone()
    };

    let session_service = SessionService{
            session_repo:session_repo.clone(),
            user_repo:user_repo.clone()
        };
    let app_state = AppState{
        user_service:UserService { user_repo,
                                    session_repo },
        session_repo:SessionRepository { db_pool:db.clone()},
        task_service,
        session_service
        };

    let new_state = Arc::new(app_state);
    Ok(new_state)
    }


