pub trait Repository {
    pub async fn add(&self, batch: model::batch) -> Result<(),Error>{
        not_implemented!()
    };
    pub async fn get(&self, reference: String) -> Result<Option<model::batch>,Error>{
        not_implemented!()
    };
}

pub trait comRepository<T> {
    pub async fn add(&self, item: T) -> Result<(),Error>{
        not_implemented!()
    };
    pub async fn sync<S: AsRef<str>>(&self, item: T, id: S) -> Result<(),Error>{
        not_implemented!()
    };
    pub async fn delete<S: AsRef<str>>(&self, id: S) -> Result<(),Error>{
        not_implemented!()
    };
    pub async fn get<S: AsRef<str>>(&self, id: S) -> Result<Option<T>,Error>{
        not_implemented!()
    };
    pub async fn list(&self) -> Result<Vec<T>,Error>{
        not_implemented!()
    };
}