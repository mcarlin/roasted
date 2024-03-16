// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod bean
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertBeanParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,> { pub name: T1,pub description: T2,pub ts: time::OffsetDateTime,pub region: T3,pub grade: T4,}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct Bean
{ pub bean_id : i32,pub name : String,pub description : Option<String>,pub ts : time::OffsetDateTime,pub region : Option<String>,pub grade : Option<String>,}pub struct BeanBorrowed<'a> { pub bean_id : i32,pub name : &'a str,pub description : Option<&'a str>,pub ts : time::OffsetDateTime,pub region : Option<&'a str>,pub grade : Option<&'a str>,}
impl<'a> From<BeanBorrowed<'a>> for Bean
{
    fn from(BeanBorrowed { bean_id,name,description,ts,region,grade,}: BeanBorrowed<'a>) ->
    Self { Self { bean_id,name: name.into(),description: description.map(|v| v.into()),ts,region: region.map(|v| v.into()),grade: grade.map(|v| v.into()),} }
}pub struct BeanQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> BeanBorrowed,
    mapper: fn(BeanBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> BeanQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(BeanBorrowed) -> R) ->
    BeanQuery<'a,C,R,N>
    {
        BeanQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params)
        .await?.map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct InsertBean
{ pub bean_id : i32,pub name : String,pub description : String,pub ts : time::OffsetDateTime,pub region : String,pub grade : String,}pub struct InsertBeanBorrowed<'a> { pub bean_id : i32,pub name : &'a str,pub description : &'a str,pub ts : time::OffsetDateTime,pub region : &'a str,pub grade : &'a str,}
impl<'a> From<InsertBeanBorrowed<'a>> for InsertBean
{
    fn from(InsertBeanBorrowed { bean_id,name,description,ts,region,grade,}: InsertBeanBorrowed<'a>) ->
    Self { Self { bean_id,name: name.into(),description: description.into(),ts,region: region.into(),grade: grade.into(),} }
}pub struct InsertBeanQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> InsertBeanBorrowed,
    mapper: fn(InsertBeanBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> InsertBeanQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(InsertBeanBorrowed) -> R) ->
    InsertBeanQuery<'a,C,R,N>
    {
        InsertBeanQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params)
        .await?.map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn all_beans() -> AllBeansStmt
{ AllBeansStmt(cornucopia_async::private::Stmt::new("select bean_id, name, description, ts, region, grade
from core.bean")) } pub struct
AllBeansStmt(cornucopia_async::private::Stmt); impl AllBeansStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> BeanQuery<'a,C,
Bean, 0>
{
    BeanQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { BeanBorrowed { bean_id: row.get(0),name: row.get(1),description: row.get(2),ts: row.get(3),region: row.get(4),grade: row.get(5),} }, mapper: |it| { <Bean>::from(it) },
    }
} }pub fn find_bean_by_id() -> FindBeanByIdStmt
{ FindBeanByIdStmt(cornucopia_async::private::Stmt::new("select bean_id, name, description, ts, region, grade
from core.bean
where bean_id = $1")) } pub struct
FindBeanByIdStmt(cornucopia_async::private::Stmt); impl FindBeanByIdStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
bean_id: &'a i32,) -> BeanQuery<'a,C,
Bean, 1>
{
    BeanQuery
    {
        client, params: [bean_id,], stmt: &mut self.0, extractor:
        |row| { BeanBorrowed { bean_id: row.get(0),name: row.get(1),description: row.get(2),ts: row.get(3),region: row.get(4),grade: row.get(5),} }, mapper: |it| { <Bean>::from(it) },
    }
} }pub fn insert_bean() -> InsertBeanStmt
{ InsertBeanStmt(cornucopia_async::private::Stmt::new("insert into core.bean (name, description, ts, region, grade)
values ($1, $2, $3, $4, $5)
returning
    bean_id,
    name,
    description,
    ts,
    region,
    grade")) } pub struct
InsertBeanStmt(cornucopia_async::private::Stmt); impl InsertBeanStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
name: &'a T1,description: &'a T2,ts: &'a time::OffsetDateTime,region: &'a T3,grade: &'a T4,) -> InsertBeanQuery<'a,C,
InsertBean, 5>
{
    InsertBeanQuery
    {
        client, params: [name,description,ts,region,grade,], stmt: &mut self.0, extractor:
        |row| { InsertBeanBorrowed { bean_id: row.get(0),name: row.get(1),description: row.get(2),ts: row.get(3),region: row.get(4),grade: row.get(5),} }, mapper: |it| { <InsertBean>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
InsertBeanParams<T1,T2,T3,T4,>, InsertBeanQuery<'a, C,
InsertBean, 5>, C> for InsertBeanStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertBeanParams<T1,T2,T3,T4,>) -> InsertBeanQuery<'a, C,
    InsertBean, 5>
    { self.bind(client, &params.name,&params.description,&params.ts,&params.region,&params.grade,) }
}}pub mod roast
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(Clone,Copy, Debug)] pub struct InsertRoastParams<> { pub beanId: i32,pub roastLevelId: i32,pub ts: time::OffsetDateTime,}#[derive( Debug)] pub struct InsertRoastStepParams<T1: cornucopia_async::StringSql,> { pub roastId: i32,pub stepOrder: i32,pub description: T1,pub ts: time::OffsetDateTime,pub fanSpeed: i32,pub tempSetting: i32,pub temperature: rust_decimal::Decimal,}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct AllRoasts
{ pub roast_id : i32,pub bean_id : i32,pub roast_ts : time::OffsetDateTime,pub roast_level_id : i32,pub roast_level_name : String,pub roast_level_description : String,}pub struct AllRoastsBorrowed<'a> { pub roast_id : i32,pub bean_id : i32,pub roast_ts : time::OffsetDateTime,pub roast_level_id : i32,pub roast_level_name : &'a str,pub roast_level_description : &'a str,}
impl<'a> From<AllRoastsBorrowed<'a>> for AllRoasts
{
    fn from(AllRoastsBorrowed { roast_id,bean_id,roast_ts,roast_level_id,roast_level_name,roast_level_description,}: AllRoastsBorrowed<'a>) ->
    Self { Self { roast_id,bean_id,roast_ts,roast_level_id,roast_level_name: roast_level_name.into(),roast_level_description: roast_level_description.into(),} }
}pub struct AllRoastsQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> AllRoastsBorrowed,
    mapper: fn(AllRoastsBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> AllRoastsQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(AllRoastsBorrowed) -> R) ->
    AllRoastsQuery<'a,C,R,N>
    {
        AllRoastsQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params)
        .await?.map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct FindRoastById
{ pub roast_id : i32,pub bean_id : i32,pub roast_ts : time::OffsetDateTime,pub roast_level_id : i32,pub roast_level_name : String,pub roast_level_description : String,}pub struct FindRoastByIdBorrowed<'a> { pub roast_id : i32,pub bean_id : i32,pub roast_ts : time::OffsetDateTime,pub roast_level_id : i32,pub roast_level_name : &'a str,pub roast_level_description : &'a str,}
impl<'a> From<FindRoastByIdBorrowed<'a>> for FindRoastById
{
    fn from(FindRoastByIdBorrowed { roast_id,bean_id,roast_ts,roast_level_id,roast_level_name,roast_level_description,}: FindRoastByIdBorrowed<'a>) ->
    Self { Self { roast_id,bean_id,roast_ts,roast_level_id,roast_level_name: roast_level_name.into(),roast_level_description: roast_level_description.into(),} }
}pub struct FindRoastByIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> FindRoastByIdBorrowed,
    mapper: fn(FindRoastByIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> FindRoastByIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(FindRoastByIdBorrowed) -> R) ->
    FindRoastByIdQuery<'a,C,R,N>
    {
        FindRoastByIdQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params)
        .await?.map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct AllRoastSteps
{ pub roast_step_id : i32,pub roast_id : i32,pub step_order : i32,pub description : String,pub ts : time::OffsetDateTime,pub fan_speed : i32,pub temp_setting : i32,pub temperature : rust_decimal::Decimal,}pub struct AllRoastStepsBorrowed<'a> { pub roast_step_id : i32,pub roast_id : i32,pub step_order : i32,pub description : &'a str,pub ts : time::OffsetDateTime,pub fan_speed : i32,pub temp_setting : i32,pub temperature : rust_decimal::Decimal,}
impl<'a> From<AllRoastStepsBorrowed<'a>> for AllRoastSteps
{
    fn from(AllRoastStepsBorrowed { roast_step_id,roast_id,step_order,description,ts,fan_speed,temp_setting,temperature,}: AllRoastStepsBorrowed<'a>) ->
    Self { Self { roast_step_id,roast_id,step_order,description: description.into(),ts,fan_speed,temp_setting,temperature,} }
}pub struct AllRoastStepsQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> AllRoastStepsBorrowed,
    mapper: fn(AllRoastStepsBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> AllRoastStepsQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(AllRoastStepsBorrowed) -> R) ->
    AllRoastStepsQuery<'a,C,R,N>
    {
        AllRoastStepsQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params)
        .await?.map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct InsertRoast
{ pub roast_id : i32,pub bean_id : i32,pub roast_ts : time::OffsetDateTime,pub roast_level_id : i32,pub roast_level_name : String,pub roast_level_description : String,}pub struct InsertRoastBorrowed<'a> { pub roast_id : i32,pub bean_id : i32,pub roast_ts : time::OffsetDateTime,pub roast_level_id : i32,pub roast_level_name : &'a str,pub roast_level_description : &'a str,}
impl<'a> From<InsertRoastBorrowed<'a>> for InsertRoast
{
    fn from(InsertRoastBorrowed { roast_id,bean_id,roast_ts,roast_level_id,roast_level_name,roast_level_description,}: InsertRoastBorrowed<'a>) ->
    Self { Self { roast_id,bean_id,roast_ts,roast_level_id,roast_level_name: roast_level_name.into(),roast_level_description: roast_level_description.into(),} }
}pub struct InsertRoastQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> InsertRoastBorrowed,
    mapper: fn(InsertRoastBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> InsertRoastQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(InsertRoastBorrowed) -> R) ->
    InsertRoastQuery<'a,C,R,N>
    {
        InsertRoastQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params)
        .await?.map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct InsertRoastStep
{ pub roast_step_id : i32,pub roast_id : i32,pub step_order : i32,pub description : String,pub ts : time::OffsetDateTime,pub fan_speed : i32,pub temp_setting : i32,pub temperature : rust_decimal::Decimal,}pub struct InsertRoastStepBorrowed<'a> { pub roast_step_id : i32,pub roast_id : i32,pub step_order : i32,pub description : &'a str,pub ts : time::OffsetDateTime,pub fan_speed : i32,pub temp_setting : i32,pub temperature : rust_decimal::Decimal,}
impl<'a> From<InsertRoastStepBorrowed<'a>> for InsertRoastStep
{
    fn from(InsertRoastStepBorrowed { roast_step_id,roast_id,step_order,description,ts,fan_speed,temp_setting,temperature,}: InsertRoastStepBorrowed<'a>) ->
    Self { Self { roast_step_id,roast_id,step_order,description: description.into(),ts,fan_speed,temp_setting,temperature,} }
}pub struct InsertRoastStepQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> InsertRoastStepBorrowed,
    mapper: fn(InsertRoastStepBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> InsertRoastStepQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(InsertRoastStepBorrowed) -> R) ->
    InsertRoastStepQuery<'a,C,R,N>
    {
        InsertRoastStepQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params)
        .await?.map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn all_roasts() -> AllRoastsStmt
{ AllRoastsStmt(cornucopia_async::private::Stmt::new("select r.roast_id        as roast_id,
       r.bean_id         as bean_id,
       r.ts              as roast_ts,
       rl.roast_level_id as roast_level_id,
       rl.name           as roast_level_name,
       rl.description    as roast_level_description
from core.roast r
         inner join core.roast_level rl using (roast_level_id)")) } pub struct
AllRoastsStmt(cornucopia_async::private::Stmt); impl AllRoastsStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> AllRoastsQuery<'a,C,
AllRoasts, 0>
{
    AllRoastsQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { AllRoastsBorrowed { roast_id: row.get(0),bean_id: row.get(1),roast_ts: row.get(2),roast_level_id: row.get(3),roast_level_name: row.get(4),roast_level_description: row.get(5),} }, mapper: |it| { <AllRoasts>::from(it) },
    }
} }pub fn find_roast_by_id() -> FindRoastByIdStmt
{ FindRoastByIdStmt(cornucopia_async::private::Stmt::new("select r.roast_id        as roast_id,
       r.bean_id         as bean_id,
       r.ts              as roast_ts,
       rl.roast_level_id as roast_level_id,
       rl.name           as roast_level_name,
       rl.description    as roast_level_description
from core.roast r
         inner join core.roast_level rl using (roast_level_id)
where roast_id = $1")) } pub struct
FindRoastByIdStmt(cornucopia_async::private::Stmt); impl FindRoastByIdStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
roastId: &'a i32,) -> FindRoastByIdQuery<'a,C,
FindRoastById, 1>
{
    FindRoastByIdQuery
    {
        client, params: [roastId,], stmt: &mut self.0, extractor:
        |row| { FindRoastByIdBorrowed { roast_id: row.get(0),bean_id: row.get(1),roast_ts: row.get(2),roast_level_id: row.get(3),roast_level_name: row.get(4),roast_level_description: row.get(5),} }, mapper: |it| { <FindRoastById>::from(it) },
    }
} }pub fn all_roast_steps() -> AllRoastStepsStmt
{ AllRoastStepsStmt(cornucopia_async::private::Stmt::new("select roast_step_id,
       roast_id,
       step_order,
       description,
       ts,
       fan_speed,
       temp_setting,
       temperature
from core.roast_step
where roast_id = $1")) } pub struct
AllRoastStepsStmt(cornucopia_async::private::Stmt); impl AllRoastStepsStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
roastId: &'a i32,) -> AllRoastStepsQuery<'a,C,
AllRoastSteps, 1>
{
    AllRoastStepsQuery
    {
        client, params: [roastId,], stmt: &mut self.0, extractor:
        |row| { AllRoastStepsBorrowed { roast_step_id: row.get(0),roast_id: row.get(1),step_order: row.get(2),description: row.get(3),ts: row.get(4),fan_speed: row.get(5),temp_setting: row.get(6),temperature: row.get(7),} }, mapper: |it| { <AllRoastSteps>::from(it) },
    }
} }pub fn insert_roast() -> InsertRoastStmt
{ InsertRoastStmt(cornucopia_async::private::Stmt::new("with r as (
    insert into core.roast (bean_id, roast_level_id, ts)
        values ($1, $2, $3)
        returning roast_id, bean_id, roast_level_id, ts)
select r.roast_id        as roast_id,
       r.bean_id         as bean_id,
       r.ts              as roast_ts,
       rl.roast_level_id as roast_level_id,
       rl.name           as roast_level_name,
       rl.description    as roast_level_description
from r
         inner join core.roast_level rl using (roast_level_id)")) } pub struct
InsertRoastStmt(cornucopia_async::private::Stmt); impl InsertRoastStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
beanId: &'a i32,roastLevelId: &'a i32,ts: &'a time::OffsetDateTime,) -> InsertRoastQuery<'a,C,
InsertRoast, 3>
{
    InsertRoastQuery
    {
        client, params: [beanId,roastLevelId,ts,], stmt: &mut self.0, extractor:
        |row| { InsertRoastBorrowed { roast_id: row.get(0),bean_id: row.get(1),roast_ts: row.get(2),roast_level_id: row.get(3),roast_level_name: row.get(4),roast_level_description: row.get(5),} }, mapper: |it| { <InsertRoast>::from(it) },
    }
} }impl <'a, C: GenericClient,> cornucopia_async::Params<'a,
InsertRoastParams<>, InsertRoastQuery<'a, C,
InsertRoast, 3>, C> for InsertRoastStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertRoastParams<>) -> InsertRoastQuery<'a, C,
    InsertRoast, 3>
    { self.bind(client, &params.beanId,&params.roastLevelId,&params.ts,) }
}pub fn insert_roast_step() -> InsertRoastStepStmt
{ InsertRoastStepStmt(cornucopia_async::private::Stmt::new("insert into core.roast_step(roast_id, step_order, description, ts, fan_speed, temp_setting, temperature)
values ($1, $2, $3, $4, $5, $6, $7)
returning
    roast_step_id,
    roast_id,
    step_order,
    description,
    ts,
    fan_speed,
    temp_setting,
    temperature")) } pub struct
InsertRoastStepStmt(cornucopia_async::private::Stmt); impl InsertRoastStepStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
roastId: &'a i32,stepOrder: &'a i32,description: &'a T1,ts: &'a time::OffsetDateTime,fanSpeed: &'a i32,tempSetting: &'a i32,temperature: &'a rust_decimal::Decimal,) -> InsertRoastStepQuery<'a,C,
InsertRoastStep, 7>
{
    InsertRoastStepQuery
    {
        client, params: [roastId,stepOrder,description,ts,fanSpeed,tempSetting,temperature,], stmt: &mut self.0, extractor:
        |row| { InsertRoastStepBorrowed { roast_step_id: row.get(0),roast_id: row.get(1),step_order: row.get(2),description: row.get(3),ts: row.get(4),fan_speed: row.get(5),temp_setting: row.get(6),temperature: row.get(7),} }, mapper: |it| { <InsertRoastStep>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
InsertRoastStepParams<T1,>, InsertRoastStepQuery<'a, C,
InsertRoastStep, 7>, C> for InsertRoastStepStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertRoastStepParams<T1,>) -> InsertRoastStepQuery<'a, C,
    InsertRoastStep, 7>
    { self.bind(client, &params.roastId,&params.stepOrder,&params.description,&params.ts,&params.fanSpeed,&params.tempSetting,&params.temperature,) }
}}}